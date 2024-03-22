fn main() {
    // Main aim if lifetimes: preventing dangling references
    // example program with a dangling reference
    // the borrow checker compares scopes to determine whether all borrows are valid
    let r;

    {
        let x = 5;
        let r = &x;
    }

    println!("r: {}", r);
}
