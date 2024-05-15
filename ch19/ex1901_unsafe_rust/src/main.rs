use core::slice;

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

    // Calling an Unsafe Function or Method
    // the second type of operation you can perform in an unsafe block is calling unsafe functions
    unsafe {
        dangerous();
    }

    // Creating a safe abstraction over unsafe code
    // wrapping unsafe code in a safe function is a common abstraction
    // example: the split_at_mut function from the standard library
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];
    println!("{:?}", r);
    // split_at_mut() divides one mutable slice into two at an index
    // let (a, b) = r.split_at_mut(3);
    let (a, b) = r.split_at_mut(3);
    println!("a: {:?}, b: {:?}", a, b);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    // using our own implementation of split_at_mut():
    let mut v2 = vec![1, 2, 3, 4, 5, 6];
    let (c, d) = my_split_at_mut(&mut v2, 3);
    println!("c: {:?}, d: {:?}", a, b);
    assert_eq!(c, &mut [1, 2, 3]);
    assert_eq!(d, &mut [4, 5, 6]);

    // println!("Gets this code printed?");
}

// unsafe function to be called in an unsafe block
unsafe fn dangerous() {}

// implementing our own version of split_at_mut() with using only safe Rust
fn my_split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    /* 
    let len = values.len();

    assert!(mid <= len);
    // the borrow checker will complain because we're borrowing from the same slice twice
    (&mut values[..mid], &mut values[mid..])
    */

    // doing it with unsafe,
    // using an unsafe block, a raw pointer & some calls to unsafe functions
    let len = values.len();
    // as_mut_ptr() returns an unsafe mutable pointer to the slice's buffer.
    let ptr = values.as_mut_ptr(); 

    assert!(mid <= len);
    
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }

}
