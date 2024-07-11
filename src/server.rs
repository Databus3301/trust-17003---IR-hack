use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};

fn handle_client(stream: TcpStream) {
    println!("Client connected: {:?}", stream.peer_addr().unwrap());

    let mut reader = BufReader::new(&stream);
    let mut buffer = String::new();

    loop {
        buffer.clear();
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected");
                break; // Connection closed
            }
            Ok(_) => {
                println!("Received: {}", buffer.trim());
                if buffer.as_str().trim().starts_with("GET /l")  {
                    println!("Received: {}", buffer.trim());
                    std::process::Command::new("sh")
                        .arg("-c")
                        .arg("./control.sh l")
                        .spawn()
                        .expect("Failed to execute command");
                }
                if buffer.as_str().trim().starts_with("GET /r") {
                    println!("Received: {}", buffer.trim());
                    std::process::Command::new("sh")
                        .arg("-c")
                        .arg("./control.sh r")
                        .spawn()
                        .expect("Failed to execute command");
                }
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread for each incoming connection
                std::thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
