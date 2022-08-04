use serde::Deserialize;
use std::cmp::Reverse;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::exit;

use toml;

fn main() {
    let args: Vec<String> = env::args_os()
        .collect::<Vec<OsString>>()
        .iter()
        .map(|s| s.clone().into_string().unwrap())
        .collect();

    let config_path = &args[1];
    let config = read_route_config(config_path);

    let port: u32 = match config.port {
        Some(port) => {
            println!("Running on Port {port}");
            port
        }
        None => match env::var("PORT") {
            Ok(port) => {
                println!("Running on Port {port}");
                port.parse::<u32>().unwrap()
            }
            Err(_) => {
                println!("No Port Configured");
                println!("Please set PORT or configure ports in the configuration file");

                exit(1)
            }
        },
    };

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("{:?}", listener);
    for stream in listener.incoming() {
        handle_connection(stream.unwrap(), config.clone())
    }
}
#[derive(Debug, Deserialize, Clone)]
struct Config {
    port: Option<u32>,
    routes: Vec<RoutesConfig>,
}
#[derive(Debug, Deserialize, Clone)]
struct RoutesConfig {
    location: String,
    source: String,
}

fn read_route_config(path: &str) -> Config {
    let mut f = File::open(path).unwrap();
    let mut buffer = String::new();

    f.read_to_string(&mut buffer).unwrap();

    let config: Config = toml::from_str(&buffer).unwrap();
    config
}

fn read_page_source(path: &str) -> String {
    println!("{path}");
    let mut f = File::open(path).unwrap();
    let mut buffer = String::new();

    f.read_to_string(&mut buffer).unwrap();

    buffer
}

#[derive(Debug)]
struct HTTPRequest {
    method: String,
    location: String,
}

fn new_http_request(method: String, location: String) -> HTTPRequest {
    HTTPRequest { method, location }
}

fn parse_request(request: String) -> HTTPRequest {
    let start_line = request.lines().next().unwrap();
    let words = start_line.split(" ").collect::<Vec<&str>>();
    let method = words.first().unwrap().to_string();
    let location = words[1].to_string();

    new_http_request(method, location)
}

fn handle_connection(mut stream: TcpStream, config: Config) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = parse_request(String::from_utf8_lossy(&buffer[..]).to_string());
    handle_request(&mut stream, request, config);
}

fn handle_request(stream: &mut TcpStream, request: HTTPRequest, config: Config) {
    let mut route_list = config.clone().routes;
    route_list.sort_by(|a, b| b.location.len().cmp(&a.location.len()));
    match route_list
        .iter()
        .find(|route| route.location == request.location)
    {
        Some(route) => {
            let base_response = "HTTP/1.1 200 OK\n".to_owned();
            let content_type = "Content-Type: text/html\r\n\r\n".to_owned();
            let response_body = read_page_source(&route.source).to_owned();
            let response = base_response + &content_type + &response_body;
            let res = response.as_bytes();
            stream.write(res).unwrap();
            stream.flush().unwrap();
        }
        None => {
            let response = "HTTP/1.1 404 Not Found\r\n\r\n".to_owned();
            let res = response.as_bytes();
            stream.write(res).unwrap();
            stream.flush().unwrap();
        }
    };
}
