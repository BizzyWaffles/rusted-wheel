extern crate ws;
extern crate uuid;
extern crate time;
extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;

mod web;
mod sock;

use std::env;
use std::thread;

fn main() {
    let domain : String = match env::var("DOMAIN") {
        Ok(val) => val,
        Err(_)  => {
            println!("* DOMAIN not set; using localhost.");
            String::from("localhost")
        }
    };

    let tcp_thread_handle = {
        let domain = domain.clone();
        thread::spawn(|| { web::server(domain) })
    };

    let ws_thread_handle = {
        let domain = domain.clone();
        thread::spawn(|| { sock::server(domain) })
    };

    tcp_thread_handle.join().unwrap();
    ws_thread_handle.join().unwrap();
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
