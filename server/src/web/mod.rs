//! web
//!
//! Web is a static file server for putting the front end up.
//! Sister server to `sock`, which is the websocket server.
use mount::Mount;
use std::path::Path;
use iron::prelude::*;
use staticfile::Static;

pub fn server (domain: String, port: i32) -> () {
    let addr = format!("{}:{}", domain, port);
    println!("TCP server listening on {}", addr);
    println!("Go to http://{}/client/html", addr);

    let mut assets_mount = Mount::new();
    assets_mount.mount("/client", Static::new(Path::new("../client/")));

    Iron::new(assets_mount).http(addr.clone()).unwrap();
}
