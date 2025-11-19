use crate::quote_generator::QuoteGenerator;
use common::errors::symbol::SymbolError;
use common::models::Symbol;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::net::{TcpStream, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use crate::errors::ServerError;

pub fn handle_client(
    stream: TcpStream,
    generator: Arc<Mutex<QuoteGenerator>>,
    udp_socket: Arc<Mutex<UdpSocket>>,
) -> Result<String, ServerError> {
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() != 3 || parts[0] != "STREAM" {
            eprintln!("Invalid command: {}", line);
            continue;
        }

        let udp_addr: std::net::SocketAddr = parts[1]
            .replace("udp://", "")
            .parse()?;

        let tickers: HashSet<String> = parts[2].split(',').map(|s| s.to_string()).collect();

        let symbols: Vec<Symbol> = tickers
            .into_iter()
            .map(|s| {
                s.parse::<Symbol>()
                    .map_err(|_| SymbolError::UnsupportedSymbol)
                    .and_then(|sym| sym.validate().map(|_| sym))
            })
            .collect::<Result<Vec<_>, SymbolError>>()?;

        println!("Starting stream to {} for tickers {:?}", udp_addr, symbols);

        // Создаем UDP сокет для отправки
        udp_socket
            .lock()
            .unwrap()
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();

        let generator = Arc::clone(&generator);
        let symbols_clone = symbols.clone();
        let udp_socket_send = udp_socket.lock().unwrap().try_clone().unwrap();

        // Время последнего Ping от клиента
        let last_ping = Arc::new(Mutex::new(Instant::now()));

        // Поток для стриминга котировок
        let last_ping_stream = Arc::clone(&last_ping);
        thread::spawn(move || {
            loop {
                // Проверяем Keep-Alive
                let elapsed = last_ping_stream.lock().unwrap().elapsed();
                if elapsed > Duration::from_secs(5) {
                    println!("Client {} timed out, stopping stream", udp_addr);
                    break;
                }

                // Генерация и отправка котировок
                for &symbol in &symbols_clone {
                    let quote = {
                        // TODO: генерация должна быть общая для всех клиентов
                        let mut gen_ = generator.lock().unwrap();
                        match gen_.generate_quote(symbol) {
                            Ok(q) => q,
                            Err(_) => continue,
                        }
                    };

                    let msg = format!(
                        "{} {} {} {}",
                        quote.ticker, quote.price, quote.volume, quote.timestamp
                    );

                    let _ = udp_socket_send.send_to(msg.as_bytes(), udp_addr);
                }

                thread::sleep(Duration::from_secs(1));
            }
        });

        let last_ping_monitor = Arc::clone(&last_ping);
        let udp_socket_recv = udp_socket.lock().unwrap().try_clone().unwrap();

        // Поток для приема Ping от клиента
        thread::spawn(move || {
            let mut buf = [0u8; 1024];
            loop {
                match udp_socket_recv.recv_from(&mut buf) {
                    Ok((size, src)) => {
                        let msg = String::from_utf8_lossy(&buf[..size]);
                        if msg.trim() == "Ping" {
                            let mut lp = last_ping_monitor.lock().unwrap();
                            *lp = Instant::now();
                            let _ = udp_socket_recv.send_to(b"Pong", src);
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // Нет данных — продолжаем
                        thread::sleep(Duration::from_millis(50));
                        continue;
                    }
                    Err(e) => eprintln!("UDP error: {}", e),
                }
            }
        });
    }

    Ok("OK".to_string())
}
