#![allow(dead_code, unused_variables)]
/// Rc<T>, the Reference Counted Smart Pointer
/// in some cases a single value might have multiple owners like in a graph data structure
/// -> multiple edges might point to the same node
/// -> and that node is conceptually owned by all of the edges that point to it
/// in Rust it's possible to enable multiple ownership explicitly 
/// by using the Rust type Rc<T> (reference counting)
/// Rc<T> type keeps track of the number of references to a value
/// if there are zero references to a value it can be cleaned up

enum List {
    // Cons(i32, Box<List>),
    // using Rc<T> in place of Box<T>
    // each Cons variant will now hold a value and an Rc<T> pointing to a List
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // creating two lists (b and c) that both share ownership of a third list (a)
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // instead of taking ownership of a
    // b and c are cloning the Rc<List> that a is holding
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a)); // error[E0382]: use of moved value: `a`
    // Rc::clone doesn’t make a deep copy of all the data
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(3, Rc::clone(&a));
}

