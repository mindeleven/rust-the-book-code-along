/// when to call panic! and when to return Result?
/// panic! means there’s no way of recovering
/// returning a Result value gives the calling code options
/// the calling code could choose to attempt to recover in a way that’s appropriate

fn main() {
    println!("Hello, world!");
}
