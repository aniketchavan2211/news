// Network
pub const BIND_ADDR: &str = "127.0.0.1:8080";

// Filesystem
pub const INDEX_PATH: &str = "/opt/news/static/index.html";

// Global request limits
pub const MAX_REQUEST_SIZE: usize = 8 * 1024; // 8 KB total
pub const MAX_REQUEST_LINE_LEN: usize = 1024;
pub const MAX_HEADER_LINES: usize = 32;
pub const MAX_HEADER_LINE_LEN: usize = 1024;