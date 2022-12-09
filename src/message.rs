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
