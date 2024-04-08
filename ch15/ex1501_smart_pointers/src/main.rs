fn main() {
    // Using a Box<T> to store data on the heap
    let b = Box::new(5);
    println!("b = {}", b);
}
