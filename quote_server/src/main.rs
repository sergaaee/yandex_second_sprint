mod quote_generator;
mod server;
mod errors;
mod constants;

use crate::quote_generator::QuoteGenerator;
use crate::server::handle_client;
use std::net::{TcpListener, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::constants::SERVER_PORT;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{SERVER_PORT}"))?;
    println!("Server listening on port {SERVER_PORT}");
    let generator = Arc::new(Mutex::new(QuoteGenerator::new()));
    let udp_socket = Arc::new(Mutex::new(
        UdpSocket::bind("0.0.0.0:9000")?,
    ));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New TCP client: {}", stream.peer_addr()?);
                let generator = Arc::clone(&generator);
                let udp_clone = Arc::clone(&udp_socket);
                thread::spawn(move || handle_client(stream, generator, udp_clone));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}
