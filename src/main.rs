mod config;
mod request;
mod response;

use config::read_route_config;
use config::{get_port, Config};
use request::{handle_request, parse_request};

use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args_os()
        .collect::<Vec<OsString>>()
        .iter()
        .map(|s| s.clone().into_string().unwrap())
        .collect();

    let config = read_route_config(&args[0]);

    let port: u32 = match get_port(&config) {
        Some(port) => port,
        None => exit(1),
    };

    start_server(port, config)
}

fn start_server(port: u32, config: Config) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("{:?}", listener);
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
