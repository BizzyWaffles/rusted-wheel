use uuid::Uuid;
use ws::util::Token;
use super::Connection;
use super::ConnectionMap;

pub trait AuthorizesTicket<T> {
    fn authorize_ticket(&self, token: Token, ticket: T) -> Result<Connection, String>;
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
    fn authorize_ticket(&self, token: Token, ticket: Uuid) -> Result<Connection, String> {
        println!("authorizing ticket {:?}", ticket);
        self.connections.borrow()
            .get(&token)
            .ok_or(String::from("authorization failed: no connection for token"))
            .and_then(|conn| {
                if conn.ticket == ticket {
                    println!("authorization success");
                    Ok(conn.clone())
                } else {
                    Err(String::from("authorization failed: ticket mismatch"))
                }
            })
    }
}
