/// the reasons to choose Box<T>, Rc<T> or RefCell<T>:
/// 
/// Box<T> allows immutable or mutable borrows checked at compile time
/// Box<T> and RefCell<T> have single owners
/// 
/// Rc<T> allows only immutable borrows checked at compile time
/// Rc<T> enables multiple owners of the same data
/// 
/// RefCell<T> allows immutable or mutable borrows checked at runtime
/// -> you can mutate the value inside the RefCell<T> 
///    even when the RefCell<T> itself is immutable
/// Box<T> and RefCell<T> have single owners

/// Interior Mutability: A Mutable Borrow to an Immutable Value

fn main() {
    println!("Hello, world!");
}
