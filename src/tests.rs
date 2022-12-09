use crate::message::Message;
use crate::departure::Departure;
use crate::station::Station;
use crate::state::State;


fn create_message() -> Message {
    return Message::new("This is a message.".to_string());
}

fn create_depature() -> Departure {
    return Departure::new("3:00".to_string(), "3.3".to_string(), "Grafing".to_string(),
    "1".to_string(), "S 6".to_string(), vec![create_message()]);
}

fn create_station() -> Station {
    return Station::new("Vaterstetten".to_string(), "MVS".to_string(), vec![create_depature()]);
}

fn create_state() -> State {
    return State::new(vec![create_station()]);
}


#[test]
fn test_create_message() {
    create_message();
}

#[test]
fn test_create_depature() {
    create_depature();
}

#[test]
fn test_create_station() {
    create_station();
}

#[test]
fn test_create_state() {
    create_state();
}
