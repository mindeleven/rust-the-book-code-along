/// Improving Our I/O Project
/// Coding along with chapter 13 of the "The Rust Programming Language" book
/// 
/// goal: improving the I/O project in Chapter 12 by using iterators 
/// to make places in the code clearer and more concise
///
/// the starting point here is copying the code from the codealong with chapter 12
/// 
/// goal of chapter 12 was: 
/// building a command line tool that interacts with file and command line input/output
/// it will be a simplified version of the classic command line search tool grep 
/// grep == globally search a regular expression and print
/// simplest use case: grep searches a specified file for a specified string
/// -> (1) grep takes as its arguments a file path and a string
/// -> (2) it reads the file
/// -> (3) it finds lines in that file that contain the string argument
/// -> (4) it prints those lines
/// 
/// Accepting command line arguments
/// first task making minigrep accept two command line arguments: 
/// the file path and a string to search for
/// cargo run -- searchstring example-filename.txt
/// cargo run -- searchstring data/poem.txt
/// std::fs is needed to handle files

use std::{
    env, 
    process
};

use ex1201_minigrep::Config;

fn main() {

    // passing the Iterator itself as an argumment to build
    // the ownership of the iterator returned from env::args gets passed to Config::build directly
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        // standard library provides the eprintln! macro that prints to the standard error stream
        eprintln!("Problem parsing arguments: {}", err);
        // the process::exit function will stop the program immediately 
        // and return the number that was passed as the exit status code
        process::exit(1);
    });

    //  use if let rather than unwrap_or_else to check whether run returns an Err value
    if let Err(e) = ex1201_minigrep::run(config) {
        // standard library provides the eprintln! macro that prints to the standard error stream
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
