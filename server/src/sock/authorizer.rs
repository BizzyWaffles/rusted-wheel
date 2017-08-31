use uuid::Uuid;
use std::rc::Rc;
use ws::util::Token;
use super::Connection;
use std::cell::RefCell;
use super::ConnectionMap;

pub trait AuthorizesTicket<T> {
    fn authorize_ticket(&self, token: Token, ticket: T) -> Result<Rc<RefCell<Connection>>, String>;
}

pub struct DumbTicketStamper {
    connections: ConnectionMap
}

impl DumbTicketStamper {
    pub fn new(conn_map: ConnectionMap) -> DumbTicketStamper {
        DumbTicketStamper {
            connections: conn_map
        }
    }
}

impl AuthorizesTicket<Uuid> for DumbTicketStamper {
    fn authorize_ticket(&self, token: Token, ticket: Uuid) -> Result<Rc<RefCell<Connection>>, String> {
        println!("authorizing ticket {:?}", ticket);
        self.connections
            .borrow()
            .get(&token)
            .ok_or(String::from("authorization failed: no connection for token"))
            .and_then(|conn| {
                if conn.borrow().ticket == ticket {
                    println!("authorization success");
                    Ok(conn.clone())
                } else {
                    Err(String::from("authorization failed: ticket mismatch"))
                }
            })
    }
}
