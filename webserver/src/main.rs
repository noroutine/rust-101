#![allow(special_module_name)]

use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering}, Arc
    },
    thread::{self, sleep},
    time::Duration,
};

use uuid::Uuid;

use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("[::]:7878").unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

    let pool = ThreadPool::build(5).expect("Pool creation error");
    let running = Arc::new(AtomicBool::new(true));
    let running_ctrlc_clone = Arc::clone(&running);

    let ctrc_handler = ctrlc2::set_handler(move || {
        println!(" ");
        println!("Ctrl-C received, ready to exiting...");
        // https://en.cppreference.com/w/cpp/atomic/memory_order.html
        running_ctrlc_clone.store(false, Ordering::SeqCst);
        true
    })
    .unwrap();
    println!("Ctrl-C to shutdown...");

    while running.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((stream, _)) => {
                stream.set_nonblocking(false).expect("Cannot set blocking");
                let connection_id = Uuid::new_v4();

                pool.execute( connection_id, move || {
                    handle_connection(connection_id, stream);
                });
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // println!("No connection available, sleep briefly");
                thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
                break;
            }
        }
    }

    println!("Got it! Shutting down...");
    ctrc_handler.join().unwrap();
}

fn handle_connection(connection_id: Uuid, mut stream: TcpStream) {
    // Set read timeout
    stream
        .set_read_timeout(Some(Duration::from_millis(10)))
        .unwrap();

    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .take(100) // Limit to 100 lines max
        .map(|result| match result {
            Ok(line) => line,
            Err(e) => {
                eprintln!("[{}]: Read error: {}", connection_id, e);
                return String::new(); // Return empty string on error
            }
        })
        .take_while(|line| !line.is_empty())
        .collect();

    if http_request.is_empty() {
        eprintln!("[{}] No valid HTTP request received", connection_id);
        return;
    }

    let response = match http_request.iter().next() {
        Some(request_line) if request_line == "GET / HTTP/1.1" || request_line == "GET / HTTP/1.0" => {
            println!("[{}] {}", connection_id, request_line);

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
        Some(request_line) if request_line.ends_with("HTTP/1.1") || request_line.ends_with("HTTP/1.0") => {
            println!("[{connection_id}] {request_line}");

            // thread::sleep(Duration::from_millis(100));

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
            println!("[{connection_id}] bad request {:?}", http_request);

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
