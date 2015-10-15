use std::net::TcpStream;
use std::io::prelude::*;
use bufstream::BufStream;

pub fn handle_client(mut stream: BufStream<TcpStream>) {
    println!("incoming req");
    loop {
        let mut line = String::new();
        let _ = stream.read_line(&mut line).unwrap();
        print!("{}", line);
        stream.write(line.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
