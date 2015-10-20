extern crate bufstream;
extern crate rustc_serialize;
extern crate docopt;

use std::net::TcpListener;
use std::thread;
use bufstream::BufStream;
use docopt::Docopt;

mod doit;
mod read_header;
mod parse_uri;
mod util;

const USAGE: &'static str = "
Hever.

Usage:
  hever -b <bind> -p <port> -d <public-folder>

Options:
  -h --help     Show this screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_bind: String,
    arg_port: u16,
    arg_public_folder: Option<String>,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    let ip = args.arg_bind;
    let port = args.arg_port;
    let pub_folder = args.arg_public_folder;
    let addr = &*(format!("{}:{}", ip, port));

    let listener = TcpListener::bind(addr).unwrap();
    println!("listening on {}:{}, ready to accept", ip, port);
    for stream in listener.incoming() {
        thread::spawn(|| {
            let stream = BufStream::new(stream.unwrap());
            let _ = doit::handle_client(stream);
        });
    }
}

