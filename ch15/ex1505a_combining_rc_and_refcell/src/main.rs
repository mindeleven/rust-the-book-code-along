#![allow(dead_code)]
/// Having multiple owners of mutable data by combining Rc<T> and RefCell<T>
/// -> Rc<T> lets you have multiple owners of some data but it only gives immutable access
/// with an Rc<T> holding a RefCell<T> 
/// -> you can get a value that can have multiple owners
/// -> and that can be mutated
/// 
/// back to the cons list example: 
/// -> we used Rc<T> to allow multiple lists to share ownership of another list
/// here we're adding in RefCell<T> to gain the ability to change the values in the lists

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // creating a value that is an instance of Rc<RefCell<i32>>
    let value = Rc::new(RefCell::new(5));
    // creating a List in a with a Cons variant that holds value
    // we need to clone value so both a and value have ownership of the inner 5 value
    // we wrap the list a in an Rc<T> so when we create lists b and c, they can both refer to a
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    
    // adding 10 to the value in value by calling borrow_mut() on value
    // borrow_mut() uses automatic dereferencing
    // to dereference the Rc<T> to the inner RefCell<T> value
    // borrow_mut() returns a RefMut<T> smart pointer, 
    // and we use the dereference operator on it and change the inner value
    *value.borrow_mut() += 10;
    
    // when we printing a, b, and c we can see that they all have the modified value
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
