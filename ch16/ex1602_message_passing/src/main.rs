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

fn main() {
    
    // creating a channel and assigning the two halves to tx and rx
    let (tx, rx) = mpsc::channel::<String>();
/* 
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
*/
}
