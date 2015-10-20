use std::net::TcpStream;
use std::io::prelude::*;
use std::fs::File;
use bufstream::BufStream;

pub fn write_line(mut stream: &mut BufStream<TcpStream>, s: &String) {
    stream.write((format!("{}\r\n", s)).as_bytes()).ok().expect("1");
}

pub fn write_head(mut stream: &mut BufStream<TcpStream>, size: usize) {
    write_line(stream, &"HTTP/1.0 200 OK".to_string());
    write_line(stream, &"Server: Hever".to_string());
    write_line(stream, &"Content-type: text/html".to_string());
    let len_line = format!("Content-length: text/html: {}\r\n", size);
    write_line(stream, &len_line.to_string());
}

pub fn read_file(path: &str) -> Option<Vec<u8>> {
    let f = File::open(path);
    let mut buffer = Vec::<u8>::new();
    match f {
        Ok(mut f) => {
            f.read_to_end(&mut buffer).unwrap();
            Some(buffer)
        },
        Err(_) => {
            None
        }
    }
}
