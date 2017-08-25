use ws;
use uuid::Uuid;
use std::rc::Rc;
use lib::ParseFrom;
use super::{Item,Action};

#[derive(Debug)]
pub enum MsgVal {
    WsMessage(ws::Message),
    Uuid(Uuid),
    Item(Item),
    Action(Action),
    String(String),
}

impl Into<MsgVal> for Uuid {
    fn into(self) -> MsgVal {
        MsgVal::Uuid(self)
    }
}
impl Into<MsgVal> for Item {
    fn into(self) -> MsgVal {
        MsgVal::Item(self)
    }
}
impl Into<MsgVal> for Action {
    fn into(self) -> MsgVal {
        MsgVal::Action(self)
    }
}
impl Into<MsgVal> for String {
    fn into(self) -> MsgVal {
        MsgVal::String(self)
    }
}

#[derive(Debug)]
pub struct MsgCell {
    val:  MsgVal,
    next: Option<Rc<MsgCell>>,
}

#[derive(Debug)]
struct PartialMsg<T> {
    parsed:  Option<Rc<MsgCell>>,
    pending: Option<T>,
}

impl PartialMsg <ws::Message> {
    fn new (msg: ws::Message) -> Self {
        PartialMsg {
            parsed:  None,
            pending: Some(msg),
        }
    }
}

// TODO(jordan): investigate whether clone is necessary
impl <T: Clone> PartialMsg <T> {
    fn parse_next <F, Tnew, Tresult: Into<MsgVal>> (&self, parser: F) -> Result<PartialMsg<Tnew>, String>
        where F: Fn(T) -> Result<(Tresult, Option<Tnew>), String> {
        self.pending
            .clone()
            .ok_or(String::from("No pending to parse"))
            .and_then(parser)
            .map(|(newly_parsed, still_pending)| {
                PartialMsg {
                    parsed: Some(Rc::new(MsgCell {
                        next: self.parsed.clone(),
                        val: newly_parsed.into(),
                    })),
                    pending: still_pending,
                }
            })
    }
}

fn parse_ticket (msg: ws::Message) -> Result<(Uuid, Option<String>), String> {
    fn parse_error_msg (reason: &str) -> String {
        format!("parse_ticket failure: {}", reason)
    }

    msg.into_text()
        .or(Err(parse_error_msg("cannot get text from Message; is it a binary Message?")))
        ?.splitn(2, ':')
        .map(|p| p.to_string())
        .collect::<Vec<String>>()
        .split_first()
        .ok_or(parse_error_msg("no ticket"))
        .map(|(ticket_string, msg_remainder)| {
            Uuid::parse_str(ticket_string)
                .or(Err(parse_error_msg("ticket is invalid uuidv4")))
                .map(|uuid| (uuid, Vec::from(msg_remainder).pop()))
        })?
}

fn parse_action (mut action_string: String) -> Result<(Action, Option<String>), String> {
    let left_paren: usize = action_string
        .find("(")
        .unwrap_or(action_string.len());

    let action_type: String = action_string.drain(..left_paren).collect();

    let right_paren: usize = action_string
        .find(")")
        .unwrap_or(action_string.len());

    let mut action_parameters: Vec<String> = action_string
        .drain(1..right_paren)
        .collect::<String>()
        .split(",")
        .map(|s| s.to_string())
        .collect();

    let remainder: Option<String> = if action_string.len() > 0 {
        Some(action_string)
    } else {
        None
    };

    if action_type.len() == 0 {
        Err(String::from("missing action type"))
    } else if action_parameters.len() == 0 {
        Err(String::from("missing action params"))
    } else {
        match action_type.as_str() {
            "addItemToInventory" => {
                action_parameters.pop()
                    .ok_or(String::from("addItemToInventory: no item"))
                    .and_then(|item_str| {
                        item_str.parse::<i32>().map_err(|_err| String::from("invalid i32"))
                    })
                    .and_then(|item_code| Item::parse(item_code))
                    .map(|item| (Action::addItemToInventory(item), remainder))
            },
            _ => Err(String::from(format!("unrecognized action {}", action_type)))
        }
    }
}

pub fn parse (msg: ws::Message) -> Result<Rc<MsgCell>, String> {
    PartialMsg::new(msg)
        .parse_next(parse_ticket)
        ?.parse_next(parse_action)
        ?.parsed.ok_or(String::from(""))
}

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
