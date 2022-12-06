use serde_json::Value;
use reqwest::Error;
use reqwest::blocking::Response;

use crate::departure::Departure;
use crate::message::Message;


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
        let parsed_json: Value = serde_json::from_str(txt_response).unwrap();
        let departures = self.to_vector_struct_departures(parsed_json);
        return departures;
    }

    fn get_api_response(&self, name: &String) -> Result<Response, Error> {
        let request_url = format!("https://dbf.finalrewind.org/{}.json?version=3&limit={}",
                                  name, self.departure_limit);
        let response = reqwest::blocking::get(&request_url)?;
        return Ok(response);
    }

    fn to_vector_struct_departures(&self, parsed_json: Value) -> Vec<Departure>{
        let departures_json = parsed_json["departures"].as_array().unwrap();
        let mut departures = Vec::new();
        for departure_json in departures_json {
            let mut qos_messages = Self::messages_to_vec(&departure_json["messages"]["qos"]);
            let delay_messages = Self::messages_to_vec(&departure_json["messages"]["delay"]);
            qos_messages.extend(delay_messages);
            let departure = Departure::new(
                Self::text_value_to_string(&departure_json["scheduledDeparture"]),
                departure_json["delayDeparture"].to_string(),
                Self::text_value_to_string(&departure_json["destination"]),
                Self::text_value_to_string(&departure_json["scheduledPlatform"]),
                Self::text_value_to_string(&departure_json["train"]),
                qos_messages, 
                );
            departures.push(departure);
        }
        return departures;
    }

    fn messages_to_vec(val_messages: &Value) -> Vec<Message> {
        let mut messages: Vec<Message> = Vec::new();
        for message in  val_messages.as_array().unwrap() {
            messages.push(Message::new(Self::text_value_to_string(&message["text"])));
        }
        return messages;
    }

    fn text_value_to_string(text_val: &Value) -> String {
        return Self::unwrap_option_or_empty_str(text_val.as_str()).trim_end().trim_start().to_string();
    }
    
    fn unwrap_option_or_empty_str(str_option: Option<&str>) -> &str {
        return match str_option {
                Some(unwrapped_str) => unwrapped_str,
                None => "Error",
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn create_dbf_api() -> DBFAPI {
        return DBFAPI::new(1);
    }

    #[test]
    fn test_to_vector_struct_departures() {
        let dbf_api = create_dbf_api();
        let txt_response = "{\"departures\":[{\"delayArrival\":28,\"delayDeparture\":28,\"destination\":\"Erding\",\"isCancelled\":0,\"messages\":{\"delay\":[{\"text\":\"Defekt an der Strecke\",\"timestamp\":\"2022-11-25T17:59:00\"},{\"text\":\"Verspätung eines vorausfahrenden Zuges\",\"timestamp\":\"2022-11-25T17:48:00\"}],\"qos\":[]},\"missingRealtime\":false,\"platform\":\"1\",\"route\":[{\"name\":\"Petershausen(Obb)\"},{\"name\":\"Vierkirchen-Esterhofen\"}],\"scheduledArrival\":\"18:14\",\"scheduledDeparture\":\"18:15\",\"scheduledPlatform\":\"1\",\"train\":\"S 2\",\"trainClasses\":[\"S\"],\"trainNumber\":\"6291\",\"via\":[\"Grub(Oberbay)\",\"Poing\",\"Markt Schwaben\"]}]}";

        let parsed_json: Value = serde_json::from_str(txt_response).unwrap();
        let departure = &dbf_api.to_vector_struct_departures(parsed_json)[0];
        assert_eq!(departure.scheduled_departure, "18:15");
        assert_eq!(departure.delay_departure, "28");
        assert_eq!(departure.destination, "Erding");
        assert_eq!(departure.scheduled_platform, "1");
        assert_eq!(departure.train, "S 2");
    }
}
