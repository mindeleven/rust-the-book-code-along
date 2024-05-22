/// Using the newtype pattern to implement external traits on external types
use std::fmt;

/// the newtype pattern is a lightweight way to achieve encapsulation to hide implementation details
/// kind of a workaround for implementing Display on Vec<T>:
/// (1) defining a Wrapper tuple struct that holds an instance of Vec<T>
struct Wrapper(Vec<String>);
/// (2) then implementing Display on Wrapper and using the Vec<T>
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // the implementation of Display uses self.0 to access the inner Vec<T>
        write!(f, "[{}]", self.0.join(", "))
    }   
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    // Creating type synonyms with type aliases
    // creating the alias Kilometers to i32
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 10;
    // Kilometers and i32 are the same type
    println!("x + y = {}", x + y);

    // main use case for type synonyms: reducing repetition
    // (= reducing typing when it comes to lengthy types)
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let _f: Thunk = Box::new(|| println!("hi"));
    // as function parameter or return type:
    fn _takes_long_type(f: Thunk) {
       unimplemented!()
    }
    fn _returns_long_type() -> Thunk {
        unimplemented!()
    }
}
