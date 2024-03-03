/// Coding along with The Book 
/// The Rust Programming Language
/// Chapter 2, Programming a Guessing Game
/// Source code and comments are based on The Book

use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().get_range(1..=100);
    
    println("The secret number is {}", secret_number);
    
    println!("It's a guessing game!");

    println!("Please enter a number:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Something went wrong");

    println!("Your input is: {}", guess);
}
