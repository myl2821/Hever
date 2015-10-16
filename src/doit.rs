use std::net::TcpStream;
use std::io::prelude::*;
use std::io;
use bufstream::BufStream;
use std::ascii::AsciiExt;

pub fn handle_client(mut stream: BufStream<TcpStream>) -> Result<(), io::Error> {
    println!("incoming req");
    loop {
        let mut line = String::new();
        try!(stream.read_line(&mut line));
        // get request params
        let req_vec: Vec<&str> = line.split(' ').collect();
        if req_vec.len() < 3 {
            return Ok(());
        }
        let method = req_vec[0].to_ascii_uppercase();

        match method.as_ref() {
            "GET" => {
            },
            "HEAD" => {
            },
            _ => {
            }
        }

        try!(stream.write(line.as_bytes()));
        try!(stream.flush());
        return Ok(());
    }
}
