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

fn main() {
    // Using a Box<T> to store data on the heap
    let b = Box::new(5);
    println!("b = {}", b);
}
