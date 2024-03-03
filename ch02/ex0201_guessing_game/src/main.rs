/// Coding along with The Book 
/// The Rust Programming Language
/// Chapter 2, Programming a Guessing Game
/// Source code and comments are based on The Book

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("The secret number is {}", secret_number);
    
    println!("It's a guessing game!");

    println!("Please enter a number:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Something went wrong");
        
    let guess: u32 = guess
        .trim()
        .parse()
        .expect("Please type a number");

    println!("Your input is: {}", guess);
    
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small"),
      Ordering::Equal => println!("You guessed right"),
      Ordering::Greater => println!("Too big")
    }
}
