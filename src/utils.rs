use serde_json::Value;

use crate::message::Message;


pub fn messages_to_vec(val_messages: &Value) -> Vec<Message> {
    let mut messages: Vec<Message> = Vec::new();
    for message in  val_messages.as_array().unwrap() {
        messages.push(Message::new(text_value_to_string(&message["text"])));
    }
    return messages;
}

pub fn text_value_to_string(text_val: &Value) -> String {
    return unwrap_option_or_empty_str(text_val.as_str()).trim_end().trim_start().to_string();
}

pub fn unwrap_option_or_empty_str(str_option: Option<&str>) -> &str {
    return match str_option {
            Some(unwrapped_str) => unwrapped_str,
            None => "Error",
    }
}
