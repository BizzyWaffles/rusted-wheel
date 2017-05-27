extern crate iron;

use iron::prelude::*;
use iron::status;

fn hck(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "")))
}

fn main() {
    Iron::new(|req: &mut Request| {
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
