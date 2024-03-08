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

}
