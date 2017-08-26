use super::ConnectionMap;
use super::MsgVal;

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

impl AuthorizesTicket<MsgVal> for DumbTicketStamper {
    fn authorize_ticket(&self, ticket_val: MsgVal) -> Result<(), String> {
        println!("authorizing ticket {:?}", ticket_val);
        if let MsgVal::Uuid(ticket) = ticket_val {
            if self.conn_map.borrow().contains_key(&ticket) {
                Ok(())
            } else {
                Err(String::from("authorize_ticket_dumb: authorization failed"))
            }
        } else {
            Err(String::from("authorize_ticket_dumb: ticket is not MsgVal::Uuid"))
        }
    }
}
