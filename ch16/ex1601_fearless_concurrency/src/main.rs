/// concurrent programming, 
/// -> where different parts of a program execute independently
/// parallel programming
/// -> where different parts of a program execute at the same time
/// 
/// topics to be covered:
/// -> how to create threads to run multiple pieces of code at the same time
/// -> message-passing concurrency, where channels send messages between threads
/// -> shared-state concurrency, where multiple threads have access to some piece of data
/// -> the Sync and Send traits, 
///    which extend Rust’s concurrency guarantees to user-defined types 
///    as well as types provided by the standard library

mod m1_threads;

fn main() {
    println!("Coding along with The Rust Programming Language Book, Chapter 16");
    println!("Fearless Concurrency")
}
