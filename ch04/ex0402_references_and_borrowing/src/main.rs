fn main() {
    println!("------------------------------------------------------------");
    println!("References and Borrowing");
    println!("------------------------------------------------------------");
    // a reference is like a pointer in that you can follow to access the data stored at that address
    // unlike with a pointer the  data is owned by some other variable
    // a reference is guaranteed to point to a valid value of a particular type 
    // for the life of that reference

    // the opposite of referencing by using & is dereferencing
    // with the dereference operator *

    let s1 = String::from("The bluest sky");
    // the argument passed to the function is a reference to s1
    // the &s1 syntax creates a reference that refers to the value of s1 but does not own it
    let s2 = calculate_length(&s1);

    println!("The string \"{}\" has a length of {}.", s1, s2);
}

// function that has a reference to an object as a parameter 
// and is not taking ownership of the value
fn calculate_length(s: &String) -> usize {
    s.len()
}