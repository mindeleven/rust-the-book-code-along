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

    // Checking for panics with should_panic
    // checking that our code handles error conditions as expected
    // as an example, back to the Guess type from Chapter 9
    // writing a test that ensures that attempting to create a Guess instance 
    // with a value outside that range between 1 and 100 panics
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
    // same as above that will fail because it doesn't panic
    #[test]
    #[should_panic]
    fn greater_than_100_err() {
        Guess::new(100);
    }

    // Using Result<T, E> in tests
    // test that returns an Err instead of panicking:
    #[test]
    fn it_works_with_result() -> Result<(), String> {
        // instead of calling assert_eq!
        // we return Ok(()) when the test passes 
        // and an Err with a String inside when the test fails
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

}

// as an example, back to the Guess type from Chapter 9
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
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
