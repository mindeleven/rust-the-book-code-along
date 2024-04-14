/// Reference cycles can leak memory
/// Rust allows memory leaks by using Rc<T> and RefCell<T>:
/// -> it’s possible to create references where items refer to each other in a cycle
/// the reference count of each item in the cycle will never reach 0
/// and the values will never be dropped.
/// -> memory leaks are created

fn main() {
    println!("Hello, world!");
}
