use ws;
use time;
use std::fmt;
use uuid::Uuid;
use std::sync::{Arc,Mutex};
use std::collections::{HashMap,HashSet};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Item {
    Potatoes,
    Berries,
    TreeSap,
}

#[derive(Debug, Clone)]
pub struct PlayerState {
    inventory: HashSet<Item>,
}

#[derive(Debug, Clone)]
pub enum Player {
    AnonymousPlayer {
        state : PlayerState,
    },
    RegisteredPlayer {
        id    : i32,
        name  : String,
        state : PlayerState,
    }
}

mod AnonymousPlayer {
    use super::*;
    pub fn new () -> Player {
        let mut new_player_inventory = HashSet::new();
        new_player_inventory.insert(Item::Potatoes);

        Player::AnonymousPlayer {
            state: PlayerState {
                inventory: new_player_inventory
            }
        }
    }
}

mod RegisteredPlayer {
    use super::*;
    pub fn new (id: i32, name: String) -> Player {
        let mut new_player_inventory = HashSet::new();
        new_player_inventory.insert(Item::Potatoes);

        Player::RegisteredPlayer {
            id: id,
            name: name,
            state: PlayerState {
                inventory: new_player_inventory
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Connection {
    uuid   : Uuid,
    player : Player,
}

impl Connection {
    fn new(u: Uuid, p: Player) -> Connection {
        Connection {
            uuid  : u,
            player: p
        }
    }
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "conn{{{}}}", self.uuid)
    }
}

fn parse_cookies (req: &ws::Request) -> HashMap<String, String> {
    req.header("cookie")
        .and_then(|cookies_bytes| String::from_utf8(cookies_bytes.to_vec()).ok())
        .unwrap_or(String::from(""))
        .rsplit(";")
        .filter_map(|cookie_string| {
            let mut cookie_pair = cookie_string.split("=");
            match (cookie_pair.next(), cookie_pair.next()) {
                (Some(name), Some(value)) => {
                    Some((String::from(name.trim()), String::from(value.trim())))
                },
                _ => None
            }
        })
        .collect()
}

fn put_cookie (name: String, value: String, resp: &mut ws::Response) {
    let headers = resp.headers_mut();
    let cookie_bytes = format!("{}={}", name, value).as_bytes().to_vec();
    headers.push((String::from("Set-Cookie"), cookie_bytes));
}

type ConnectionMap = Arc<Mutex<HashMap<Uuid, Connection>>>;

struct WSServer {
    out: ws::Sender,
    connections: ConnectionMap,
}

impl ws::Handler for WSServer {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<ws::Response> {
        let mut resp = ws::Response::from_request(req).unwrap();

        let mut conn_map = self.connections.lock().unwrap();

        println!("ws:req[{}]", time::precise_time_ns());

        // NOTE(jordan): cookie parser
        let cookies : HashMap<String, String> = parse_cookies(req);

        let ticket = cookies.get("bzwf_anon_wstx")
            .and_then(|uuid_string| Uuid::parse_str(uuid_string.as_str()).ok())
            .unwrap_or_else(|| {
                println!("ws:req[{}]: no bzwf_anon_wstx cookie found", time::precise_time_ns());
                Uuid::new_v4()
            });

        let users_count = conn_map.len();

        if conn_map.contains_key(&ticket) {
            println!("ws:req[{}]: reconnect: uuid {}", time::precise_time_ns(), ticket);
        } else {
            println!("ws:req[{}]: new connection with uuid {}", time::precise_time_ns(), ticket);
            println!("ws:req[{}]: putting cookie: bzwf_anon_wstx={}", time::precise_time_ns(), ticket);
            println!("{} connected users", users_count + 1);
            put_cookie(String::from("bzwf_anon_wstx"), ticket.to_string(), &mut resp);
            conn_map.insert(ticket, Connection::new(ticket, AnonymousPlayer::new()));
        }

        Ok(resp)
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        // TODO
        println!("ws:rcv[{}]: user with uuid {} sent ws msg {}",
                 time::precise_time_ns(),
                 "[[[ we don't have uuids in ws yet ]]]",
                 msg);
        self.out.send("I hear you loud and clear").unwrap();
        Ok(())
    }

    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        // TODO
        println!("ws:opn[{}]: user with uuid {} opened ws cxn",
                 time::precise_time_ns(),
                 "[[[ we don't have uuids in ws yet ]]]");
        Ok(())
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        println!("ws:cls[{}]: user with uuid {} closed ws cxn\n\tCode [{:?}] reason: {}",
                 time::precise_time_ns(),
                 "[[[ we don't have uuids in ws yet ]]]",
                 code,
                 reason);
        // TODO
    }

    fn on_error(&mut self, err: ws::Error) {
        println!("ws:err[{}]: user with uuid {} got error {}",
                 time::precise_time_ns(),
                 "[[[ we don't have uuids in ws yet ]]]",
                 err);
        // TODO
    }
}

pub fn server (domain: String, port: i32) -> () {
    let connections : ConnectionMap = Arc::new(Mutex::new(HashMap::new()));
    let addr = format!("{}:{}", domain, port);
    println!("WebSockets server listening on {}", addr);
    ws::listen(addr, |out| WSServer {
        out: out,
        connections: connections.clone(),
    }).unwrap();
}
