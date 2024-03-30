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
/// cargo run -- searchstring data/poem.txt
/// std::fs is needed to handle files
use std::{
    env, 
    error::Error, 
    fs, 
    process
};

fn main() {
    // cargo run -- searchstring data/poem.txt
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
    // let _config = _parse_config(&args);
    // let _config = Config::new(&args);
    // unwrap_or_else allows us to define some custom, non-panic! error handling
    // if the Result is an Ok value the inner value that Ok is wrapping gets returned
    // if the value is an Err value, this method calls the code in the closure
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        // the process::exit function will stop the program immediately 
        // and return the number that was passed as the exit status code
        process::exit(1);
    });

    // let's print out what we got
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // fs::read_to_string takes the file_path
    // opens that file
    // and returns a std::io::Result<String> of the file’s contents
    /* 
    let contents = fs::read_to_string(config.file_path)
        .expect("Couldn't read the file from the path you provided.");

    println!("Text read from file:\n{}", contents);
    */
    //  use if let rather than unwrap_or_else to check whether run returns an Err value
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}

// extracting logic from main
// return type of the run function is Result<(), Box<dyn Error>>
// unit type () is returned in case of Ok case
// in error case the trait object Box<dyn Error> is returned
// Box<dyn Error> means the function will return a type that implements the Error trait
// allows us to return error values that may be of different type
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // let contents = fs::read_to_string(config.file_path)
    //    .expect("Couldn't read the file from the path you provided.");
    // the ? operator will return the error value from the current function 
    // for the caller to handle
    let contents = fs::read_to_string(config.file_path)?;

    println!("Text read from file:\n{}", contents);

    Ok(())
}

/* 
fn _parse_config(args: &[String]) -> Config {
    // Config is defined to contain owned String values
    // the args variable in main is the owner of the argument 
    // parse_config() only borrows them
    // the clone method is called on the values to make a full copy of the data 
    // this full copy is for the Config instance to own
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
*/
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    /* 
    // logic from the parse_config() functionality gets moved into a new constructor
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Config { query, file_path }
    } */
    // replacing new() with build() that returns a Result
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // returning Error variant in case of not enough arguments
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        // returning Ok with a Config wrapped inside
        Ok(Config { query, file_path })
    }
}