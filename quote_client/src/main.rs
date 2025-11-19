use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpStream, UdpSocket};

use std::thread;
use std::time::Duration;


fn main() -> io::Result<()> {
    // UDP порт, на который сервер будет стримить котировки
    let udp_port = 9001;
    let udp_ping_port = 9000;
    let udp_address = format!("127.0.0.1:{}", udp_port);
    let udp_ping_addr = format!("127.0.0.1:{udp_ping_port}",);

    // Создаем UDP сокет и биндимся на локальном порту
    let udp_socket = UdpSocket::bind(format!("127.0.0.1:{}", udp_port))?;
    println!("Listening for UDP stream on {}", udp_address);

    // Создаем UDP сокет и биндимся на локальном порту PING
    let udp_ping_socket = UdpSocket::bind("0.0.0.0:0")?;
    println!("Listening for UDP stream on {}", udp_address);

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
    let mut tcp_stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected to TCP server");

    // Тикеры
    let tickers = "TSLA,MSFT";

    // Формируем команду STREAM
    let command = format!("STREAM udp://{} {}\n", udp_address, tickers);
    tcp_stream.write_all(command.as_bytes())?;
    println!("Command sent: {}", command.trim());

    // Читаем ответ TCP (например "OK")
    let mut reader = BufReader::new(tcp_stream.try_clone()?);
    let mut response = String::new();
    thread::spawn(move || {
        loop {
            if let Err(e) = udp_ping_socket.send_to(b"Ping", &udp_ping_addr) {
                eprintln!("Failed to send Ping: {}", e);
            }
            thread::sleep(Duration::from_secs(2));
        }
    });
    reader.read_line(&mut response)?;

    Ok(())
}
