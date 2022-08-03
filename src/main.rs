use std::{
    env,
    fmt::format,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: \n{}", String::from_utf8_lossy(&buffer[..]));

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap()
}

fn main() {
    let port: u32 = match env::var("PORT") {
        Ok(v) => {
            println!("Running on Port {}", v);
            v.parse::<u32>().unwrap()
        }
        Err(_e) => {
            println!("$PORT not set, defaulting to 8000");
            println!("Running on Port 8000");
            8000 as u32
        }
    };

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("Hello, world!, on port {}", port);
    println!("{:?}", listener);
    for stream in listener.incoming() {
        handle_connection(stream.unwrap())
    }
}
