use druid::{Data, Lens};
use druid::im::Vector;

use crate::message::Message;


#[derive(Clone, Data, Lens)]
pub struct Departure {
    pub scheduled_departure: String,
    pub delay_departure: String,
    pub destination: String,
    pub scheduled_platform: String,
    pub train: String,
    pub messages: Vector<Message>,
}


impl Departure {
    pub fn new(scheduled_departure: String, delay_departure: String, destination: String,
               scheduled_platform: String, train: String, messages: Vec<Message>) -> Self {
        let messages_im_vec = Vector::from(messages);
        return Self {
            scheduled_departure,
            delay_departure,
            destination,
            scheduled_platform,
            train,
            messages: messages_im_vec,
        }
    }
}
