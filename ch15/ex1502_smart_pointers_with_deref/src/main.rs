fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    // to make an assertion about the value in y we have to dereference it
    // a number and a reference to a number are of different types
    // we can't compare them
    assert_eq!(5, *y);
}
