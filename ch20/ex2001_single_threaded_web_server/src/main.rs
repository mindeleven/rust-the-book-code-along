#![allow(unused_variables)]

/// Building a Single-Threaded Web Server

use std::{
    net::{
        TcpListener, 
        TcpStream
    },
    io::{
        prelude::*,
        BufReader
    }, 
};

fn main() {
    // listening to a TCP connection
    // at the local address 127.0.0.1:7878 for incoming TCP streams
    // bind will return a new TcpListener instance and returns a Result<T, E>
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    // the incoming method on TcpListener returns an iterator 
    // with a sequence of streams of type TcpStream
    // echo "TEST" | netcat 127.0.0.1 7878
    for stream in listener.incoming() {
        // single stream represents an open connection between the client and the server
        let stream = stream.unwrap();

        println!("Connection established!");
    }

    println!("The sky above the port was the color of television, tuned to a dead channel.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader 
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}