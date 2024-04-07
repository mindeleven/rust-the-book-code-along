use rand::prelude::*;

pub fn use_rand_to_get_f64() -> f64 {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen(); // generates a float between 0 and 1
    y
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// running cargo test in a workspace structured will run tests for all crates in the workspace
/// tests for one particular crate in a workspace can be run from the top-level directory 
/// by using the -p flag and specifying the name of the crate we want to test
/// `cargo test -p add_one`

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
        // let's just test add_one() for this example
        assert_eq!(3, add_one(2));

    }
}
