mod config;
mod request;
mod response;

use clap::Parser;

use config::read_route_config;
use config::{get_port, Config};
use request::{handle_request, parse_request};

use std::fs::File;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::exit;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short, long, value_parser)]
    port: Option<u32>,
    #[clap(short, long, value_parser)]
    config: PathBuf,
}

fn main() {
    let args = Args::parse();

    let config = read_route_config(args.config.to_str().unwrap());

    let port: u32 = match args.port {
        Some(port) => port,
        None => match get_port(&config) {
            Some(port) => port,
            None => exit(1),
        },
    };

    start_server(port, config)
}

fn start_server(port: u32, config: Config) {
    let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)) {
        Ok(l) => {
            let port = l.local_addr().unwrap().port();
            let address = l.local_addr().unwrap().ip();
            println!("Serving HTTP on {address} port {port} (http://{address}:{port}/)");
            l
        }
        Err(e) => {
            eprintln!("Unable to open port {port}");
            eprintln!("Error: {:?}", e.kind());

            exit(1);
        }
    };
    for stream in listener.incoming() {
        handle_connection(stream.unwrap(), config.to_owned())
    }
}
fn read_page_source(path: &str) -> String {
    let mut f = File::open(path).unwrap();
    let mut buffer = String::new();

    f.read_to_string(&mut buffer).unwrap();

    buffer
}

fn handle_connection(mut stream: TcpStream, config: Config) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    handle_request(
        &mut stream,
        parse_request(&String::from_utf8_lossy(&buffer[..])),
        config,
    );
}
