use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::time::Duration;

fn handle_client(mut stream: TcpStream) {
    println!("Client connected: {:?}", stream.peer_addr().unwrap());

    let mut contents = [0; 8];
    stream.read_exact(&mut contents).expect("Failed to read from stream");
    let contents: String = contents.as_mut().iter().map(|&c| c as char).collect();
    println!("Received:\n{}", contents);

    for l in contents.lines().into_iter() {
        if l.starts_with("GET /l") {
            std::process::Command::new("sh")
                .arg("-c")
                .arg("../control.sh l")
                .spawn()
                .expect("Failed to execute command");
        }
        if l.starts_with("GET /r") {
            std::process::Command::new("sh")
                .arg("-c")
                .arg("../control.sh r")
                .spawn()
                .expect("Failed to execute command");
        }
        stream.set_read_timeout(Option::from(Duration::new(3, 0))).expect("failed to set timeout");
        if stream.read(&mut []).is_ok() {
            stream.write(b"HTTP/1.1 200 OK\r\n").expect("Failed to write to stream");
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