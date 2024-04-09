#![allow(dead_code, unused_variables)]
/// building a smart pointer similar to the Box<T>
/// Box<T> type is defined as a tuple struct with one element
/// we're doing the same with MyBox<T>
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    // MyBox::new function takes one parameter of type T 
    // and returns a MyBox instance that holds the value passed in
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    } 
}

/// to dereference the MyBox<T> type we have to implement the Deref ability on our type
/// implementating Deref as an addition to the definition of MyBox
/// the Deref trait requires us to implement one method named deref 
/// deref() borrows self and returns a reference to the inner data
impl<T> Deref for MyBox<T> {
    // defining an associated type for the Deref trait to use
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // we return &self.0 so that deref returns a reference to the value 
        // we want to access with the * operator
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // to make an assertion about the value in y we have to dereference it
    // a number and a reference to a number are of different types
    // we can't compare them
    assert_eq!(5, *y);
}
