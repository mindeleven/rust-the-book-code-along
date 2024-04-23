use std::error::Error;

fn main() {

    // Conditional if let Expressions
    // if let expressionscan be used to write the equivalent of a match 
    // that only matches one case
    // optionally, if let can have a corresponding else 

    // example: determining what color to make your background 
    // based on a series of checks for several conditions
    let favourite_color: Option<&str> = None;
    // let favourite_color: Option<&str> = Some("black");
    let is_tuesday = false;
    // let is_tuesday = true;
    let age: Result<u8, _> = "34".parse();
    let age: Result<u8, u8> = Err(1);
    
    if let Some(color) = favourite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        } 
    } else {
        println!("Something is wrong here!");
        println!("Using blue as the background color");
    }
    
}
