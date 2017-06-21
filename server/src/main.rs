use std::env;
use std::net::TcpListener;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut addr = String::new();
    let mut ip = "127.0.0.1";
    let mut port = "8001";

    if args.len() == 1 {
    } else if args.len() == 2 {
        ip = &args[1];
    } else {
        ip = &args[1];
        port = &args[2];
    }

    addr.push_str(ip);
    addr.push_str(":");
    addr.push_str(port);

    let listener: TcpListener;
    match TcpListener::bind(addr) {
        Ok(l) => {
            listener = l;
            println!("Bind {}:{} successfully.", ip, port);
        },
        Err(e) => {
            println!("Bind {}:{} error: {}.", ip, port, e.description());
            return;
        }
    }

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let peer = stream.peer_addr();
                match peer {
                    Ok(p) => {
                        println!("Client address: {}:{}.", p.ip(), p.port());
                    },
                    Err(e) => {
                        println!("Connection error: {}.", e.description());
                    }
                }
            }
            Err(e) =>  {
                println!("Connection error: {}.", e.description());
            }
        }
    }
}