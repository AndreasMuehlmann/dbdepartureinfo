use druid::{Data, Lens};


#[derive(Clone, Data, Lens)]
pub struct Message {
    pub message: String,
}


impl Message {
    pub fn new(message: String) -> Self {
        return Self {
            message,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_message() {
        Message::new("This is a message.".to_string());
    }
}
