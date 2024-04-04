#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]

use::std::{
    thread,
    time::Duration
};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // cargo test tests_threads_join_handles -- --nocapture
    fn tests_threads_join_handles() {
        // of the threads will take turns depends on 
        // how the operating system schedules the threads
        // when the main thread of a Rust program completes all spawned threads are shut down
        // the problem that the spawned thread shuts down prematurely 
        // can be sloved by saving the return value of thread::spawn in a variable

        // the return type of thread::spawn is JoinHandle
        // a JoinHandle is an owned value that will wait for its thread to finish
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the SPAWNED thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
        
        // putting handle.join() before the code that follows the spawned thread
        // will cause the main thread to wait for the spawned thread to finish
        // and then continue (that is, run its for loop)
        handle.join().unwrap();

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            // calls to thread::sleep force a thread to stop its execution for a short duration
            thread::sleep(Duration::from_millis(1));
        }
        
        // calling join on the handle blocks the thread currently running 
        // until the thread represented by the handle terminates
        // main thread waits and does not end until the spawned thread is finished
        // handle.join().unwrap();

    }
}