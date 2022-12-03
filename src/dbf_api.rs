use serde_json::Value;
use reqwest::Error;
use reqwest::Response;

use super::departure::Departure;


pub struct DBFAPI {
    departure_limit: u32, 
}


impl DBFAPI {
    pub fn new(departure_limit: u32) -> Self {
        return Self {
            departure_limit,
        } 
    }


    pub async fn get_departures(&self, name: String) -> Vec<Departure> {
        let response: Response = self.get_api_response(name).await.unwrap();
        let txt_response: &str = &response.text().await.unwrap();
        let parsed_json: Value = serde_json::from_str(txt_response).unwrap();
        let departures = self.to_vector_struct_departures(parsed_json);
        return departures;
    }

    async fn get_api_response(&self, name: String) -> Result<Response, Error> {
        let request_url = format!("https://dbf.finalrewind.org/{}.json?version=3&limit={}",
                                  name,
                                  self.departure_limit);
        let response = reqwest::get(&request_url).await?;
        return Ok(response);
    }

    fn to_vector_struct_departures(&self, parsed_json: Value) -> Vec<Departure>{
        let departures_json = parsed_json["departures"].as_array().unwrap();
        let mut departures = Vec::new();
        for departure_json in departures_json {
            let departure = Departure::new(
                Self::unwrap_option_or_empty_str(departure_json["scheduledDeparture"].as_str()).trim_end().trim_start().to_string(),
                departure_json["delayDeparture"].to_string(),
                Self::unwrap_option_or_empty_str(departure_json["destination"].as_str()).trim_end().trim_start().to_string(),
                Self::unwrap_option_or_empty_str(departure_json["scheduledPlatform"].as_str()).trim_end().trim_start().to_string(),
                Self::unwrap_option_or_empty_str(departure_json["train"].as_str()).trim_end().trim_start().to_string(),
                );
            departures.push(departure);
        }
        return departures;
    }

    fn unwrap_option_or_empty_str(str_option: Option<&str>) -> &str {
        return match str_option {
                Some(unwrapped_str) => unwrapped_str,
                None => "None",
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
