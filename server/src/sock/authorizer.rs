use uuid::Uuid;
use super::ConnectionMap;

pub trait AuthorizesTicket<T, U> {
    fn authorize_ticket(&self, msg_ticket_and_contents_tuple: (T, U)) -> Result<U, String>;
}

pub struct DumbTicketStamper {
    conn_map: ConnectionMap
}

impl DumbTicketStamper {
    pub fn new(conns: ConnectionMap) -> DumbTicketStamper {
        DumbTicketStamper {
            conn_map: conns
        }
    }
}

impl AuthorizesTicket<Uuid, Vec<String>> for DumbTicketStamper {
    fn authorize_ticket(&self, (msg_ticket, rest): (Uuid, Vec<String>)) -> Result<Vec<String>, String> {
        println!("authorizing ticket {}", msg_ticket);
        if self.conn_map.borrow().contains_key(&msg_ticket) {
            Ok(rest)
        } else {
            Err(String::from("authorize_ticket_dumb authorization failed"))
        }
    }
}
