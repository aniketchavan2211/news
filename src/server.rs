use std::net::{TcpListener, TcpStream};
use std::time::Duration;

use crate::config;
use crate::http;

pub fn run() {
    let listener = TcpListener::bind(config::BIND_ADDR)
        .expect("Failed to bind address");

    println!("Listening on {}", config::BIND_ADDR);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_client(stream);
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    // Protect against slow clients
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(5)));

    http::handle_request(&mut stream);
}

/* ===== Test support (integration tests) ===== */

#[cfg(feature = "test-support")]
pub fn run_test(listener: TcpListener) {
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            http::handle_request(&mut stream);
            break;
        }
    }
}
