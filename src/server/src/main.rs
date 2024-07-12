use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::time::Duration;

fn handle_client(mut stream: TcpStream) {
    println!("Client connected: {:?}", stream.peer_addr().unwrap());
    stream.set_read_timeout(Option::from(Duration::new(3, 0))).expect("failed to set timeout");

    loop {
        let mut contents = [0; 8];
        match stream.read_exact(&mut contents) {
            Ok(_) => {
                let contents: String = contents.as_mut().iter().map(|&c| c as char).collect();
                for l in contents.lines().into_iter() {
                    if l.starts_with("GET / ") {
                        server_html(&mut stream, "./src/index.html");
                    }
                    if l.starts_with("GET /l") {
                        control("l");
                        serve_html(&mut stream, "HTTP/1.1 200 OK\r\n")
                    }
                    if l.starts_with("GET /r") {
                        control("r");
                        serve_html(&mut stream, "HTTP/1.1 200 OK\r\n")
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

fn serve_html(mut stream: &mut TcpStream, html: &str) {
    if stream.read(&mut []).is_ok() {
        match stream.write(html.as_bytes()) {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to write to stream: {}", e),
        }
    }
}

fn server_html(mut stream: &mut TcpStream, file_path: &str) {
    let contents: String = std::fs::read_to_string(file_path).expect("Failed to read file");
    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}", contents);
    serve_html(&mut stream, &response);
}