use mount::Mount;
use std::path::Path;
use iron::prelude::*;
use staticfile::Static;

pub fn server (domain: String) -> () {
    let addr = format!("{}:3000", domain);
    println!("TCP server listening on {}", addr);
    println!("Go to http://{}/client/html", addr);

    let mut assets_mount = Mount::new();
    assets_mount.mount("/client", Static::new(Path::new("../client/")));

    Iron::new(assets_mount).http(addr.clone()).unwrap();
}
