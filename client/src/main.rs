use std::process;
use std::env;
use std::net::TcpStream;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: client ip port");
        process::exit(1);
    }

    let mut addr = String::new();
    addr.push_str(&args[1]);
    addr.push_str(":");
    addr.push_str(&args[2]);

    let peer = addr.clone();
    match TcpStream::connect(addr) {
        Ok(_) => {
            println!("Connect {} successfully.", peer);
        }
        Err(e) => {
            println!("Connect {} error: {}.", peer, e.description());
        }

    }
}
