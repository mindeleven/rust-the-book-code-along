#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]

/// the goal here is to clearly show that two separate threads are talking to each other 
/// over the channel (== the code is running concurrently)

use::std::{
    sync::mpsc,
    thread,
    time::Duration
};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // cargo test tests_multiple_messages -- --nocapture
    fn tests_multiple_messages() {

        let (tx, rx) = mpsc::channel::<String>();

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
        
        // dbg!("cargo test tests_multiple_messages -- --nocapture");
    }
}