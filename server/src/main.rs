extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;

use std::collections::HashMap;
use std::path::Path;

use iron::prelude::*;
use iron::{Handler};
use iron::status;

use mount::Mount;
use staticfile::Static;

struct Api(f32);

impl Api {
    fn version(&self) -> f32 {
        self.0
    }
}

fn main() {

    Iron::new(|req: &mut Request| {
        let api = Api (0.1);

        let hck = |_: &mut Request| -> IronResult<Response> {
            Ok(Response::with((status::Ok, format!("{}", api.version()))))
        };

        // NOTE: explicitly version the api
        let path = &req.url.path();
        let version = path[0];
        if version != api.version().to_string() {
            panic!("API version mismatch! This API version: {}", api.version());
        }

        println!("{:?}", path);
        println!("{:?}", req);
        println!("{}", req.version);
        println!("{:?}", req.headers);
        println!("{:?}", req.method);
        // println!("{}", req.body);
        Ok(Response::with((status::Ok, "Hello, world!")))
    }).http("localhost:3000").unwrap();

    let mut router = router::Router::new();

    let mut assets_mount = Mount::new();
    assets_mount
        .mount("/", router)
        .mount("/client", Static::new(Path::new("../client")));

    Iron::new(assets_mount).http("localhost:3000").unwrap();

}

// NOTE: a "'static" lifetime means that this refence is used statically when borrowed
fn string_to_print() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    // NOTE: bring into scope everything defined above
    use super::*;

    #[test]
    fn string_to_print_is_correct() {
        assert_eq!(string_to_print(), "Hello, world!");
    }
}
