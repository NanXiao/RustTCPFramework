use std::net::TcpListener;
use std::error::Error;

fn main() {
    let listener: TcpListener;
    match TcpListener::bind("127.0.0.1:8001") {
        Ok(l) => {
            listener = l;
            println!("Bind successfully.");
        },
        Err(e) => {
            println!("Bind error: {}.", e.description());
        }
    }
}