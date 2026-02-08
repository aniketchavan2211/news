use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;

use crate::config;

use std::path::{Path, PathBuf};

pub fn handle_request(stream: &mut TcpStream) {
    let mut buffer = [0u8; config::MAX_REQUEST_SIZE];

    let bytes_read = match stream.read(&mut buffer) {
        Ok(0) | Err(_) => return,
        Ok(n) => n,
    };
    // Reject truncated / oversized requests
    if bytes_read == config::MAX_REQUEST_SIZE {
        respond(stream, "431 Request Header Fields Too Large", b"");
        return;
    }

    let request = match std::str::from_utf8(&buffer[..bytes_read]) {
        Ok(r) => r,
        Err(_) => {
            respond(stream, "400 Bad Request", b"");
            return;
        }
    };

    let mut lines = request.lines();

    // ---- Request line ----
    let request_line = match lines.next() {
        Some(l) if l.len() <= config::MAX_REQUEST_LINE_LEN => l,
        _ => {
            respond(stream, "400 Bad Request", b"");
            return;
        }
    };
    // Explicitly reject oversized request lines
    if request_line.len() > config::MAX_REQUEST_LINE_LEN {
        respond(stream, "400 Bad Request", b"");
        return;
    }

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");

    // ---- Headers ----
    let mut header_count = 0;

    for line in lines {
        if line.is_empty() {
            break;
        }

        header_count += 1;

        if header_count > config::MAX_HEADER_LINES
            || line.len() > config::MAX_HEADER_LINE_LEN
        {
            respond(
                stream,
                "431 Request Header Fields Too Large",
                b"",
            );
            return;
        }
    }

    // ---- Method enforcement ----
    if method != "GET" && method != "HEAD" {
        respond(stream, "405 Method Not Allowed", b"");
        return;
    }

    // ---- Path enforcement ----
    /*
    if path != "/" {
        respond(stream, "404 Not Found", b"");
        return;
    }
    */
    // Root document
    if path.len() > 256 {
      respond(stream, "400 Bad Request", b"");
      return;
    }
    if path == "/" {
        let body = match fs::read(config::INDEX_PATH) {
            Ok(b) => b,
            Err(_) => {
                respond(stream, "500 Internal Server Error", b"");
                return;
            }
        };
    
        if method == "HEAD" {
            respond(stream, "200 OK", b"");
        } else {
            respond(stream, "200 OK", &body);
        }
        return;
    }
    
    // Static allow-listed files
    if let Some(fs_path) = resolve_static_path(path) {
        let body = match fs::read(&fs_path) {
            Ok(b) => b,
            Err(_) => {
                respond(stream, "404 Not Found", b"");
                return;
            }
        };
    
        let content_type = if path.ends_with(".css") {
            "text/css"
        } else {
            "text/html"
        };
    
        let header = format!(
            "HTTP/1.1 200 OK\r\n\
             Content-Type: {}\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\
             X-Content-Type-Options: nosniff\r\n\
             X-Frame-Options: SAMEORIGIN\r\n\
             Cache-Control: no-store\r\n\
             \r\n",
            content_type,
            body.len()
        );
    
        let _ = stream.write_all(header.as_bytes());
        let _ = stream.write_all(&body);
        return;
    }
    
    // Everything else
    respond(stream, "404 Not Found", b"");


    // ---- Serve bulletin ----
    let body = match fs::read(config::INDEX_PATH) {
        Ok(b) => b,
        Err(_) => {
            respond(stream, "500 Internal Server Error", b"");
            return;
        }
    };

    if method == "HEAD" {
        respond(stream, "200 OK", b"");
    } else {
        respond(stream, "200 OK", &body);
    }
}

fn respond(stream: &mut TcpStream, status: &str, body: &[u8]) {
    let header = format!(
        "HTTP/1.1 {}\r\n\
         Content-Type: text/html; charset=utf-8\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         X-Content-Type-Options: nosniff\r\n\
         X-Frame-Options: DENY\r\n\
         \r\n",
        status,
        body.len()
    );

    let _ = stream.write_all(header.as_bytes());
    let _ = stream.write_all(body);
}

fn resolve_static_path(path: &str) -> Option<PathBuf> {
    // Serve CSS explicitly
    if path == "/style.css" {
        return Some(PathBuf::from("/opt/news/static/style.css"));
    }

    // Serve entries only from /entries/
    if let Some(rest) = path.strip_prefix("/entries/") {
        // Reject empty or suspicious paths
        if rest.is_empty() || rest.contains("..") || rest.contains('/') {
            return None;
        }

        // Only allow .html files
        if !rest.ends_with(".html") {
            return None;
        }

        return Some(
            Path::new("/opt/news/static/entries")
                .join(rest),
        );
    }

    None
}