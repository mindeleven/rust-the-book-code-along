/// when to call panic! and when to return Result?
/// panic! means there’s no way of recovering
/// returning a Result value gives the calling code options
/// the calling code could choose to attempt to recover in a way that’s appropriate

use rand::Rng;
use std::{
    cmp::Ordering, 
    io
};

fn main() {
    // Creating Custom Types for Validation
    // let's go back to the code of the guessing game from chapter two

    // so far the user’s guesses don't get validated
    // it would be a useful enhancement to guide the user toward valid guesses 
    // and have different behavior when a user guesses a number that’s out of range 
    // versus when a user types, for example, letters instead

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    println!("It's a guessing game!");
    loop {
        println!("Please enter a number:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Something went wrong");
        
        // parsing the guess as an i32 instead of only a u32 
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // then add a check for the number being in range
        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        println!("Your input is: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You guessed right");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
}
