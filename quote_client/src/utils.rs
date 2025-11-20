use clap::Parser;
use std::io::Read;
use std::path::PathBuf;
use std::string::String;

#[derive(Parser, Debug)]
#[command(name = "quote_client")]
#[command(about = "Listens to quotes data on provided UDP port")]
pub struct Args {
    /// {addr}:{port}
    #[arg(long, default_value = "127.0.0.1:7878")]
    pub tcp_address: String,

    /// UDP port to receive data
    #[arg(short, long, default_value_t = 9001)]
    pub port: u16,

    /// Input file path
    #[arg(short, long, default_value = "tickers.txt")]
    pub input: PathBuf,
}

pub struct Config {
    pub udp_port: u16,
    pub udp_addr: String, // Using raw types is an antipattern in rust, read abount new-type pattern someday but for now will work
    pub udp_ping_addr: String, // Using raw types is an antipattern in rust, read abount new-type pattern someday but for now will work
}

impl Config {
    pub fn build(udp_port: u16) -> Self {
        let udp_addr = format!("127.0.0.1:{udp_port}");
        let udp_ping_addr = "127.0.0.1:9000".to_string();
        Config {
            udp_port,
            udp_addr,
            udp_ping_addr,
        }
    }
}

pub fn read_tickers<R: Read>(r: &mut R) -> Result<String, std::io::Error> {
    let mut s = String::new();
    r.read_to_string(&mut s)?;

    // удаляем пустые строки и пробелы по краям
    let tickers: Vec<&str> = s
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    Ok(tickers.join(","))
}
