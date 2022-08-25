mod config;
mod mimetypes;
mod request;
mod response;

use clap::Parser;

use config::read_route_config;
use config::{get_port, Config};
use request::{handle_request, parse_request};

use std::fs::File;
use std::env;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
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

fn expand_tidle(path: &str) -> String {
    match path.chars().next() {
        Some('~') => {
            let home_directory = env::var("HOME").unwrap();
            let rest = path.split_at(1);
            format!("{}{}", home_directory, rest.1)
        }
        _ => path.to_string(),
    }
}
fn read_page_source(path: &str) -> Vec<u8> {
    let expanded_path = expand_tidle(path);
    println!("Expanded File: {}", expanded_path);

    let mut f = File::open(&expanded_path).unwrap();
    let mut buffer = Vec::new();

    if Path::new(&expanded_path).is_dir() {
        return read_page_source(&format!("{path}/index.html"));
    }

    match f.read_to_end(&mut buffer) {
        Ok(_f) => {}
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => eprintln!("Error: {path} No such File or Directory"),
            k => eprintln!("Error: {k:#?}\n Kind: {:#?}", e.kind()),
        },
    };
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
