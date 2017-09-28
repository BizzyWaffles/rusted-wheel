//! sock
//!
//! Sock is a websocket server.
//! Sister server to `web`, which is the static file server.
use ws;
use time;
use std::fmt;
use uuid::Uuid;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap};

mod msg;
mod cookie;
mod authorizer;

use self::cookie::{parse_cookies,put_cookie};
use game::{Action,Player,AnonymousPlayer};
use self::authorizer::{AuthorizesTicket,DumbTicketStamper};
use self::msg::{ActionMsg,parse};

#[derive(Debug, Clone)]
pub struct Connection {
    token  : ws::util::Token,
    ticket : Uuid,
    player : Player,
}

impl Connection {
    fn new(tk: ws::util::Token, tx: Uuid, p: Player) -> Connection {
        Connection {
            token : tk,
            ticket: tx,
            player: p,
        }
    }
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "conn[{:?}]{{{}}}", self.token, self.ticket)
    }
}

fn ws_log (evt: &str, token: ws::util::Token, msg: &str) {
    println!("ws:{}[{}][{:?}]: {}", evt, time::precise_time_ns(), token, msg)
}

fn ws_conn_log (evt: &str, con: &Connection, msg: &str) {
    println!("ws:{}[{}]:[{}]: {}", evt, time::precise_time_ns(), con, msg)
}

pub type ConnectionMap = Rc<RefCell<HashMap<ws::util::Token, Rc<RefCell<Connection>>>>>;

struct WSServer<T> {
    out: ws::Sender,
    connections: ConnectionMap,
    authorizer: T,
}

impl ws::Handler for WSServer<DumbTicketStamper> {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<ws::Response> {
        let token = self.out.token();
        ws_log("req", token, "on_request received");

        let cookies: HashMap<String, String> = parse_cookies(req);
        let ticket: Option<Uuid> = cookies
            .get("bzwf_anon_wstx")
            .and_then(|uuid_string| Uuid::parse_str(uuid_string.as_str()).ok());

        let mut resp = ws::Response::from_request(req).unwrap();

        if let Some(ticket) = ticket {
            // If you have a ticket, you may have authenticated already.
            // Your connection was already established and has been upgraded.
            let conn_map = self.connections.borrow();
            let conn = conn_map.get(&token).unwrap().borrow();
            // TODO(jordan): ???
            let _ = self.authorizer.authorize_ticket(token, ticket)
                .map(|_| {
                    ws_conn_log("req", &conn, "ticket matches connection ticket");
                })
                .map_err(|err| {
                    ws_conn_log("req", &conn, &format!("err {}", err));
                });
        } else {
            // QUESTION(jordan): does lack of a ticket always mean anonymous player?
            // You do not have a ticket. Create a new Connection and authenticate anonymously.
            let ticket = Uuid::new_v4();
            let new_conn = Connection::new(token, ticket, AnonymousPlayer::new());
            ws_conn_log("req", &new_conn, "new connection");
            ws_conn_log("req", &new_conn, &format!("put cookie bzwf_anon_wstx={}", ticket));
            put_cookie(String::from("bzwf_anon_wstx"), ticket.to_string(), &mut resp);
            self.connections.borrow_mut().insert(token, Rc::new(RefCell::new(new_conn)));
        }

        ws_log("req", token, &format!("{} connected users", self.connections.borrow().len()));

        Ok(resp)
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let token = self.out.token();

        let _ = parse(msg)
            .and_then(|msg_cell: ActionMsg| {
                let action: Action = msg_cell.val.clone();
                let ticket: Uuid = msg_cell.next.val;

                self.authorizer.authorize_ticket(token, ticket)
                    .map(|conn| (conn, action))
            })
            .map(|(conn, action)| {
                let Action::addItemToInventory(item) = action.clone();
                conn.borrow_mut().player.state().inventory.insert(item);
                ws_conn_log("rcv", &conn.borrow(), &format!("{:?}", action));
                println!("{:?}", conn.borrow_mut().player.state());
                let _ = self.out.send(format!("{:?}", conn.borrow_mut().player.state()));
            })
            .map_err(|err: String| {
                ws_log("rcv", token, &format!("err {}", err));
                let _ = self.out.send(err);
            });

        Ok(())
    }

    fn on_open(&mut self, hs: ws::Handshake) -> ws::Result<()> {
        // NOTE(jordan): There should be a Connection in the map, and a cookie matching the ticket.
        let token   = self.out.token();
        let cookies = parse_cookies(&hs.request);

        let _ = cookies.get("bzwf_anon_wstx")
            .ok_or(String::from("No bzwf_anon_wstx cookie!"))
            // ^^ TODO?(jordan): ws::Error?
            .and_then(|ticket| Uuid::parse_str(ticket).map_err(|_| String::from("invalid cookie")))
            .map(|ticket| (token, ticket))
            .and_then(|(token, ticket)| self.authorizer.authorize_ticket(token, ticket))
            .map(|conn| ws_conn_log("opn", &conn.borrow(), "cxn opened"))
            .map_err(|err| ws_log("opn", token, &format!("error: {}", err)));

        Ok(())
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        // TODO(jordan): remove a user's Connection struct
        let token = self.out.token();
        self.connections.borrow()
            .get(&token)
            .map(|conn| {
                ws_conn_log("cls", &conn.borrow(), &format!("cxn closed\n\tcode [{:?}] reason: {}", code, reason));
            });

        self.connections
            .borrow_mut()
            .remove(&token);

        ws_log("cls", token, &format!("{} users connected", self.connections.borrow().len()));
    }

    fn on_error(&mut self, err: ws::Error) {
        let token = self.out.token();
        // NOTE(jordan): this happens when we return a ws::Error. Handle it individually. match?
        ws_log("err", token, &format!("got error {}", err));
    }
}

pub fn server (domain: String, port: i32) -> () {
    let connections: ConnectionMap = Rc::new(RefCell::new(HashMap::new()));
    let addr: String = format!("{}:{}", domain, port);
    println!("WebSockets server listening on {}", addr);
    ws::listen(addr, |out| WSServer {
        out: out,
        connections: connections.clone(),
        authorizer: DumbTicketStamper::new(connections.clone()),
    }).unwrap();
}
