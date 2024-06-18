mod parsing;
mod models;

use models::Host;
use parsing::request_parser::parse_http_request;

use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

fn main() {
    let bind_addr = Host {
        hostname: "127.0.0.1".to_string(),
        port: 8080,
    };

    let listener = match TcpListener::bind(format!("{}", bind_addr)) {
        Ok(listener) => listener,
        Err(msg) => {
            eprintln!(
                "Failed to bind tcp listener for {}. Reason: {}",
                bind_addr, msg
            );
            return;
        }
    };

    let mut stream_iter = listener.incoming();

    while let Some(Ok(stream)) = stream_iter.next() {
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);

    match parse_http_request(&mut buf_reader) {
        Ok(data) => {
            println!("{}", data);
        }
        Err(msg) => {
            dbg!(msg);
        }
    }
}

