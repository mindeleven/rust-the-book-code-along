use std::thread;

pub struct ThreadPool {
    // ThreadPool to hold a vector of thread::JoinHandle<()> instances
    // threads: Vec<thread::JoinHandle<()>>,
    // change ThreadPool to hold a vector of Worker instances
    workers: Vec<Worker>,
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
        // the with_capacity function performs the same task as Vec::new 
        // but with an important difference: it preallocates space in the vector
        // let mut threads = Vec::with_capacity(size);

        let mut workers = Vec::with_capacity(size);

        // setting up a for loop that will run some code to create the threads
        for id in 0..size {
            // create some threads and store them in the vector
            // we want to create the threads and have them wait for code that we’ll send later
            // we'll implement the Worker data structure as a new data structure 
            // between the ThreadPool and the threads to get this behavior

            // use the for loop counter to generate an id
            // create a new Worker with that id
            // store the worker in the vector
            workers.push(Worker::new(id));
        }

        // returned a ThreadPool instance containing the threads
        // ThreadPool { threads }

        ThreadPool { workers }

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

// we want to create the threads and have them wait for code that we’ll send later
// we'll implement the Worker data structure as a new data structure 
// between the ThreadPool and the threads to get this behavior

// defining a Worker struct that holds an id and a JoinHandle<()>
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    // defining a Worker::new function that takes an id number and returns a Worker instance
    // that holds the id and a thread spawned with an empty closure

    fn new(id: usize) -> Worker {
        // spawning thread with empty closure
        let thread = thread::spawn(|| {});
        
        // returning a Worker instance that holds the id and a thread spawned with an empty closure
        Worker { id, thread }
    }
}