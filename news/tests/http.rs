use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

use news::server;


/* GET '/' */
#[test]
fn get_root_returns_200() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    thread::spawn(move || {
        server::run_test(listener);
    });

    let mut stream = std::net::TcpStream::connect(addr).unwrap();

    stream
        .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")
        .unwrap();

    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();

    assert!(response.starts_with("HTTP/1.1 200 OK"));
}


/* Method Rejection */
#[test]
fn post_is_rejected() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    std::thread::spawn(move || {
        server::run_test(listener);
    });

    let mut stream = std::net::TcpStream::connect(addr).unwrap();

    stream
        .write_all(b"POST / HTTP/1.1\r\nHost: localhost\r\n\r\n")
        .unwrap();

    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();

    assert!(response.starts_with("HTTP/1.1 405"));
}


/* Over Sized Request Line */
#[test]
fn oversized_request_line_is_rejected() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    std::thread::spawn(move || {
        server::run_test(listener);
    });

    let mut stream = std::net::TcpStream::connect(addr).unwrap();

    let long_path = "A".repeat(2000);
    let request = format!("GET /{} HTTP/1.1\r\n\r\n", long_path);

    stream.write_all(request.as_bytes()).unwrap();

    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();

    assert!(response.starts_with("HTTP/1.1 400"));
}