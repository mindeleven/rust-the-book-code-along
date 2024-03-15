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
    // using & and [] gives us a reference to the element at the index value
    let third_element_of_v = &v[2];
    println!("the third element of v is {}", third_element_of_v);
    // accessing a value in a vector using the get method
    // returns an Option<T>
    let third_elem_by_get = v.get(2);
    match third_elem_by_get {
        Some(elem) => println!("The third element by get is {elem}"),
        None => println!("There is no third element."),
    }
    // useful if you try to use an index value outside the range of existing elements
    let tenth_elem_by_get = v.get(10);
    match tenth_elem_by_get {
        Some(elem) => println!("The 10th element by get is {elem}"),
        None => println!("There is no 10th element."),
    }

    // when the program has a valid reference
    // the borrow checker enforces the ownership and borrowing rules
    let _fourth_element = &v[3]; // immutable borrow occurs here
    // attempting to add an element to a vector while holding a reference to an item
    v.push(109); // mutable borrow occurs here
    // compiler error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    // println!("the 4th element is {}", _fourth_element); // immutable borrow later used here
    // the rule states you can’t have mutable and immutable references in the same scope
    // we hold an immutable reference to the fourth element in a vector 
    // and try to add an element to the end
    // this program won’t work if we also try to refer to that element later in the function

    // Iterating over the Values in a Vector
    let v2 = vec![23, 45, 56, 77, 83, 91, 100, 101, 153];
    // iterating through all the elements to access each element in a vector in turn
    for elem in &v2 {
        println!("{}", elem);
    }

    // iterating over mutable references in a mutable vector 
    // and making changes to all the elements
    let mut v3 = vec![233, 435, 356, 772, 833, 911, 110, 161, 153];
    for elem in &mut v3 {
        // change the value that the mutable reference refers to
        // the value in elem needs to be dereferenced with the * dereference operator
        *elem = *elem*2;
    }
    println!("v3 afterwards: {:?}", v3);

    // Using an Enum to Store Multiple Types
    // workaround for storing a list of items of different types
    // when one type is neede that represents elements of different types an enum can be used
    #[allow(dead_code)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    // now defining a row of the spreadsheet
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(6.89),
        SpreadsheetCell::Text(String::from("some string"))
    ];
    
    // Dropping a Vector Drops Its Elements
    {
        let _v8 = vec![233, 435, 356];
    } // v8 goes out of scope and is freed here
}
