#![allow(dead_code)]
/// Reference cycles can leak memory
/// Rust allows memory leaks by using Rc<T> and RefCell<T>:
/// -> it’s possible to create references where items refer to each other in a cycle
/// the reference count of each item in the cycle will never reach 0
/// and the values will never be dropped.
/// -> memory leaks are created

/// Creating a reference cycle
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

/// a Cons list definition that holds a RefCell<T> 
/// so we can modify what a Cons variant is referring to
/// the second element in the Cons variant is a RefCell<Rc<List>>
/// we now can modify the List value a Cons variant is pointing to
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    // tail method makes it convenient to access the second item if we have a Cons variant
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    // creating a list in variable a
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    // creating a list in variable b that points to a
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    
    // modifying the list in a to point to b
    // thereby creating a reference cycle
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
}
