extern crate iron;

use iron::prelude::*;
use iron::status;

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
