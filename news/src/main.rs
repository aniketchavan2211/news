use news::config;
use news::server;
// use bulletin_board::http;

fn main() {
    // Fail fast if static content is missing
    if !std::path::Path::new(config::INDEX_PATH).exists() {
        eprintln!(
            "FATAL: index.html not found at {}",
            config::INDEX_PATH
        );
        std::process::exit(1);
    }

    println!("Starting News Service …");
    server::run();
}