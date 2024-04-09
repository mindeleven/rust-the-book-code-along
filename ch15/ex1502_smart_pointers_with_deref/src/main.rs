#![allow(dead_code, unused_variables)]
/// building a smart pointer similar to the Box<T>
/// Box<T> type is defined as a tuple struct with one element
/// we're doing the same with MyBox<T>
struct MyBox<T>(T);

impl<T> MyBox<T> {
    // MyBox::new function takes one parameter of type T 
    // and returns a MyBox instance that holds the value passed in
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
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
