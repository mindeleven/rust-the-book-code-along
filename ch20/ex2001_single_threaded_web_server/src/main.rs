/// Building a Single-Threaded Web Server

use std::net::TcpListener;

fn main() {
    // listening to a TCP connection
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }

    println!("The sky above the port was the color of television, tuned to a dead channel.");
}
