use ws;
use uuid::Uuid;
use std::rc::Rc;
use lib::ParseFrom;
use super::{Item,Action};

#[derive(Debug,Clone)]
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
pub struct MsgCell<T,U> {
    pub val:  T,
    pub next: U,
}

#[derive(Debug)]
pub struct UnitMsgCell<T> {
    pub val: T,
}

#[derive(Debug)]
struct PendingMsg<T> {
    pending: T,
}

#[derive(Debug)]
struct PartialMsg<T,P> {
    parsed:  P,
    pending: Option<T>,
}

#[derive(Debug)]
struct ParsedMsg<P> {
    parsed: P,
}

impl PendingMsg <ws::Message> {
    fn new (msg: ws::Message) -> Self {
        PendingMsg {
            pending: msg,
        }
    }
}

// TODO(jordan): investigate whether clone is necessary
impl <T: Clone> PendingMsg <T> {
    fn parse_next <F,Tnew,V> (&self, parser: F)
        -> Result<PartialMsg<Tnew,Rc<UnitMsgCell<V>>>, String>
        where F: Fn(Option<T>) -> Result<(V, Option<Tnew>), String> {
        parser(Some(self.pending.clone()))
            .map(|(newly_parsed, still_pending)| {
                PartialMsg {
                    parsed: Rc::new(UnitMsgCell {
                        val: newly_parsed,
                    }),
                    pending: still_pending,
                }
            })
    }
}

impl <T: Clone,P> PartialMsg <T,Rc<P>> {
    fn parse_next <F,Tnew,V> (&self, parser: F)
        -> Result<PartialMsg<Tnew,Rc<MsgCell<V,Rc<P>>>>, String>
        where F: Fn(Option<T>) -> Result<(V, Option<Tnew>), String> {
        parser(self.pending.clone())
            .map(|(newly_parsed, still_pending)| {
                PartialMsg {
                    parsed: Rc::new(MsgCell {
                        next: self.parsed.clone(),
                        val:  newly_parsed,
                    }),
                    pending: still_pending,
                }
            })
    }
}

fn parse_ticket (msg: Option<ws::Message>) -> Result<(Uuid, Option<String>), String> {
    fn parse_error_msg (reason: &str) -> String {
        format!("parse_ticket failure: {}", reason)
    }

    msg.ok_or(parse_error_msg("there is no message"))
        ?.into_text()
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

fn parse_action (action_string: Option<String>) -> Result<(Action, Option<String>), String> {
    let mut action_string: String = action_string
        .ok_or(String::from("parse_action failure: got None for action_string"))?;

    let left_paren: usize = action_string
        .find("(")
        .ok_or(String::from("parse_action failure: could not find \"(\""))?;

    let action_type: String = action_string.drain(..left_paren).collect();

    let right_paren: usize = action_string
        .find(")")
        .ok_or(String::from("parse_action failure: could not find \")\""))?;

    let mut action_parameters: Vec<String> = action_string
        .drain(1..right_paren)
        .collect::<String>()
        .split(",")
        .map(|s| s.to_string())
        .filter(|s| s.len() > 0)
        .collect();

    let remainder: Option<String> = if action_string.len() > 0 {
        Some(action_string)
    } else {
        None
    };

    if action_type.len() == 0 {
        Err(String::from("missing action type"))
    } else {
        match action_type.as_str() {
            "addItemToInventory" => {
                action_parameters.pop()
                    .ok_or(String::from("addItemToInventory: no item"))
                    .and_then(|item_str| {
                        item_str.parse::<i32>()
                            .map_err(|_err| String::from("invalid i32"))
                    })
                    .and_then(|item_code| Item::parse(item_code))
                    .map(|item| (Action::addItemToInventory(item), remainder))
            },
            _ => Err(String::from(format!("unrecognized action {}", action_type)))
        }
    }
}

pub type ActionMsg = Rc<MsgCell<Action,Rc<UnitMsgCell<Uuid>>>>;

pub fn parse (msg: ws::Message) -> Result<ActionMsg, String> {
    PendingMsg::new(msg)
        .parse_next(parse_ticket)
        ?.parse_next(parse_action)
        .map(|partial_msg| partial_msg.parsed)
}
