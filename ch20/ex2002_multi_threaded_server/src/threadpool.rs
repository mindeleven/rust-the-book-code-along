use std::thread;

pub struct ThreadPool {
    // ThreadPool to hold a vector of thread::JoinHandle<()> instances
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        // checking that size is greater than zero 
        // and have the program panic if it receives a zero by using the assert! macro
        assert!(size > 0);

        // initializing the vector with a capacity of size
        let mut threads = Vec::with_capacity(size);

        // setting up a for loop that will run some code to create the threads
        for _ in 0..size {
            // create some threads and store them in the vector

        }

        // returned a ThreadPool instance containing the threads
        ThreadPool { threads }
    }
    // pool.execute needs to be implemented in a way that it takes the closure 
    // and gives it to a thread in the pool to run
    // we can take closures as parameters with three different traits: Fn, FnMut, and FnOnce
    // the F type parameter is the one we’re concerned with here
    // the F type parameter also has the trait bound Send and the lifetime bound 'static
    // we need Send to transfer the closure from one thread to another 
    // we need 'static because we don’t know how long the thread will take to execute
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}