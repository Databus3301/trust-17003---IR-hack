use std::fmt::format;
use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::time::Duration;

fn handle_client(mut stream: TcpStream) {
    println!("Client connected: {:?}", stream.peer_addr().unwrap());

    loop {
        let mut contents = [0; 8];
        match stream.read_exact(&mut contents) {
            Ok(_) => {
                let contents: String = contents.as_mut().iter().map(|&c| c as char).collect();
                for l in contents.lines().into_iter() {
                    if l.starts_with("GET /l") {
                        control("l");
                    }
                    if l.starts_with("GET /r") {
                        control("r");
                    }
                    stream.set_read_timeout(Option::from(Duration::new(3, 0))).expect("failed to set timeout");
                    if stream.read(&mut []).is_ok() {
                        match stream.write(b"HTTP/1.1 200 OK\r\n") {
                            Ok(_) => (),
                            Err(e) => eprintln!("Failed to write to stream: {}", e),
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
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
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

fn control(sequence: &str) {
    std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("../control.sh {}", sequence))
        .spawn()
        .expect("Failed to execute command");
}