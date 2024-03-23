/// Lifetime annotations in struct definitions
/// if a struct is defined to hold references we need to add a lifetime annotation 
/// on every reference in the struct’s definition
#[derive(Debug)]
#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    // field part that holds a string slice which is a reference
    // annotation means an instance of this struct can’t outlive the reference it holds here
    part: &'a str,
}

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

    // Lifetime annotations in struct definitions
    let novel = String::from("It was a bright cold day in April, and the clocks were striking thirteen. Winston Smith, his chin nuzzled into his breast in an effort to escape the vile wind, slipped quickly through the glass doors of Victory Mansions, though not quickly enough to prevent a swirl of gritty dust from entering along with him.");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence
    };
    // data in novel exists before the ImportantExcerpt instance is created
    // novel doesn’t go out of scope until after the ImportantExcerpt goes out of scope
    // so the reference in the ImportantExcerpt instance is valid
    println!("{:?}", i);

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
