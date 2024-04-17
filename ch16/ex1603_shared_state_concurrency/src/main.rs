/// Shared-State Concurrency
/// handling concurrency by allowing multiple threads to access the same shared data
///
/// Shared memory concurrency is like multiple ownership: 
/// -> multiple threads can access the same memory location at the same time
/// 
/// mutexes are one of the more common concurrency primitives for shared memory
/// Mutex is an abbreviation for mutual exclusion
/// -> a mutex allows only one thread to access some data at any given time
/// -> a thread must first signal that it wants access by asking to acquire the mutex’s lock
/// -> the lock is a data structure that keeps track of who currently has exclusive access
/// -> the mutex is described as guarding the data it holds via the locking system
/// -> when done with the data the mutex guarded data must be unlocked
///
/// Using mutexes to allow access to data from one thread at a time
use std::sync::Mutex;

fn main() {
    // using a mutex in a single-threaded context
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
