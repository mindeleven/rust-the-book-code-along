/// An I/O Project: Building a Command Line Program
/// Coding along with chapter 12 of the "The Rust Programming Language" book
/// 
/// goal: building a command line tool that interacts with file and command line input/output
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
/// cargo run -- searchstring poem.txt
/// std::fs is needed to handle files
use std::{env, fs};

fn main() {
    // reading command line arguments
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    // saving the argument values in variables
    // first value args[0] contains the program's/binary's name
    // arguments start at index 1
    /* 
    // first argument at args[1] contains the query
    let query = &args[1];
    // second argument at args[2] contains the file path
    let file_path = &args[2];
    */
    let config = parse_config(&args);
    // let's print out what we got
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // fs::read_to_string takes the file_path
    // opens that file
    // and returns a std::io::Result<String> of the file’s contents
    let contents = fs::read_to_string(config.file_path)
        .expect("Couldn't read the file from the path you provided.");

    println!("Text read from file:\n{}", contents);
}

fn parse_config(args: &[String]) -> Config {
    // Config is defined to contain owned String values
    // the args variable in main is the owner of the argument 
    // parse_config() only borrows them
    // the clone method is called on the values to make a full copy of the data 
    // this full copy is for the Config instance to own
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}

struct Config {
    query: String,
    file_path: String,
}