#![allow(dead_code)]
/// a pointer is a general concept for a variable that contains an address in memory
/// -> smart pointers are data structures that have additional metadata and capabilities
/// -> while references only borrow data, smart pointers own the data they point to
/// -> smart pointers implement the Deref and Drop traits
/// 
/// the most common smart pointers in the Rust standard library are:
/// (1) Box<T> for allocating values on the heap
/// (2) Rc<T>, a reference counting type that enables multiple ownership
/// (3) Ref<T> and RefMut<T>, a type that enforces the borrowing rules at runtime
///     both are accessed through RefCell<T>
///
/// Using a Box<T> to store data on the heap
/// the most straightforward smart pointer is a box, whose type is written Box<T>
/// -> boxes allow you to store data on the heap rather than the stack
/// -> what remains on the stack is the pointer to the heap data
/// 
/// situations where you would use boxes:
/// (1) you have a type whose size can’t be known at compile time 
///     and you want to use a value of that type in a context that requires an exact size
/// (2) you have a large amount of data and you want to transfer ownership 
///     but ensure the data won’t be copied when you do so
/// (3) you want to own a value and you care only 
///     that it’s a type that implements a particular trait 
///     rather than being of a specific type
/// 
/// Enabling recursive types with Boxes
/// a value of recursive type can have another value of the same type as part of itself
/// an example of a recursive type is the cons list
/// each item in a cons list is made up of nested pairs (contains two elements)
/// each item in the list contains the value of the current item and the next item
/// example: an enum definition for a cons list
/// 
/// we can put a Box<T> inside the Cons variant instead of another List value directly
/// Box<T> is a pointer and Rust always knows how much space a Box<T> needs 
/// -> a pointer’s size doesn’t change based on the amount of data it’s pointing to
/// the Box<T> will point to the next List value that will be on the heap 
/// rather than inside the Cons variant
/// the Box<T> type is a smart pointer because it implements the Deref trait
/// -> which allows Box<T> values to be treated like references
/// -> the Drop trait implementation guarantees that the heap data that the box is pointing to 
///    is cleaned up as well when a Box<T> value goes out of scope
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // Using a Box<T> to store data on the heap
    let b = Box::new(5);
    println!("b = {}", b);
    
    // Enabling recursive types with Boxes
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
    
}
