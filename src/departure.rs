use druid::{Data, Lens};


#[derive(Clone, Data, Lens)]
pub struct Departure {
    pub scheduled_departure: String,
    pub delay_departure: String,
    pub destination: String,
    pub scheduled_platform: String,
    pub train: String,
}


impl Departure {
    pub fn new(scheduled_departure: String, delay_departure: String,
               destination: String, scheduled_platform: String, train: String) -> Self {
        return Self {
            scheduled_departure,
            delay_departure,
            destination,
            scheduled_platform,
            train,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_depature() {
        Departure::new("3:00".to_string(), "3.3".to_string(), "Grafing".to_string(), "1".to_string(), "S 6".to_string());
    }
}
