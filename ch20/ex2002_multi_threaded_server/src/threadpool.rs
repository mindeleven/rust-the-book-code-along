pub struct ThreadPool;

impl ThreadPool {
    // associated new function for ThreadPool
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}