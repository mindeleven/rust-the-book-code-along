fn main() {
    // Main aim if lifetimes: preventing dangling references
    // example program with a dangling reference
    // the borrow checker compares scopes to determine whether all borrows are valid
    /* let r;
    {
        let x = 5;
        let r = &x;
    } */
    // fixing the code so it doesn't have a dangling reference
    let x = 5;
    let r = &x;

    println!("r: {}", r);

    // Generic Lifetimes in Functions
    let string1 = String::from("abc");
    let string2 = "wxyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

/// Generic Lifetimes in Functions
/// function takes string slices (which are references) rather than strings
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}