/// Unsafe Rust
/// starting a new block with the unsafe keyword allows you to switch to unsafe Rust
/// it gives you the ability to:
/// -> dereference a raw pointer
/// -> call an unsafe function or method
/// -> access or modify a mutable static variable
/// -> implement an unsafe trait
/// -> access fields of unions

fn main() {
    // Dereferencing a raw pointer
    // raw pointers that are similar to references and can be immutable or mutable 
    // they are written as *const T and *mut T
    // the asterisk in this case part of the type name
    // in the context of raw pointers immutable means 
    // that the pointer can’t be directly assigned to after being dereferenced

    // raw pointers can be created in safe code
    // they just can’t be dereference outside of an unsafe block
    let mut num = 5;
    // creating an immutable raw pointer from a reference
    let r1 = &num as *const i32; 
    // creating a mutable raw pointer from a reference
    let r2 =  &mut num as *mut i32; 

    // dereferencing these two raw pointers and reading the data 
    // they are pointing to 
    unsafe {
        // using the dereference operator * on a raw pointer which requires an unsafe block
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    
}
