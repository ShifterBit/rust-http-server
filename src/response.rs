pub enum HTTPResponseCode {
    OK,
    NotFound,
}
impl HTTPResponseCode {
    fn to_string(&self) -> &str {
        match self {
            HTTPResponseCode::OK => "200 OK",
            HTTPResponseCode::NotFound => "404 Not Found",
        }
    }
}

pub struct HTTPResponse<'a> {
    code: HTTPResponseCode,
    content_type: &'a str,
    body: &'a str,
}

impl HTTPResponse<'_> {
    pub fn new<'a>(code: HTTPResponseCode, content_type: &'a str, body: &'a str) -> HTTPResponse<'a> {
        HTTPResponse {
            code,
            content_type,
            body,
        }
    }
}

impl ToString for HTTPResponse<'_> {
    fn to_string(&self) -> String {
        let base = format!("HTTP/1.1 {}", self.code.to_string());
        let content_type = format!("Content-Type: {}", self.content_type);
        let header_end = "\r\n\r\n";
        let body = self.body;

        format!(
            "{base}\n
                {content_type}\n
                {header_end}
                {body}"
        )
    }
}
