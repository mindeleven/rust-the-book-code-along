pub struct ThreadPool;

impl ThreadPool {
    // associated new function for ThreadPool
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
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