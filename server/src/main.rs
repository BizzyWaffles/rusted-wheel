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
use std::fmt::Display;
use std::str::FromStr;

fn env_or <T> (name: &str, default: T) -> T
    where T: Display + FromStr {
    env::var(name).ok().and_then(|v| {
        v.parse::<T>().ok()
    }).unwrap_or_else(|| {
        println!("* {} not set; using {}.", name, default);
        default
    })
}

fn main() {
    let domain : String = env_or("DOMAIN", String::from("localhost"));
    let port   : i32    = env_or("PORT", 3000);

    let tcp_thread_handle = {
        let domain = domain.clone();
        thread::spawn(move || { web::server(domain, port) })
    };

    let ws_thread_handle = {
        let domain = domain.clone();
        let port   = port + 1;
        thread::spawn(move || { sock::server(domain, port) })
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
