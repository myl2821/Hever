use std::net::TcpStream;
use std::io::prelude::*;
use std::io;
use bufstream::BufStream;

pub fn handle_client(mut stream: BufStream<TcpStream>) -> Result<(), io::Error> {
    println!("incoming req");
    loop {
        let mut line = String::new();
        try!(stream.read_line(&mut line));
        print!("{}", line);
        try!(stream.write(line.as_bytes()));
        try!(stream.flush());
    }
}
