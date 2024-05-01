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

        // println!("Connection established!");

        handle_connection(stream);
    }

    println!("The sky above the port was the color of television, tuned to a dead channel.");
}

fn handle_connection(mut stream: TcpStream) {
    // creating a new BufReader instance that wraps a mutable reference to the stream
    // BufReader adds buffering by managing calls to the std::io::Read trait
    let buf_reader = BufReader::new(&mut stream);
    // collecting the lines of the request the browser sends to our server
    let http_request: Vec<_> = buf_reader 
        // the std::io::Read trait provides the lines method
        // lines() returns an iterator of Result<String, std::io::Error> 
        .lines()
        // to get each String, we map and unwrap each Result
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // println!("Request: {:#?}", http_request);

    // writing a tiny successful HTTP response to the stream
    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    // as_bytes() converts the string data to bytes
    // write_all() takes a &[u8] and sends those bytes directly down the connection

    // Sending the contents of hello.html as the body of the response
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("files/hello.html").unwrap();
    // adding the Content-Length header and setting it to the size of our response body
    // ensures a valid HTTP response
    let length = contents.len();
    // we use format! to add the file’s contents as the body of the success response
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

}

// rewriting the handle_connection functionality to handle different requests (by uri)
fn handle_connection_2(mut stream: TcpStream) {
    // checking that the browser is requesting / before returning the HTML file
    // returning an error if the browser requests anything else
    
    let buf_reader = BufReader::new(&mut stream);
    // we only want the first line of the HTTP request 
    // so we’re calling next to get the first item from the iterator
    // unwrap() no. 1 takes care of the Option and stops the program if the iterator has no items
    // unwrap() no. 2 handles the Result
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    // checking request_line to see 
    // if it equals the request line of a GET request to the / path
    if request_line == "GET / HTTP/1.1" {
        // in this case all like above
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("files/hello.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();

    } else {
        // do something else
    }

}