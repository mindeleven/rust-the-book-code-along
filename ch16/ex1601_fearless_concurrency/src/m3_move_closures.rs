#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]

use::std::thread;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // cargo test tests_move_closures -- --nocapture
    fn tests_move_closures() {

        let v = vec![1, 2, 3];
        
        // attempting to use a vector created by the main thread in another thread
        // without move the spawned thread’s closure must capture (borrow) the values it needs
        // problem: Rust can’t tell how long the spawned thread will run
        // so it doesn’t know if the reference to v will always be valid
        // adding the move keyword before the closure forces the closure to take ownership
        let handle = thread::spawn(move || {
            println!("we can get v in here: {:?}", v);
        });
        
        handle.join().unwrap();
    }
}