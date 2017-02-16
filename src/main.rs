extern crate chunked_transfer;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::fs::File;
use chunked_transfer::Encoder;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9527").unwrap();
    println!("Listening for connections on port {}", 9527);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => println!("Unable to connect: {}", e),
        }
    }
}

fn get_path(mut stream: &TcpStream) -> String {
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            let path: Vec<&str> = req_str.lines().next().unwrap().split(" ").collect();
            println!("GET {}", path[1]);
            // println!("{}", req_str);
            path[1].to_string()
        }
        Err(e) => {
            println!("Unable to read stream: {}", e);
            "/".to_string()
        }
    }
}

fn response(path: &str, mut stream: TcpStream) {
    let file_path: &str = &("/home/xinbg/Downloads/wallpaper".to_string() + path);
    println!("{}", file_path);
    let mut buf = Vec::new();
    if let Ok(mut file) = File::open(file_path) {
        file.read_to_end(&mut buf);

        let mut encoded: Vec<u8> = vec![];
        {
            Encoder::with_chunks_size(&mut encoded, 8).write_all(&buf);
        }

        let headers =
            ["HTTP/1.1 200 OK", "Content-type: image/jpeg", "Transfer-Encoding: chunked", "\r\n"];
        let mut response: Vec<u8> = headers.join("\r\n")
            .to_string()
            .into_bytes();
        response.extend(encoded);

        match stream.write(&response) {
            Ok(_) => println!("Response sent"),
            Err(e) => println!("Failed sending response: {}", e),
        }
    } else {
        let headers = ["HTTP/1.1 404 NOT FOUND", "\r\n"];
        let response = headers.join("\r\n").to_string().into_bytes();
        println!("Not Found: {}", file_path);
        match stream.write(&response) {
            Ok(_) => println!("Response sent"),
            Err(e) => println!("Failed sending response: {}", e),
        }
    }
}

fn handle_client(stream: TcpStream) {
    response(&get_path(&stream), stream);
}

//     let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html;
// charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
