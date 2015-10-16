use std::net::TcpStream;
use bufstream::BufStream;
use std::io;
use std::io::prelude::*;

/// Just bypass request headers for now
pub fn read_requesthdrs(stream: &mut BufStream<TcpStream>) -> Result<(), io::Error> {
    loop {
        let mut line = String::new();
        try!(stream.read_line(&mut line));
        if line == "\r\n" {
            break;
        }
    }
    Ok(())
}
