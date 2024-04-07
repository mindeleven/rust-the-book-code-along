/// to run the binary crate from the add directory
/// we can specify which package in the workspace we want to run 
/// by using the -p argument and the package name with cargo run:
/// `$ cargo run -p adder`
use add_one;
use rand::prelude::*;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}

pub fn use_rand_to_get_f64() -> f64 {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen(); // generates a float between 0 and 1
    y
}
