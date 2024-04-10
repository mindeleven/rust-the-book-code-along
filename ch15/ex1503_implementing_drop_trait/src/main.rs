#![allow(unused_variables)]
/// the Drop trait lets you customize what happens when a value is about to go out of scope
/// the functionality of the Drop trait is almost always used when implementing a smart pointer
/// you can specify the code to run when a value goes out of scope by implementing the Drop trait

struct CustomSmartPointer {
    data: String,
}

// implementing drop with println! statement
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointer created.");

    // c.drop(); // dropping c early returns "explicit destructor calls not allowed" error

    // if we need to force a value to be cleaned up early
    // we can use the std::mem::drop function instead
    println!("Calling drop on c....");
    drop(c);

    println!("CustomSmartPointer dropped before the end of main.");
}
