use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write, Read};

fn handle_client(mut stream: TcpStream) {
    println!("Client connected: {:?}", stream.peer_addr().unwrap());

    let mut contents: String = Default::default();
    {
        let mut reader = BufReader::new(&stream);
        reader.read_to_string(&mut contents).expect("Failed to read from stream");
    }

    for (i, l) in contents.lines().enumerate() {
        if l.starts_with("GET /l") {
            println!("Received:\n{}", contents);
            std::process::Command::new("sh")
                .arg("-c")
                .arg("../control.sh l")
                .spawn()
                .expect("Failed to execute command");
        }
        if l.starts_with("GET /r") {
            println!("Received: {}", contents);
            std::process::Command::new("sh")
                .arg("-c")
                .arg("../control.sh r")
                .spawn()
                .expect("Failed to execute command");
        }
        stream.write(b"HTTP/1.1 200 OK\r\n").expect("Failed to write to stream");
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