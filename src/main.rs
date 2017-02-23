extern crate chunked_transfer;
extern crate servo;
use std::net::TcpListener;
use std::thread;
use servo::request;
use servo::request::Req;
use servo::response;
use servo::response::{Res, Response};
use servo::file::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9527").unwrap();
    println!("Listening for connections on port {}", 9527);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    let req_stream = &stream.try_clone().unwrap();
                    let req = request::new(req_stream);
                    let res = response::new(stream.try_clone().unwrap());
                    handle_client(req, res)
                });
            }
            Err(e) => println!("Unable to connect: {}", e),
        }
    }
}

fn handle_client(req: Req, mut res: Res) {
    match get_file_buffer(&req.path) {
        Ok(buf) => {
            println!("{:?}", req);
            res.send(buf).expect("Failed sending response");
        }
        Err(e) => {
            println!("{}", &e.msg);
            res.code(e.code)
                .msg(&e.msg)
                .error(format!("{}", e.origin).into_bytes())
                .expect(&format!("Failed to send response: {}", &e.msg));
        }
    }
}
