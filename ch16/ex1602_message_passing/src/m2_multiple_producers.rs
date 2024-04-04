#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]

/// Sending multiple messages from multiple producers
/// mpsc is an acronym for multiple producer, single consumer
/// the goal here is to expand the code from m1_multiple_messages.rs 
/// to create multiple threads that all send values to the same receiver

use::std::{
    sync::mpsc,
    thread,
    time::Duration
};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // cargo test tests_multiple_producers -- --nocapture
    fn tests_multiple_producers() {

        let (tx, rx) = mpsc::channel::<String>();

        // creating multiple threads by cloning the transmitter
        let tx1 = tx.clone();
        
        // 1st subthread
        thread::spawn(move || {
            // the spawned thread has a vector of strings 
            // that we want to send to the main thread
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                // pausing between each calling with thread::sleep
                thread::sleep(Duration::from_millis(1));
            }
        });
        
        // 2nd subthread
        thread::spawn(move || {
            // the spawned thread has a vector of strings 
            // that we want to send to the main thread
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                // pausing between each calling with thread::sleep
                thread::sleep(Duration::from_millis(1));
            }
        });
        
        // the recv() function is not called explicitly anymore
        // instead rx is treated as an iterator. 
        for received_msg in rx {
            println!("We've received the following message: {}", received_msg);
        } // when the channel is closed iteration will end
        
    }
}