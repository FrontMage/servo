use std::io::{Write, Error};
use std::net::TcpStream;

pub struct Res {
    pub code: i32,
    pub msg: String,
    pub stream: TcpStream,
}

pub enum Body {
    AsString,
    AsBuffer,
}

pub trait Response {
    fn send(&mut self, body: Vec<u8>) -> Result<String, Error>;
    fn error(&mut self, body: Vec<u8>) -> Result<String, Error>;
}

impl Response for Res {
    fn send(&mut self, body: Vec<u8>) -> Result<String, Error> {
        let headers =
            ["HTTP/1.1 200 OK", "Content-type: image/jpeg", "Transfer-Encoding: chunked", "\r\n"];
        let mut response: Vec<u8> = headers.join("\r\n")
            .to_string()
            .into_bytes();
        response.extend(body);

        return match self.stream.write(&response) {
            Ok(_) => Ok("Response sent".to_string()),
            Err(e) => Err(e),
        };
    }

    fn error(&mut self, body: Vec<u8>) -> Result<String, Error> {
        let headers = [&format!("HTTP/1.1 {code} {msg}", code = self.code, msg = self.msg), "\r\n"];
        let response = headers.join("\r\n").to_string().into_bytes();
        match self.stream.write(&response) {
            Ok(_) => Ok("Response sent".to_string()),
            Err(e) => Err(e),
        }
    }
}
