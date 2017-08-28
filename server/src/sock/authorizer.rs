use ws;
use uuid::Uuid;
use super::ConnectionMap;

pub trait AuthorizesTicket<T> {
    fn authorize_ticket(&self, token: ws::util::Token, ticket: T) -> Result<(), String>;
}

pub struct DumbTicketStamper {
    conn_map: ConnectionMap
}

impl DumbTicketStamper {
    pub fn new(conn_map: ConnectionMap) -> DumbTicketStamper {
        DumbTicketStamper {
            conn_map: conn_map
        }
    }
}

impl AuthorizesTicket<Uuid> for DumbTicketStamper {
    fn authorize_ticket(&self, token: ws::util::Token, ticket: Uuid) -> Result<(), String> {
        println!("authorizing ticket {:?}", ticket);
        let conn_map = self.conn_map.borrow();
        conn_map.get(&token)
            .ok_or(String::from("authorization failed: no connection for token"))
            .and_then(|conn| {
                if conn.ticket == ticket {
                    println!("authorization success");
                    Ok(())
                } else {
                    Err(String::from("authorization failed: ticket mismatch"))
                }
            })
    }
}
