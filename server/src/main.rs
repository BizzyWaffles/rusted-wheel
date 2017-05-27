extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;

use std::path::Path;
use std::sync::{Arc,Mutex};

use iron::prelude::*;
use iron::status;

use mount::Mount;
use staticfile::Static;

fn main() {
    let connections = Arc::new(Mutex::new(0u32));

    let mut router = router::Router::new();

    {
        let conn = connections.clone();
        let game_connect = move |_: &mut Request| -> IronResult<Response> {
            let mut count = conn.lock().unwrap();

            *count += 1;

            Ok(Response::with((status::Ok, format!("{}", *count))))
        };
        router.get("/connect", game_connect, "connect");
    }

    {
        let conn = connections.clone();
        let game_disconnect = move |_: &mut Request| -> IronResult<Response> {
            let mut count = conn.lock().unwrap();

            if *count != 0 {
                *count -= 1;
            }

            Ok(Response::with((status::Ok, format!("{}", *count))))
        };
        router.get("/disconnect", game_disconnect, "disconnect");
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
