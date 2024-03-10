fn main() {
    println!("------------------------------------------------------------");
    println!("A first attempt without using slices");
    println!("------------------------------------------------------------");

    // programming problem: write a function that takes a string of words separated by spaces 
    // and returns the first word it finds in that string
    // if there isn't found a space in the string the entire string should be returned
    let mut s = String::from("This is the way the world ends");
    println!("first space in string is at {:?}", find_first_word(&s));

    s.clear(); // this emptying the String (making it equal to "")
    
    // if we rely on the index to extract the first word we can't do it anymore
    // s has still the value 5 but the contents of s have changed

    println!("------------------------------------------------------------");
    println!("Making things better with string slices");
    println!("------------------------------------------------------------");
    let s1 = String::from("Not with a bang but a whimper");
    // a string slice is a reference to part of a String
    let bang = &s1[11..15];
    let whimper = &s1[22..];
    // alternatively 
    // let len = s1.len();
    // let whimper = &s1[22..len];
    let not = &s1[..3];
    let all_of_it = &s1[..];
    println!("{} {} but {}", not, bang, whimper);
    println!("{}", all_of_it);
    let s1 = String::from("This is the way the world ends");
    println!("first word in the string is {:?}", find_first_slice(&s1));

}

// function for a second attempt using slices
// return type that signifies string slice is written as &str
fn find_first_slice(s: &String) -> &str {
    // same procedure as with first attempt below
    // to get the index for the end of the word
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // this time we return a string slice 
            // using the start of the string and the index of the space 
           return &s[0..i];
        }
    }
    &s[..]
}

// function for a first attempt without using slices
// returning the index of the end of the word indicated by a space
fn find_first_word(s: &String) -> usize {
    // to go through the String element by element it's converted
    // to an array of bytes
    let bytes = s.as_bytes();
    // creating an iterator over the array
    for (i, &item) in bytes.iter().enumerate() {
        // println!("{}, {}", i, item);
        // enumerate returns a tuple which gets destructured by a pattern
        if item == b' ' {
           // if we find a space we return the position
           return i;
        }
    }
    // otherwise we return the length of the string
    s.len()
}