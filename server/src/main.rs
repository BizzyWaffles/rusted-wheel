extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate uuid;

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
            println!("new user w uuid {} connected", new_uuid);
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
            let dropped_conn = conn_map.remove(&req_uuid);

            match dropped_conn {
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
