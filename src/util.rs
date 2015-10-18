use std::net::TcpStream;
use std::io::prelude::*;
//use std::io;
use bufstream::BufStream;

pub fn write_line(mut stream: &mut BufStream<TcpStream>, s: &String) {
    stream.write((format!("{}\r\n", s)).as_bytes()).unwrap();
}

pub fn write_head(mut stream: &mut BufStream<TcpStream>, size: usize) {
    write_line(stream, &"HTTP/1.0 200 OK".to_string());
    write_line(stream, &"Server: Hever".to_string());
    write_line(stream, &"Content-type: text/plain".to_string());
    let len_line = format!("Content-length: text/plain: {}\r\n", size);
    write_line(stream, &len_line.to_string());
    write_line(stream, &"Hello world!".to_string());
}

