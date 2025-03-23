use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use log::{info, error};
use env_logger;

fn handle_client(mut stream: TcpStream) {
    let peer = stream.peer_addr().unwrap_or_else(|_| "unknown".parse().unwrap());

    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(n) => {
            info!("Received from {}: {}", peer, String::from_utf8_lossy(&buffer[..n]));
            let _ = stream.write_all(b"Hello from Rust daemon\n");
        }
        Err(e) => {
            error!("Failed to read from {}: {}", peer, e);
        }
    }
}

fn main() -> std::io::Result<()> {
    env_logger::init();

    let listener = TcpListener::bind("0.0.0.0:7878")?;
    info!("Listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => error!("Connection failed: {}", e),
        }
    }

    Ok(())
}

