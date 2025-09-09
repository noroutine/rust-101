#![allow(special_module_name)]

use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use uuid::Uuid;

use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("[::]:7878").unwrap();
    let pool = ThreadPool::build(4).expect("Pool creation error");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let connection_id = Uuid::new_v4();

    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("[{connection_id}] Request: {http_request:#?}");

    let response = match http_request.iter().next() {
        Some(request_line) if request_line == "GET / HTTP/1.1" => {
            let status_line = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string("assets/index.html").unwrap();
            let length = contents.len();

            let response_headers = [
                "Content-Type: text/html",
                format!("Content-Length: {length}").as_str(),
            ]
            .join("\r\n");

            format!("{status_line}\r\n{response_headers}\r\n\r\n{contents}")
        }
        Some(request_line) if request_line.ends_with("HTTP/1.1") => {
            let status_line = "HTTP/1.1 404 Not found";
            let contents = fs::read_to_string("assets/404.html").unwrap();
            let length = contents.len();

            let response_headers = [
                "Content-Type: text/html",
                format!("Content-Length: {length}").as_str(),
            ]
            .join("\r\n");

            format!("{status_line}\r\n{response_headers}\r\n\r\n{contents}")
        }
        _ => {
            let status_line = "HTTP/1.1 400 Bad request";
            let contents = "Bad request";
            let length = contents.len();

            let response_headers = [
                "Content-Type: text/plain",
                format!("Content-Length: {length}").as_str(),
            ]
            .join("\r\n");

            format!("{status_line}\r\n{response_headers}\r\n\r\n{contents}")
        }
    };

    stream.write_all(response.as_bytes()).unwrap();
}
