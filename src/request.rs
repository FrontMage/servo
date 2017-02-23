use std::io;
use std::io::Read;
use std::net::TcpStream;
use std::collections::HashMap;

pub struct Error {
    pub code: i32,
    pub level: String,
    pub msg: String,
    pub origin: io::Error,
}

#[derive(Debug)]
pub struct Req {
    pub header: HashMap<String, String>,
    pub body: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub method: String,
    pub path: String,
}

pub fn new(stream: &TcpStream) -> Req {
    match get_request_string(stream) {
        Ok(req_str) => {
            let path = get_path(&req_str);
            let method = get_method(&req_str);
            let query: HashMap<String, String> = get_query_string(&req_str);

            // split request into header and body
            let request: Vec<&str> = req_str.split("\r\n\r\n").collect();
            let headers = request[0];
            let mut headers_hash = HashMap::new();
            let lines: Vec<&str> = headers.split("\n").collect();
            for line in lines {
                let line_vec: Vec<&str> = line.split(": ").collect();
                if line_vec.len() == 2 {
                    let key = line_vec[0].to_string();
                    headers_hash.insert(key, line_vec[1].replace("\r", "").to_string());
                }
            }

            // TODO decode body
            // let body = request[1];

            Req {
                header: headers_hash,
                body: HashMap::new(),
                query: query,
                method: method,
                path: path,
            }
        }
        Err(e) => panic!("Unable to read stream: {}", e.origin),
    }
}

fn get_request_string<'a>(mut stream: &TcpStream) -> Result<String, Error> {
    let mut buf = [0u8; 4096];
    return match stream.read(&mut buf) {
        Ok(_) => Ok(String::from_utf8_lossy(&buf).into_owned()),
        Err(e) => {
            Err(Error {
                code: 500,
                level: "Critical".to_string(),
                msg: "Failed to read stream".to_string(),
                origin: e,
            })
        }
    };
}

fn get_query_string(req_str: &str) -> HashMap<String, String> {
    let path: Vec<&str> = req_str.lines().next().unwrap().split(" ").collect();
    let path_query_string: &str = path[1];
    let path_query_string_vec: Vec<&str> = path_query_string.split("?").collect();
    if path_query_string_vec.len() > 1 {
        let query_string: Vec<&str> = path_query_string_vec[1].split("&").collect();
        let mut query_string_map = HashMap::new();
        for key_value in query_string {
            let key = key_value.split("=").collect::<Vec<&str>>()[0];
            let value = key_value.split("=").collect::<Vec<&str>>()[1];
            query_string_map.insert(key.to_string(), value.to_string());
        }
        query_string_map
    } else {
        HashMap::new()
    }
}

fn get_method(req_str: &str) -> String {
    let method: Vec<&str> = req_str.lines().next().unwrap().split(" ").collect();
    method[0].to_string()
}

fn get_path(req_str: &str) -> String {
    let path: Vec<&str> = req_str.lines().next().unwrap().split(" ").collect();
    let path_query_string: &str = path[1];
    let path_query_string_vec: Vec<&str> = path_query_string.split("?").collect();
    path_query_string_vec[0].to_string()
}
