extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate uuid;
extern crate time;
extern crate ws;

use std::env;
use std::thread;
use std::path::Path;
use std::sync::{Arc,Mutex};
use std::collections::HashMap;

use uuid::Uuid;

use iron::prelude::*;

use mount::Mount;
use staticfile::Static;

#[derive(Debug, Clone, Copy)]
pub struct Connection {
    uuid: Uuid,
}

impl std::fmt::Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "conn{{{}}}", self.uuid)
    }
}

fn main() {
    let connections = Arc::new(Mutex::new(HashMap::new()));

    struct WSServer {
        out: ws::Sender,
        connections: Arc<Mutex<HashMap<Uuid, Connection>>>
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
            println!("wsconnect: new user with uuid {}", new_uuid);
            println!("{} connected users", conn_map.len());

            println!("wsconnect: putting cookie: bzwf_anon_wstx={}", new_conn);
            let new_bzwf_cookie_str = format!("bzwf_anon_wstx={}", new_conn);
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

    {
        let domain : String;

        match env::var("DOMAIN") {
            Ok(val) => domain = val,
            Err(_)  => {
                println!("* DOMAIN not set; using localhost.");
                domain = String::from("localhost")
            }
        }

        let web_domain = domain.clone();
        let webserver_thread = thread::spawn(move || {
            let web_addr = format!("{}:3000", web_domain);
            println!("TCP server listening on {}", web_addr);
            println!("\tGo to: http://{}/client/html", web_addr);

            let mut assets_mount = Mount::new();
            assets_mount
                .mount("/client", Static::new(Path::new("../client")));

            Iron::new(assets_mount).http(web_addr.clone()).unwrap()
        });

        let ws_domain = domain.clone();
        let websocket_thread = thread::spawn(move || {
            let ws_addr = format!("{}:3001", ws_domain);
            println!("WebSockets server listening on {}", ws_addr);
            ws::listen(ws_addr.clone(), |out| WSServer {
                out: out,
                connections: connections.clone()
            }).unwrap();
        });

        webserver_thread.join().unwrap();
        websocket_thread.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    // NOTE: bring into scope everything defined above
    use super::*;

    #[test]
    fn it_works() {
        // TODO: write a test
    }
}
