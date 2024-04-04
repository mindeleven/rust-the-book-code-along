#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]

/// within a program you can have independent parts that run simultaneously
/// the features that run these independent parts are called threads
/// example: a web server could have multiple threads 
/// so that it could respond to more than one request at the same time
/// 
/// problems that can arise from splitting your program into multiple threads
/// -> race conditions: threads are accessing data or resources in an inconsistent order
/// -> deadlocks: two threads are waiting for each other (preventing both from continuing)
/// -> bugs that happen only in certain situations and are hard to reproduce

use::std::{
    thread,
    time::Duration
};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // cargo test tests_threads -- --nocapture
    fn tests_threads() {
        // of the threads will take turns depends on 
        // how the operating system schedules the threads
        // when the main thread of a Rust program completes all spawned threads are shut down

        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the SPAWNED thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            // calls to thread::sleep force a thread to stop its execution for a short duration
            thread::sleep(Duration::from_millis(1));
        }

        // dbg!("cargo test tests_threads -- --nocapture");
    }
}