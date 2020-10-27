use std::io;
use std::net::TcpListener;
use std::env;

use rust_http_server;

extern crate chrono;


fn main() -> io::Result<()> {
    println!("Starting Server...");
    let current_cwd = env::current_dir().unwrap();
    let current_cwd = current_cwd.to_str().unwrap();

    let listener = TcpListener::bind("0.0.0.0:8080")?;
    let static_root: &str = &(current_cwd.to_owned() + "/src/static_root");

    println!("Server listening on port 8080!");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                match rust_http_server::handle_client(stream, &static_root) {
                    Err(e) => eprintln!("Error handling client: {}", e),
                    _ => (),
                }
            },
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}
