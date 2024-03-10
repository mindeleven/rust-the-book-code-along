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