mod utils;

use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpStream, UdpSocket};
use utils::Args;

use crate::utils::{Config, read_tickers};
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut file = File::open(&args.input)?;

    let config = Config::build(args.port);

    // Создаем UDP сокет и биндимся на локальном порту
    let udp_socket = UdpSocket::bind(format!("127.0.0.1:{}", config.udp_port))?;
    println!("Listening for UDP stream on {}", config.udp_addr);

    // Поток для приема UDP сообщений
    let udp_socket_clone = udp_socket.try_clone()?;
    thread::spawn(move || {
        let mut buf = [0u8; 1024];
        loop {
            match udp_socket_clone.recv_from(&mut buf) {
                Ok((size, src)) => {
                    let msg = String::from_utf8_lossy(&buf[..size]);
                    println!("UDP from {}: {}", src, msg);
                }
                Err(e) => eprintln!("UDP recv error: {}", e),
            }
        }
    });

    // TCP соединение с сервером
    let mut tcp_stream = TcpStream::connect(&args.tcp_address)?;
    println!("Connected to TCP server");

    // Тикеры
    let tickers = read_tickers(&mut file)?;

    // Формируем команду STREAM
    let command = format!("STREAM udp://{} {}\n", config.udp_addr, tickers);
    tcp_stream.write_all(command.as_bytes())?;
    println!("Command sent: {}", command.trim());

    // Читаем ответ TCP (например "OK")
    let mut reader = BufReader::new(tcp_stream.try_clone()?);
    let mut response = String::new();
    let udp_socket_clone_for_ping = udp_socket.try_clone()?;
    thread::spawn(move || {
        loop {
            if let Err(e) = udp_socket_clone_for_ping.send_to(b"Ping", &config.udp_ping_addr) {
                eprintln!("Failed to send Ping: {}", e);
            }
            thread::sleep(Duration::from_secs(2));
        }
    });
    reader.read_line(&mut response)?;

    Ok(())
}
