extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate uuid;
extern crate time;
extern crate ws;

use std::thread;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc,Mutex};
use std::collections::HashMap;

use uuid::Uuid;

use iron::prelude::*;
use iron::status;
use iron::modifiers::Header;
use iron::headers::{AccessControlAllowOrigin};

use mount::Mount;
use staticfile::Static;

pub struct Connection {
    uuid: Uuid
}

fn respond(status: status::Status, msg: &str) -> Response {
    Response::with((status, msg, Header(AccessControlAllowOrigin::Any)))
}

fn respond_ok(msg: &str) -> Response {
    respond(status::Ok, msg)
}

fn main() {
    let connections = Arc::new(Mutex::new(HashMap::new()));

    let mut router = router::Router::new();

    {
        let connections = connections.clone();
        let game_connect = move |req: &mut Request| -> IronResult<Response> {
            let mut conn_map = connections.lock().unwrap();

            let new_uuid = Uuid::new_v4();
            let new_conn = Connection { uuid: new_uuid };
            conn_map.insert(new_conn.uuid, new_conn);
            println!("connect: new user with uuid {}", new_uuid);
            println!("{} connected users", conn_map.len());

            Ok(respond_ok(&format!("{}", new_uuid)))
        };
        router.get("/connect", game_connect, "connect");
    }

    {
        let connections = connections.clone();
        let game_disconnect = move |req: &mut Request| -> IronResult<Response> {
            let mut conn_map = connections.lock().unwrap();

            let mut req_body = String::new();
            /* NOTE(jordan): Must "let _ = ..." Otherwise this gives compiler warning "unused
             * result which must be used"
             */
            /* FIXME(jordan): Is there error-handling we aren't doing here? Whoops... That return
             * value probably *means* something.
             */
            let _ = req.body.read_to_string(&mut req_body);
            let req_uuid = Uuid::parse_str(req_body.as_str()).unwrap();

            match conn_map.remove(&req_uuid) {
                Some(dropped_conn) => {
                    println!("disconnect: user with uuid {}", dropped_conn.uuid);
                    println!("{} connected users", conn_map.len());
                    Ok(respond_ok("dropped"))
                },
                None => {
                    println!("error: disconnect: tried to drop unconnected user {}", req_uuid);
                    Ok(
                        respond(status::Conflict, "error: cannot drop a user who is not connected")
                    )
                },
            }
        };
        router.post("/disconnect", game_disconnect, "disconnect");
    }

    {
        let connections = connections.clone();
        let game_ping = move |req: &mut Request| -> IronResult<Response> {
            if let Ok(conn_map) = connections.lock() {
                let mut req_body = String::new();
                let _ = req.body.read_to_string(&mut req_body);
                let req_uuid = Uuid::parse_str(req_body.as_str()).unwrap();

                match conn_map.get(&req_uuid) {
                    Some(conn) => {
                        println!("ping[{}]: user with uuid {} sent ping",
                                 time::precise_time_ns(),
                                 conn.uuid);
                        Ok(respond_ok("pong"))
                    },
                    None => {
                        println!("error: ping: nonexistent user {} tried to ping", req_uuid);
                        Ok(respond(status::NotFound, ""))
                    },
                }
            } else {
                panic!("error: ping: cannot obtain lock on connections map");
            }
        };

        let game_ping_ref = Arc::new(game_ping);
        for endpoint in vec!["/ping", "/keep-alive"] {
            let handle = game_ping_ref.clone();
            let desc   = format!("ping:{}", endpoint);
            router.post(endpoint, move |req: &mut Request| handle(req), desc);
        }
    }

    let mut assets_mount = Mount::new();
    assets_mount
        .mount("/", router)
        .mount("/client", Static::new(Path::new("../client")));

    let webserver_thread = thread::spawn(move || {
        Iron::new(assets_mount).http("10.105.144.17:3000").unwrap()
    });

    let websocket_thread = thread::spawn(move || {
        struct Server {
            out: ws::Sender,
        }

        impl ws::Handler for Server {
            fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
                // TODO
                println!("ws:rcv[{}]: user with uuid {} sent ws msg {}",
                         time::precise_time_ns(),
                         "[[[ we don't have uuids in ws yet ]]]",
                         msg);
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

        ws::listen("10.105.144.17:3001", |out| Server { out: out }).unwrap()
    });

    // NOTE(jordan): Join all these threads so that we see stdout and stderr.
    webserver_thread.join().unwrap();
    websocket_thread.join().unwrap();
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
