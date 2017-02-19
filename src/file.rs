extern crate chunked_transfer;
use self::chunked_transfer::Encoder;
use std::io::{Read, Write, Error};
use std::fs::File;
use std::net::TcpStream;

pub fn get_file_buffer(path: &str) -> Result<Vec<u8>, Error> {
    let file_path: &str = &("/home/xinbg/Pictures".to_string() + path);
    println!("{}", file_path);

    open_file(file_path)
}

pub fn get_path(mut stream: &TcpStream) -> String {
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            let path: Vec<&str> = req_str.lines().next().unwrap().split(" ").collect();
            println!("GET {}", path[1]);
            path[1].to_string()
        }
        Err(e) => {
            println!("Unable to read stream: {}", e);
            "/".to_string()
        }
    }
}

fn open_file(file_path: &str) -> Result<Vec<u8>, Error> {
    return match File::open(file_path) {
        Ok(file) => read_file(file),
        Err(e) => {
            println!("Not Found: {}", file_path);
            Err(e)
        }
    };
}

fn read_file(mut file: File) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();
    return match file.read_to_end(&mut buf) {
        Ok(_) => chunk_encode(buf),
        Err(e) => {
            println!("Read file error");
            Err(e)
        }
    };
}

fn chunk_encode(buf: Vec<u8>) -> Result<Vec<u8>, Error> {
    let mut encoded: Vec<u8> = vec![];
    {
        // Encode error, early return
        if let Err(e) = Encoder::with_chunks_size(&mut encoded, 8).write_all(&buf) {
            println!("Chunk encoded error");
            return Err(e);
        }
    }
    Ok(encoded)
}
