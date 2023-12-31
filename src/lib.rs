use std::{
    fs,
    io::{prelude::*, BufReader},
    net::TcpStream,
};

use crate::http::{HttpResponse, HttpStatus};

pub mod http;
pub mod threading;

// TODO: figure out how to get content from request
pub fn handler(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let start_line = &http_request[0];

    let http_response: HttpResponse = match start_line.as_str() {
        "GET / HTTP/1.1" => file_handler("hello.html"),
        _ => not_found_handler(),
    };

    stream.write_all(http_response.format().as_bytes()).unwrap();
}

pub fn file_handler(filename: &str) -> HttpResponse {
    match fs::read_to_string(format!("pages/{}", filename)) {
        Ok(content) => HttpResponse::new(HttpStatus::new(200, "Ok".to_string()), content),
        Err(_err) => match fs::read_to_string("pages/404.html") {
            Ok(content) => {
                HttpResponse::new(HttpStatus::new(404, "Not Found".to_string()), content)
            }
            Err(_err) => HttpResponse::new(
                HttpStatus::new(400, "Internal Server Error".to_string()),
                "".to_string(),
            ),
        },
    }
}

pub fn not_found_handler() -> HttpResponse {
    match fs::read_to_string("pages/404.html") {
        Ok(content) => HttpResponse::new(HttpStatus::new(404, "Not Found".to_string()), content),
        Err(_err) => HttpResponse::new(
            HttpStatus::new(400, "Internal Server Error".to_string()),
            "".to_string(),
        ),
    }
}
