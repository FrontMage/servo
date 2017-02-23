extern crate chunked_transfer;
use self::chunked_transfer::Encoder;
use std::io;
use std::io::{Read, Write};
use std::fs::File;

pub struct Error {
    pub code: i32,
    pub msg: String,
    pub origin: io::Error,
}

pub struct Static {
    pub mount_points: Vec<String>,
}

trait Statics {
    fn mount(&mut self, path: &str);
}

impl Statics for Static {
    fn mount(&mut self, path: &str) {
        self.mount_points.push(path.to_owned());
    }
}

pub fn get_file_buffer(path: &str) -> Result<Vec<u8>, Error> {
    let file_path: &str = &("/home/xinbg/Pictures".to_string() + path);
    open_file(file_path)
}

fn open_file(file_path: &str) -> Result<Vec<u8>, Error> {
    return match File::open(file_path) {
        Ok(file) => read_file(file),
        Err(e) => {
            Err(Error {
                code: 404,
                msg: format!("Not Found: {}", file_path),
                origin: e,
            })
        }
    };
}

fn read_file(mut file: File) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();
    return match file.read_to_end(&mut buf) {
        Ok(_) => chunk_encode(buf),
        Err(e) => {
            Err(Error {
                code: 500,
                msg: "Read file error".to_string(),
                origin: e,
            })
        }
    };
}

fn chunk_encode(buf: Vec<u8>) -> Result<Vec<u8>, Error> {
    let mut encoded: Vec<u8> = vec![];
    {
        // Encode error, early return
        if let Err(e) = Encoder::with_chunks_size(&mut encoded, 8).write_all(&buf) {
            return Err(Error {
                code: 500,
                msg: "Chunk encode error".to_string(),
                origin: e,
            });
        }
    }
    Ok(encoded)
}
