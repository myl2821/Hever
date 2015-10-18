extern crate bufstream;

use std::env;
use std::net::TcpListener;
use std::thread;
use bufstream::BufStream;

mod doit;
mod read_header;
mod parse_uri;
mod util;

fn main() {
    let ip = env::args().nth(1).unwrap();
    let port = env::args().nth(2).unwrap();
    let addr = &*(format!("{}:{}", ip, port));

    let listener = TcpListener::bind(addr).unwrap();
    println!("listening started, ready to accept");
    for stream in listener.incoming() {
        thread::spawn(|| {
            let stream = BufStream::new(stream.unwrap());
            let _ = doit::handle_client(stream);
        });
    }
}

