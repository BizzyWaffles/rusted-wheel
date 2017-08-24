use ws;
use uuid::Uuid;
use lib::ParseFrom;
use super::{Item,Action};

pub fn parse_message_ticket (msg: ws::Message) -> Result<(Uuid, Vec<String>), String> {
    fn parse_error_msg (reason: &str) -> String {
        format!("parse_message_ticket failure: {}", reason)
    }

    msg.into_text()
        .or(Err(parse_error_msg("cannot get text from Message; is it a binary Message?")))
        ?.split(":")                         // String -> Split<'_, &str>
        .map(|p| p.to_string())              // Split<'_, &str> -> &[String]
        .collect::<Vec<String>>()            // &[String] -> Vec<String>
        .split_first()                       // Vec<String> -> Option<(String, Vec<String>)>
        .ok_or(parse_error_msg("no ticket")) // Option -> Result<(String, &[String]), String>
        .and_then(|(uuid_string, rest)| {    // Result<(String, &[String]), String>
            Uuid::parse_str(uuid_string)     //   |-> Result<(Uuid, Vec<String>), String>
                .or(Err(parse_error_msg("ticket is invalid uuidv4")))
                .map(|uuid| (uuid, Vec::from(rest)))
        })
}

pub fn parse_message_type(msg_contents: Vec<String>) -> Result<(String, Vec<String>), String> {
    msg_contents.split_first()
        .ok_or(String::from("missing message type and params"))
        .and_then(|(f, r)| {
            if f.len() == 0 {
                Err(String::from("msg_type is empty"))
            } else {
                Ok((f.to_owned(), Vec::from(r)))
            }
        })
}

pub fn parse_message_action((msg_type, msg_params): (String, Vec<String>)) -> Result<Action, String> {
    println!("trying to read contents of message {}({})", msg_type, msg_params.join(","));

    // TODO(jordan): replace with Action ParseFrom<String> impl
    match msg_type.as_str() {
        "addItemToInventory" => {
            if msg_params.len() != 1 {
                Err(String::from("parse_message failure: addItemToInventory: no item code"))
            } else if let Ok(item_num) = msg_params[0].parse::<i32>() {
                Item::parse(item_num)
                    .map(|item| Action::addItemToInventory(item))
            } else {
                Err(String::from("parse_message failure: addItemToInventory: invalid i32 item code"))
            }
        },
        _ => Err(String::from("parse_message failure: unrecognized message type"))
    }
}
