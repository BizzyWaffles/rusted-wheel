extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate uuid;
extern crate time;

use std::io::Read;
use std::path::Path;
use std::sync::{Arc,Mutex};
use std::collections::HashMap;

use uuid::Uuid;

use iron::prelude::*;
use iron::status;

use mount::Mount;
use staticfile::Static;

pub struct Connection {
    uuid: Uuid
}

fn main() {
    let connections = Arc::new(Mutex::new(HashMap::new()));

    let mut router = router::Router::new();

    {
        let connections = connections.clone();
        let game_connect = move |_: &mut Request| -> IronResult<Response> {
            let mut conn_map = connections.lock().unwrap();

            let new_uuid = Uuid::new_v4();
            let new_conn = Connection { uuid: new_uuid };
            conn_map.insert(new_conn.uuid, new_conn);
            println!("connect: new user with uuid {}", new_uuid);
            println!("{} connected users", conn_map.len());

            Ok(Response::with((status::Ok, format!("{}", new_uuid))))
        };
        router.get("/connect", game_connect, "connect");
    }

    {
        let connections = connections.clone();
        let game_disconnect = move |req: &mut Request| -> IronResult<Response> {
            let mut conn_map = connections.lock().unwrap();

            let mut req_body = String::new();
            // NOTE(jordan): Must "let _ = ..." Otherwise this gives compiler warning "unused
            // result which must be used"
            let _ = req.body.read_to_string(&mut req_body);
            let req_uuid = Uuid::parse_str(req_body.as_str()).unwrap();

            match conn_map.remove(&req_uuid) {
                Some(dropped_conn) => {
                    println!("disconnect: user with uuid {}", dropped_conn.uuid);
                    println!("{} connected users", conn_map.len());
                    Ok(Response::with((status::Ok, "dropped")))
                },
                None => {
                    println!("error: disconnect: tried to drop unconnected user {}", req_uuid);
                    Ok(
                        Response::with((
                            status::Conflict,
                            "error: cannot drop a user who is not connected"
                        ))
                    )
                },
            }
        };
        router.post("/disconnect", game_disconnect, "disconnect");
    }

    {
        let connections = connections.clone();
        let game_ping = move |req: &mut Request| -> IronResult<Response> {
            let conn_map = connections.lock().unwrap();

            let mut req_body = String::new();
            let _ = req.body.read_to_string(&mut req_body);
            let req_uuid = Uuid::parse_str(req_body.as_str()).unwrap();

            match conn_map.get(&req_uuid) {
                Some(conn) => {
                    println!("ping[{}]: user with uuid {} sent ping",
                             time::precise_time_ns(),
                             conn.uuid);
                    Ok(Response::with((status::Ok, "pong")))
                },
                None => {
                    println!("error: ping: nonexistent user {} tried to ping", req_uuid);
                    Ok(Response::with((status::NotFound, "")))
                },
            }
        };
        let game_ping_ref = Arc::new(game_ping);
        let game_ping_1 = game_ping_ref.clone();
        router.post("/ping", move |req: &mut Request| game_ping_1(req), "ping:ping");
        let game_ping_2 = game_ping_ref.clone();
        router.post("/keep-alive", move |req: &mut Request| game_ping_2(req), "ping:keep-alive");
    }

    let mut assets_mount = Mount::new();
    assets_mount
        .mount("/", router)
        .mount("/client", Static::new(Path::new("../client")));

    Iron::new(assets_mount).http("localhost:3000").unwrap();
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
