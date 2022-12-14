use serde_json::{Value, from_str};
use reqwest::Error;
use reqwest::blocking::Response;

use crate::departure::Departure;
use crate::utils::{messages_to_vec, text_value_to_string};


pub struct DBFAPI {
    departure_limit: u32, 
}


impl DBFAPI {
    pub fn new(departure_limit: u32) -> Self {
        return Self {
            departure_limit,
        } 
    }

    pub fn get_departures(&self, name: String) -> Vec<Departure> {
        let result_response: Result<Response, Error> = self.get_api_response(&name);
        let response = match result_response {
            Ok(response) => response,
            Err(error) => {
                println!("Error: {}", error);
                return Vec::new();
            },
        };
        let txt_response: &str = &response.text().unwrap();
        let parsed_json: Value = from_str(txt_response).unwrap();
        let departures = self.to_vector_struct_departures(name, parsed_json);
        return departures;
    }

    fn get_api_response(&self, name: &String) -> Result<Response, Error> {
        let request_url = format!("https://dbf.finalrewind.org/{}.json?version=3&limit={}",
                                  name, self.departure_limit);
        let response = reqwest::blocking::get(&request_url)?;
        return Ok(response);
    }

    fn to_vector_struct_departures(&self, name: String, parsed_json: Value) -> Vec<Departure>{
        let departures_value_option = parsed_json["departures"].as_array();
        if departures_value_option.is_none() {
            Self::display_error_message(name, parsed_json);
            return Vec::new();
        }
        let mut departures = Vec::new();
        for departure_value in departures_value_option.unwrap() {
            let mut qos_messages = messages_to_vec(&departure_value["messages"]["qos"]);
            let delay_messages = messages_to_vec(&departure_value["messages"]["delay"]);
            qos_messages.extend(delay_messages);
            let mut delay_departure = departure_value["delayDeparture"].to_string();
            if delay_departure == "null".to_string() {
                delay_departure = "".to_string();
            } else {
                delay_departure = format!("+{}", delay_departure);
            }
            let departure = Departure::new(
                text_value_to_string(&departure_value["scheduledDeparture"]),
                delay_departure,
                text_value_to_string(&departure_value["destination"]),
                text_value_to_string(&departure_value["scheduledPlatform"]),
                text_value_to_string(&departure_value["train"]),
                qos_messages, 
                );
            departures.push(departure);
        }
        return departures;
    }

    fn display_error_message(name: String, parsed_json: Value) {
        let error_message = text_value_to_string(&parsed_json["error"]);
        println!("Error in api response for api_name {}: {}.", name, error_message);
        println!("Note: Maybe the given api_name for a station doesn't exist.");
    }
}


#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::dbf_api::DBFAPI;


    fn create_dbf_api() -> DBFAPI {
        return DBFAPI::new(1);
    }

    #[test]
    fn test_to_vector_struct_departures() {
        let dbf_api = create_dbf_api();
        let txt_response = "{\"departures\":[{\"delayArrival\":28,\"delayDeparture\":28,\"destination\":\"Erding\",\"isCancelled\":0,\"messages\":{\"delay\":[{\"text\":\"Defekt an der Strecke\",\"timestamp\":\"2022-11-25T17:59:00\"},{\"text\":\"Versp??tung eines vorausfahrenden Zuges\",\"timestamp\":\"2022-11-25T17:48:00\"}],\"qos\":[]},\"missingRealtime\":false,\"platform\":\"1\",\"route\":[{\"name\":\"Petershausen(Obb)\"},{\"name\":\"Vierkirchen-Esterhofen\"}],\"scheduledArrival\":\"18:14\",\"scheduledDeparture\":\"18:15\",\"scheduledPlatform\":\"1\",\"train\":\"S 2\",\"trainClasses\":[\"S\"],\"trainNumber\":\"6291\",\"via\":[\"Grub(Oberbay)\",\"Poing\",\"Markt Schwaben\"]}]}";

        let parsed_json: Value = serde_json::from_str(txt_response).unwrap();
        let departure = &dbf_api.to_vector_struct_departures("MVS".to_string(), parsed_json)[0];
        assert_eq!(departure.scheduled_departure, "18:15");
        assert_eq!(departure.delay_departure, "28");
        assert_eq!(departure.destination, "Erding");
        assert_eq!(departure.scheduled_platform, "1");
        assert_eq!(departure.train, "S 2");
    }
}
