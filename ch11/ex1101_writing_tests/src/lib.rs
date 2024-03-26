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
    fn another() {
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

}
