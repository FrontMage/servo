extern crate chunked_transfer;
extern crate servo;
use std::net::{TcpListener, TcpStream};
use std::thread;
use servo::res::{Res, Response};
use servo::file::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9527").unwrap();
    println!("Listening for connections on port {}", 9527);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    let mut res = Res {
                        code: 200,
                        msg: "OK".to_string(),
                        stream: stream.try_clone().unwrap(),
                    };
                    handle_client(stream, &mut res)
                });
            }
            Err(e) => println!("Unable to connect: {}", e),
        }
    }
}

fn handle_client(stream: TcpStream, mut res: &mut Res) {
    let buf = get_file_buffer(&get_path(&stream)).unwrap();
    res.send(buf).expect("Response sent");
    // res.error("".to_string().into_bytes()).expect("Failed sending response");
}
