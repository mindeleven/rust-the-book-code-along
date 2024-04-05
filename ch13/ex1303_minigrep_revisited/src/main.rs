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
    // cargo run -- searchstring data/poem.txt
    // reading command line arguments
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    // saving the argument values in variables
    // first value args[0] contains the program's/binary's name
    // arguments start at index 1
    // first argument at args[1] contains the query
    // second argument at args[2] contains the file path
    // unwrap_or_else allows us to define some custom, non-panic! error handling
    // if the Result is an Ok value the inner value that Ok is wrapping gets returned
    // if the value is an Err value, this method calls the code in the closure
    let config = Config::build(&args).unwrap_or_else(|err| {
        // standard library provides the eprintln! macro that prints to the standard error stream
        eprintln!("Problem parsing arguments: {}", err);
        // the process::exit function will stop the program immediately 
        // and return the number that was passed as the exit status code
        process::exit(1);
    });

    // let's print out what we got
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    //  use if let rather than unwrap_or_else to check whether run returns an Err value
    if let Err(e) = ex1201_minigrep::run(config) {
        // standard library provides the eprintln! macro that prints to the standard error stream
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
