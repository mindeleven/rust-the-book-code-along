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

        handle_connection_slow(stream);
    }

    println!("The sky above the port was the color of television, tuned to a dead channel.");
}

fn _handle_connection(mut stream: TcpStream) {
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
fn _handle_connection_2(mut stream: TcpStream) {
    // checking that the browser is requesting / before returning the HTML file
    // returning an error if the browser requests anything else
    
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
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
        // do something else like returning a response with the status code 404
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("files/404.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }

}

// refactoring handle_connection_2 
fn _handle_connection_3(mut stream: TcpStream) {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    // replacing the if / else blocks
    // if and else blocks now only return values for the status line and filename in a tuple
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "files/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "files/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    
}

// handling a request with a simulated slow response
fn handle_connection_slow(mut stream: TcpStream) { 
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    // switching from if to match
    // request_line needs to be turnes into a slice explicitly to pattern match the string values
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "files/hello.html"),
        "GET /something_else HTTP/1.1" => {
            // handling a request to /something_else with a simulated slow response
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "files/something_else.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "files/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

}
