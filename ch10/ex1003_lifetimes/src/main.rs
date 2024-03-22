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

    let result = longest1(string1.as_str(), string2);
    println!("Return value of longest that always returns first string: {}", result);
}

/// Generic Lifetimes in Functions
/// function takes string slices (which are references) rather than strings
/// this signature does not compile -> "expected named lifetime parameter"
/* fn longest(x: &str, y: &str) -> &str { */
/// the return type needs a generic lifetime parameter on it
/// because Rust can’t tell whether the reference being returned refers to x or y
/// signature needs to express the following constraint: 
/// the returned reference will be valid as long as both the parameters are valid
/// the function definition specifies that all the references in the signature 
/// must have the same lifetime 'a
/// when returning a reference from a function, the lifetime parameter 
/// for the return type needs to match the lifetime parameter for one of the parameters
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // lifetime annotation syntax
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Thinking in terms of lifetimes
// example: changing the implementation of the longest function 
// to always return the first parameter rather than the longest string slice
fn longest1<'a>(x: &'a str, _y: & str) -> &'a str { 
    x
}