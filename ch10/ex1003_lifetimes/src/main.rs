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
/// this signature does not compile -> "expected named lifetime parameter"
/* fn longest(x: &str, y: &str) -> &str { */
/// the return type needs a generic lifetime parameter on it
/// because Rust can’t tell whether the reference being returned refers to x or y
/// signature needs to express the following constraint: 
/// the returned reference will be valid as long as both the parameters are valid
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // lifetime annotation syntax
    if x.len() > y.len() {
        x
    } else {
        y
    }
}