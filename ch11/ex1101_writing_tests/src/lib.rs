/// tests are Rust functions 
/// they verify that the non-test code is functioning in the expected manner
/// the body of a test functions
/// -> sets up any needed data or state
/// -> runs the code you want to test
/// -> asserts the results are what you expect

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
}
