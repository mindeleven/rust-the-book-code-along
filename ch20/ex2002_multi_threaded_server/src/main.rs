#![allow(unused_variables)]

/// Building a Single-Threaded Web Server

use std::{
    fs,
    net::{
        TcpListener, 
        TcpStream
    },
    // std::io prelude and BufReader gives us access to traits and types 
    // that let us read from and write to the stream
    io::{
        prelude::*,
        BufReader
    },
    thread,
    time::Duration
};

mod threadpool;
use threadpool::ThreadPool;

fn main() {
    // listening to a TCP connection
    // at the local address 127.0.0.1:7878 for incoming TCP streams
    // bind will return a new TcpListener instance and returns a Result<T, E>
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // creating a thread pool with a configurable number of threads
    let pool = ThreadPool::new(4);
    
    // the incoming method on TcpListener returns an iterator 
    // with a sequence of streams of type TcpStream
    // echo "TEST" | netcat 127.0.0.1 7878
    for stream in listener.incoming() {
        // single stream represents an open connection between the client and the server
        let stream = stream.unwrap();
        
        // spawning a new thread for each stream
        // thread::spawn(|| {
        // using the threadpool instead
        pool.execute(|| {
            // running the code in the closure in the new thread
            handle_connection(stream);
        });
        
    }

    println!("The sky above the port was the color of television, tuned to a dead channel.");
}

// handling a request with a simulated slow response
fn handle_connection(mut stream: TcpStream) { 
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "files/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "files/sleep.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "files/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

}
