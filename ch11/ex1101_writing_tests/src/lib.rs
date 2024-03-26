/// tests are Rust functions 
/// they verify that the non-test code is functioning in the expected manner
/// the body of a test functions
/// -> sets up any needed data or state
/// -> runs the code you want to test
/// -> asserts the results are what you expect

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
