#![allow(dead_code)]
/// tests are Rust functions 
/// they verify that the non-test code is functioning in the expected manner
/// the body of a test functions
/// -> sets up any needed data or state
/// -> runs the code you want to test
/// -> asserts the results are what you expect

/// as an example once again the rectangle struct from chapter 5
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;    

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another_one_bites_the_dust() {
        panic!("Make this test fail!");
    }

    // Checking results with the assert! macro
    // the assert! macro can ensure that some condition in a test evaluates to true
    // writing a test that exercises the can_hold method by creating a Rectangle instance
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        // the correct result of the can_hold function in this case is false
        // we need to negate that result before we pass it to the assert! macro
        assert!(!smaller.can_hold(&larger));
    }
    
    // testing equality with the assert_eq! and assert_ne! macros
    // verifying functionality by testing for equality 
    // between the result of the code under test and the value you expect the code to return
    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(10), 12);
    }

    // the assert_ne! macro will pass if the two values we give it are not equal 
    // and fail if they’re equal
    #[test]
    fn it_adds_two_and_fails() {
        assert_ne!(add_two(10), 13);
    }

    // Adding custom failure messages
    // custom message can be added to be printed with the failure message 
    // as optional arguments to the assert!, assert_eq! and assert_ne!
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // just assert that the output contains the text of the input parameter
        assert!(result.contains("Carol"));
    }
    // same test with a custom failure message 
    // composed of a format string with a placeholder 
    // filled in with the actual value from the greeting function
    #[test]
    fn greeting_contains_name_fails() {
        let result = greeting("Carol");
        // just assert that the output contains the text of the input parameter
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

}

// adding custom failure messages
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

// testing equality with the assert_eq! and assert_ne! macros
pub fn add_two(a: i32) -> i32 {
    a + 2
}
