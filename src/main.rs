extern crate bufstream;

use std::net::TcpListener;
use std::thread;
use bufstream::BufStream;

mod doit;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9123").unwrap();
    println!("listening started, ready to accept");
    for stream in listener.incoming() {
        thread::spawn(|| {
            let stream = BufStream::new(stream.unwrap());

            doit::handle_client(stream);
        });
    }
}

