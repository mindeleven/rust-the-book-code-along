/// errors are grouped into two major categories: recoverable and unrecoverable errors
/// (1) recoverable error -> they allow us to report the problem to the user and retry the operation
/// (2) unrecoverable errors are always symptoms of bugs that make it necessary stop the program
/// Rust has the type Result<T, E> for recoverable errors 
/// and the panic! macro to stop execution in case of an unrecoverable error
use std::{
    fs::File,
    io::{self, ErrorKind, Read}
};

fn main() {
    // a simple call to panic
    // panic!("crash and burn");

    // Using a panic! Backtrace
    let _v = vec![1, 2, 3]; 
    // trying to access an index in a vector beyond the range of valid indexes
    // _v[66]; // thread 'main' panicked at ...
    // compiler error:
    /* 
    thread 'main' panicked at src/main.rs:14:6:
    index out of bounds: the len is 3 but the index is 66
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    */
    // -> we can set the RUST_BACKTRACE environment variable 
    // to get a backtrace of exactly what happened to cause the error
    // RUST_BACKTRACE=1 cargo run
    
    // Recoverable Errors with Result
    // the Result enum is defined as having two variants, Ok and Err
    // T and E are generic type parameters
    /* 
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    */
    // example: trying to open a file that doesn't exist
    // the return type of File::open is a Result<T, E>
    // generic parameter T gets filled in by the implementation of File::open 
    // with the type of the success value which is a file handle
    // the type of E used in the error value is std::io::Error
    let greeting_file_result = File::open("hello.txt");
    // handling the Result with the match expression 
    // that allows us take different actions depending on the value
    /* 
    let _greeting_file: File = match greeting_file_result {
        Ok(file_handle) => file_handle,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    */

    // Matching on Different Errors
    // let greeting_file_result = File::open("hello.txt");
    // different actions for different failure reasons
    // if File::open failed because the file doesn’t exist -> create the file and return the handle
    // if File::open failed for any other reason -> panic!
    let _greeting_file = match greeting_file_result {
        Ok(file_handle) => file_handle,
        // File::open returns io::Error, a struct provided by the standard library
        // struct has a method kind that we can call to get an io::ErrorKind
        Err(error) => match error.kind() {
            // enum io::ErrorKind is provided by the standard library 
            // and has variants representing the different kinds of errors
            // ErrorKind::NotFound indicates the file we’re trying to open doesn’t exist yet
            ErrorKind::NotFound => match File::create("hello.txt") {
                // File::create might fail and returns an io::Result
                Ok(file_handle) => file_handle,
                Err(error) => panic!("creating opening the file: {:?}", error),
            },
            // program panics on any error besides the missing file error
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            },
        }
    };

    // using closures and the unwrap_or_else method as an alternatives match with Result
    let _greeting_file2 = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound { 
            File::create("hello2.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    
    // Shortcuts for Panic on Error: unwrap and expect
    // the unwrap method is a shortcut method implemented just like the match expression
    // if the Result value is the Ok variant unwrap will return the value inside the Ok
    // if the Result is the Err variant unwrap will call the panic! macro
    let _greeting_file3 = File::open("hello2.txt").unwrap();
    // using expect instead of unwrap let's us provide a good error message
    let _greeting_file4 = File::open("hello2.txt")
        .expect("Couldn't find hello3.txt in this project!");
    
    // Propagating Errors
    // propagating the error -> instead of handling the error within the function itself
    // the error is returned to the calling code
    let username = match read_username_from_file() {
        Ok(username) => username,
        Err(e) => panic!("Problem retrieving the username: {:?}", e),
    };

    println!("Hello, {}!", username);

}

// Propagating Errors
// function is returning a value of the type Result<T, E>
// parameter T has been filled in with the concrete type String
// generic type E has been filled in with the concrete type io::Error
// if function succeeds it returns an Ok value that holds the username as a String
// if function encounters problems the calling code will receive an Err value
fn read_username_from_file() -> Result<String, io::Error> {
    let username = File::open("hello.txt");
    // if File::open succeeds the file handle becomes the value in the mutable 
    // variable username_file and the function continues
    let mut username_file = match username {
        Ok(file_handle) => file_handle,
        // in the Err case we return early with the return keyword
        // the error value from File::open is passed back to the calling code
        Err(error) => return Err(error),
    };

    let mut username = String::new();
    
    // read_to_string method might fail so it returns another Result
    // if read_to_string succeeds the function returns the username wrapped in an Ok
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        // if read_to_string fails we return the error value like above
        Err(e) => Err(e),
    }
}
