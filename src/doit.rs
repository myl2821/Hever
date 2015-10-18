use std::net::TcpStream;
use std::io::prelude::*;
use std::io;
use bufstream::BufStream;
use std::ascii::AsciiExt;
use std::collections::HashMap;
use read_header;
use parse_uri;
use util;

pub fn handle_client(mut stream: BufStream<TcpStream>) -> Result<(), io::Error> {
    loop {
        let mut line = String::new();
        try!(stream.read_line(&mut line));
        // get request params
        let req_vec: Vec<&str> = line.split(' ').collect();
        if req_vec.len() < 3 {
            return Ok(());
        }
        let method = req_vec[0].to_ascii_uppercase();
        let uri = req_vec[1];
        let _version = req_vec[2]; // don't handle version for now

        match method.as_ref() {
            "GET" => {
               match read_header::read_requesthdrs(&mut stream) {
                    Ok(_) => {
                        let mut filename = String::new();
                        let mut args = HashMap::new();
                        parse_uri::parse(String::from(uri), &mut filename, &mut args);
                        util::write_head(&mut stream, 13);
                        try!(stream.flush());
                    },
                    Err(_) => {
                        return Ok(());
                    }
               }
            },
            "HEAD" => {
            },
            "POST" => {
            }
            _ => {
            }
        }

        // try!(stream.write(line.as_bytes()));
        // try!(stream.flush());
        return Ok(());
    }
}
