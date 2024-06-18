
mod models;
mod request_parser;

use models::Host;
use request_parser::HttpElemIterator;

use std::net::{TcpListener, TcpStream};
use std::io::BufReader;

fn main() {
    let bind_addr = Host {
        hostname: "127.0.0.1".to_string(),
        port: 7878,
    };

    match TcpListener::bind(format!("{}", bind_addr)) {
        Ok(listener) => {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        handle_connection(stream);
                    }
                    Err(msg) => {
                        eprintln!(
                            "Failed to handle stream on addr: {}. Reason: {}",
                            bind_addr, msg
                        );
                    }
                }
            }
        }
        Err(msg) => {
            eprintln!(
                "Failed to bind tcp listener for {}. Reason: {}",
                bind_addr, msg
            );
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    for elem in buf_reader.http_elems() {
        match elem {
            Ok(data) => {
                println!("Data: {}", data);
            }
            Err(msg) => {
                dbg!(msg);
            }
        }
    }
}

