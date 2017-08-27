use super::ConnectionMap;
use super::MsgVal;
use uuid::Uuid;

pub trait AuthorizesTicket<T> {
    fn authorize_ticket(&self, ticket: T) -> Result<(), String>;
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

impl AuthorizesTicket<Uuid> for DumbTicketStamper {
    fn authorize_ticket(&self, ticket: Uuid) -> Result<(), String> {
        println!("authorizing ticket {:?}", ticket);
        if self.conn_map.borrow().contains_key(&ticket) {
            Ok(())
        } else {
            Err(String::from("authorize_ticket_dumb: authorization failed"))
        }
    }
}
