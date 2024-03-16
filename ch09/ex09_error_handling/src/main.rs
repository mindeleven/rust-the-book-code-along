/// errors are grouped into two major categories: recoverable and unrecoverable errors
/// (1) recoverable error -> they allow us to report the problem to the user and retry the operation
/// (2) unrecoverable errors are always symptoms of bugs that make it necessary stop the program
/// Rust has the type Result<T, E> for recoverable errors 
/// and the panic! macro to stop execution in case of an unrecoverable error
use std::fs::File;

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
    let _greeting_file = match greeting_file_result {
        Ok(file_handle) => file_handle,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    
}
