/// data structures in Rust’s standard library that can contain multiple values are called collections
/// the amount of data they contain can grow or shrink as the program runs
/// types of collections:
/// (1) vector -> allows to store a variable number of values next to each other
/// (2) string -> a collection of characters
/// (3) hash map allows to associate a value with a particular key
/// 
/// Storing Lists of Values with Vectors
/// vectors, aka Vec<T>, allow to store more than one value in a single data structure
/// that puts all the values next to each other in memory
/// vectors can only store values of the same type

fn main() {
    // creating a vector
    // vectors are implemented using generics
    // type annotation added because no values are inserted
    let mut v: Vec<i32> = Vec::new();
    // updating our vector v
    v.push(100);
    v.push(101);
    v.push(102);
    v.push(103);
    v.push(104);
    println!("vector v: {:?}", v);

    // creating a Vec<T> with initial values using the vec! macro
    // Rust will infer the type of value we want to store
    let v2 = vec![1, 2, 3];
    println!("vector v2: {:?}", v2);

    // Reading Elements of Vectors
    // (1) via indexing
    // (2) using the guess method
    // accessing a value in a vector with indexing
    let third_element_of_v = &v[2];
    println!("the third element of v is {}", third_element_of_v);
    // accessing a value in a vector using the get method
    // returns an Option
    let third_elem_by_get = v.get(2);
    match third_elem_by_get {
        Some(elem) => println!("The third element by get is {elem}"),
        None => println!("There is no third element."),
    }

}
