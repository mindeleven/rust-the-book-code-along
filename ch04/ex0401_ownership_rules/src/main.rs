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
        let s = "hello"; // variable s refers to a string literal
        println!("let's do something with s at least once: {}", s);
    } // this scope is now over, and s is no longer valid

}
