/// Shared-State Concurrency
/// handling concurrency by allowing multiple threads to access the same shared data
///
/// Shared memory concurrency is like multiple ownership: 
/// -> multiple threads can access the same memory location at the same time
/// 
/// mutexes are one of the more common concurrency primitives for shared memory
/// Mutex<T> is a smart pointer
/// Mutex is an abbreviation for mutual exclusion
/// -> a mutex allows only one thread to access some data at any given time
/// -> a thread must first signal that it wants access by asking to acquire the mutex’s lock
/// -> the lock is a data structure that keeps track of who currently has exclusive access
/// -> the mutex is described as guarding the data it holds via the locking system
/// -> when done with the data the mutex guarded data must be unlocked
///
/// Using mutexes to allow access to data from one thread at a time
use std::sync::Mutex;
use std::thread;

fn main() {
    // using a mutex in a single-threaded context
    let m = Mutex::new(5);
    println!("m = {:?} // before inner scope", m);

    {
        // we use the lock method to acquire the locked data inside the Mutex
        // call to lock() will block the current thread till it’s our turn to have the lock
        // lock() returns a LockResult<MutexGuard<'_, T>>
        // unwrap will return the value or have this thread panic
        let mut num = m.lock().unwrap();
        // can treat the return value as a mutable reference to the data inside
        // the MutexGuard smart pointer implements Deref to point at the inner data
        *num = 6;

        // the Mutex smart pointer also has a Drop implementation 
        // that releases the lock automatically when a MutexGuard goes out of scope
        // which happens with the end of this inner scope
    }
    // after dropping the lock we can print the mutex value 
    // and see that we actually could change the inner i32 to 6
    println!("m = {:?} // after inner scope", m);

    // sharing a Mutex<T> between multiple threads
    // spinning up 10 threads and have them each increment a counter value by 1
    // so the counter goes from 0 to 10
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 1..10 {
        // we give all the threads the same closure that moves the counter into the thread
        let handle = thread::spawn(move || {
            // the closure acquires a lock on the Mutex<T> 
            let mut num = counter.lock().unwrap();
            // then adds 1 to the value in the mutex
            *num += 1;
            // once a thread finishes running its closure num will go out of scope
            // and release the lock
        });
        // collecting all the join handles
        handles.push(handle);
    }
    // calling join on each handle to make sure all the threads finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
