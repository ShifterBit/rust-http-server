use std::io::Write;
use std::net::TcpStream;

use crate::mimetypes::infer_mimetype;
use crate::response::{HTTPResponse, HTTPResponseCode};
use crate::{config::Config, read_page_source};

#[derive(Debug)]
pub struct HTTPRequest<'a> {
    pub method: &'a str,
    pub location: &'a str,
}
impl HTTPRequest<'_> {
    pub fn new<'a>(method: &'a str, location: &'a str) -> HTTPRequest<'a> {
        HTTPRequest { method, location }
    }
}

pub fn parse_request<'a>(request: &'a str) -> HTTPRequest<'a> {
    let start_line = request.lines().next().unwrap();
    let words = start_line.split(" ").collect::<Vec<&str>>();
    let method = words.first().unwrap();
    let location = words[1];

    HTTPRequest::new(method, location)
}

pub fn handle_request(stream: &mut TcpStream, request: HTTPRequest, config: Config) {
    match config
        .routes
        .iter()
        .find(|route| route.location == request.location)
    {
        Some(route) => {
            let response_body = read_page_source(&route.source);
            let response = HTTPResponse::new(
                HTTPResponseCode::OK,
                infer_mimetype(&route.source),
                response_body,
            );
            let mut res: Vec<u8> = response.headers().clone().concat().as_bytes().to_vec();
            res.extend(response.body);

            stream.write(&res).unwrap();
            stream.flush().unwrap();
        }
        None => {
            let response =
                HTTPResponse::new(HTTPResponseCode::NotFound, infer_mimetype(""), Vec::new());
            stream
                .write(response.headers().concat().as_bytes())
                .unwrap();
            stream.flush().unwrap();
        }
    };
}
