use ws;
use std; // NOTE: in submodules, lookup path is relative to this module, so we must `use std`
use time;
use uuid::Uuid;
use std::sync::{Arc,Mutex};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Connection {
    uuid: Uuid,
}

impl std::fmt::Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "conn{{{}}}", self.uuid)
    }
}

type ConnectionMap = Arc<Mutex<HashMap<Uuid,Connection>>>;

struct WSServer {
    out: ws::Sender,
    connections: ConnectionMap,
}

impl ws::Handler for WSServer {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<ws::Response> {
        let mut resp = ws::Response::from_request(req).unwrap();

        let mut conn_map = self.connections.lock().unwrap();

        println!("ws:req[{}]", time::precise_time_ns());

        let new_uuid = Uuid::new_v4();
        let new_conn = Connection {
            uuid: new_uuid
        };
        conn_map.insert(new_conn.uuid, new_conn);
        println!("ws:req: new user with uuid {}", new_uuid);
        println!("{} connected users", conn_map.len());

        println!("ws:req: putting cookie: bzwf_anon_wstx={}; ", new_conn);
        let new_bzwf_cookie_str = format!("bzwf_anon_wstx={}; ", new_conn);
        {
            let headers = resp.headers_mut();
            headers.push((
                String::from("Set-Cookie"),
                new_bzwf_cookie_str.as_bytes().to_vec()
            ));
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

pub fn server (domain: String) -> () {
    let connections : ConnectionMap = Arc::new(Mutex::new(HashMap::new()));
    let addr = format!("{}:3001", domain);
    println!("WebSockets server listening on {}", addr);
    ws::listen(addr, |out| WSServer {
        out: out,
        connections: connections.clone(),
    }).unwrap();
}
