use hello_macro::HelloMacro;

/// a first approach on how to implement the trait to achieve the desired functionality
struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

// 2nd approach
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes2;

#[derive(HelloMacro)]
struct TaoCat;

fn main() {
    // first approach
    Pancakes::hello_macro();
    
    // 2nd approach
    Pancakes2::hello_macro();
    TaoCat::hello_macro();
}
