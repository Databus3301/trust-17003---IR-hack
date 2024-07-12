use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::time::Duration;

fn handle_client(mut stream: TcpStream) {
    println!("Client connected: {:?}", stream.peer_addr().unwrap());
    stream.set_read_timeout(Option::from(Duration::new(3, 0))).expect("failed to set timeout");

    loop {
        let mut contents = [0; 1024];
        match stream.read(&mut contents) {
            Ok(_) => {
                let contents: String = contents.as_mut().iter().map(|&c| c as char).collect();
                if contents.starts_with("\0") {
                    break;
                }
                for l in contents.lines().into_iter() {
                    println!("Line: {:x?}", l);
                    if l.starts_with("GET / ") {
                        serve_html_file(&mut stream, "./src/index.html");
                        capture_image();
                    }
                    if l.starts_with("GET /l") {
                        control("l");
                        serve_html_file(&mut stream, "./src/redirect.html");
                    }
                    if l.starts_with("GET /r") {
                        control("r");
                        serve_html_file(&mut stream, "./src/redirect.html");
                    }
                    if l.starts_with("GET /frame_0005.jpg") {
                        println!("serving image");
                        serve_image_file(&mut stream, "./res/frame_0005.jpg");
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
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind");
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
    std::process::Command::new("../control.sh")
        .arg(sequence)
        .spawn()
        .expect("Failed to execute command");
}

fn capture_image() {
    // capture image
    println!("Capturing video");
    let mut cmd = std::process::Command::new("../capture.sh")
        .spawn()
        .expect("Failed to execute command");
    // w8 for the command to finish
    let _ = cmd.wait().expect("Failed to wait for command");
    // then segment the video
    println!("Segmenting video");
    let mut cmd = std::process::Command::new("../segment.sh")
        .spawn()
        .expect("Failed to execute command");
    // w8 for the command to finish
    let _ = cmd.wait().expect("Failed to wait for command");
    // then move the last segment into res
    println!("Moving image");
    std::process::Command::new("mv")
        .arg("./segments/frame_0005.jpg")
        .arg("./res")
        .spawn()
        .expect("Failed to execute command");
}

fn serve_html(stream: &mut TcpStream, html: &str) {
    if stream.read(&mut []).is_ok() {
        match stream.write(html.as_bytes()) {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to write to stream: {}", e),
        }
    }
}

fn serve_html_file(mut stream: &mut TcpStream, file_path: &str) {
    let contents: String = std::fs::read_to_string(file_path).expect("Failed to read file");
    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}", contents);
    serve_html(&mut stream, &response);
}

fn serve_image_file(mut stream: &mut TcpStream, file_path: &str) {
    let contents: Vec<u8> = std::fs::read(file_path).expect("Failed to read file");
    let response = "HTTP/1.1 200 OK\r\nContent-Type: image/jpeg\r\n\r\n";
    serve_html(&mut stream, response);
    match stream.write(&contents) {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to write to stream: {}", e),
    }
}