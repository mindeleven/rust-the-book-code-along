#![allow(dead_code, unused_variables)]
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
    let f: Thunk = Box::new(|| println!("hi"));
    // as function parameter or return type:
    fn takes_long_type(f: Thunk) {
       unimplemented!()
    }
    fn returns_long_type() -> Thunk {
        unimplemented!()
    }

    // Type aliases for reducing repetition with the Result<T, E> type 
    // std::io has shortcut Result type alias declaration:
    type Result<T> = std::result::Result<T, std::io::Error>;
    // it is a fully qualified alias that is a Result<T, E> with the E filled in as std::io::Error
    // using it The Write trait function signatures ends up looking like this:
    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;

        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
    }

    // The Never type that never returns
    // a special type named ! aka the empty type because it has no values
    // functions that return never are called diverging functions
    fn bar() -> ! { // the function bar returns never
        unimplemented!()
    }
    // useful in match because continue has a ! value
    let guess = "42 ";
    // snipped from guessing game example, chapter 6
    loop {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // expressions of type ! can be coerced into any other type
        };
        break; // getting outta this place
    }

}
