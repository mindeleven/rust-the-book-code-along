/// Coding along with The Rust Programming Language Book, Chapter 16, Part 2
/// Using Message Passing to Transfer Data Between Threads
/// ensuring safe concurrency by message passing
/// -> threads (or actors) communicate by sending each other messages containing data
/// 
/// Rust's std provides an implementation of channels to accomplish message-sending concurrency
/// a channel is a general programming concept by which data is sent from one thread to another
/// 
/// a channel has two halves: a transmitter and a receiver
/// the transmitter half is for sending data 
/// -> one part of the code calls methods on the transmitter with the data you want to send
/// the receiver half is for receiving data
/// -> another part of the code checks the receiving end for arriving messages
/// channel is said to be closed if either the transmitter or receiver half is dropped
/// 
/// example: 
/// a program that has one thread to generate values and send them down a channel
/// and another thread that will receive the values and print them out
/// simple values will be sent using a channel to illustrate the feature

use std::{
    sync::mpsc, 
    thread
};

/// mpsc stands for multiple producer, single consumer
/// meaning a channel can have multiple sending ends that produce values 
/// but only one receiving end that consumes those values

fn main() {
    
    // creating a channel and assigning the two halves to tx and rx
    // mpsc::channel returns a tuple
    // the first element of which is the sending end or transmitter
    // the second element is the receiving end or receiver
    // the abbreviations tx and rx are used for transmitter and receiver respectively
    let (tx, rx) = mpsc::channel::<String>();
    
    // moving the transmitting end into a spawned thread 
    // tx is moved into the closure
    thread::spawn(move || {
        // and have it send one string 
        // so the spawned thread is communicating with the main thread
        let val = String::from("The impression that remains [...]");
        // send() returns a Result<T, E> type
        // if the receiver has already been dropped (there’s nowhere to send a value)
        // the send operation will return an error
        tx.send(val).unwrap(); // unwrap will panic in case of an error
    });
    
    // get the value from the receiver in the main thread
    // recv will block the main thread’s execution 
    // and wait until a value is sent down the channel
    let received_msg = rx.recv().unwrap();
    // when the transmitter closes recv will return an error 
    // signaling that no more values will be coming
    println!("We've received the following message: {}", received_msg);

    // another available method is try_recv 
    // try_recv() doesn’t block but will instead return a Result<T, E> immediately
    // -> an Ok value holding a message if one is available 
    // -> an Err value if there aren’t any messages this time
    // try_recv() is useful if the thread has other work to do while waiting for messages
    // -> a loop could call try_recv every() so often, 
    // -> handles a message if one is available, 
    // -> otherwise does other work for a little while until checking again

}
