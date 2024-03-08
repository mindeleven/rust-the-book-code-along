/// Ownership Rules:
/// => Each value in Rust has an owner
/// => There can only be one owner at a time
/// => When the owner goes out of scope, the value will be dropped

fn main() {
    println!("------------------------------------------------------------");
    println!("Variable Scope");
    println!("------------------------------------------------------------");
    // s scope is the range within a program for which an item is valid
    {
        let s = "hello"; // variable s refers to a string literal (immutable)
        println!("let's do something with s at least once: {}", s);
    } // this scope is now over, and s is no longer valid

    println!("------------------------------------------------------------");
    println!("The String Type");
    println!("------------------------------------------------------------");
    // to illustrate the rules of ownership it's better to look at data that's stored on the heap
    // the String type is a complex data type that manages data allocated on the heap
    // it is able to store an amount of text that is unknown to us at compile time
    // creating a String from a string literal with the from function
    let mut st = String::from("hello blue sky");
    // push_str appends a literal to a String
    st.push_str(" where the sun shines so bright");
    println!("{}", st);

    println!("------------------------------------------------------------");
    println!("Memory and Allocation");
    println!("------------------------------------------------------------");
    // to support a mutable, growable piece of text we need to allocate an amount of memory on the heap
    // the memory must be requested from the memory allocator at runtime
    // Rust returns the memory automatically to the allocator
    // once the variable that owns it goes out of scope
    // Rust calls internally the drop function
    {
        let mut sx = String::from("hello blue sky");
        sx.push_str(" where the sun shines so bright");
    } // scope is over and sx is no longer valid
    // moving data
    {
        let s1 = String::from("hello");
        // when we assign s1 to s2 the String data in the stack is copied
        // meaning we copy the pointer, the length & the capacity
        // the data on the heap that the pointer refers to is not copied
        // Rust invalidates the first variable -> s1 was moved into s2
        // in Rust this is known as a move
        let mut s2 = s1;
        s2.push_str(" , world!");
        // println!("{}, world!", s1); // compiler error: borrow of moved value: `s1`
    }
    // the method clone can be used if we want to copy the heap data
    {
        let s1 = String::from("hello from the heap");
        let s2 = s1.clone();
        println!("s1: {}, s2: {}", s1, s2)
    }

    println!("------------------------------------------------------------");
    println!("Ownership and Functions");
    println!("------------------------------------------------------------");
    let another_string = String::from("handing over ownership");
    takes_ownership(another_string); // another_string moves into the function
    let q = 42;
    makes_copy(q); // i32 is Copy so no move
    let q2 = String::from("give it back!");
    let q3 = takes_and_gives_back(q2);
    println!("{}", q3);

}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("this string is mine now: {}", some_string)
} // some_string goes out of scope and `drop` is called

fn takes_and_gives_back(some_string: String) -> String {
    println!("this string is mine now: {}", some_string);
    println!("but I'll give it back");
    some_string
}

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("I got just a copy of {}, nothing to see here", some_integer)
} // some_integer goes out of scope, nothing special happens
