fn main() {
    // variables are immutable by default
    // to make them mutable the mut keyword is needed
    let mut x = 5;
    println!("The value of x = {}", x);
    x = 6;
    println!("Now the value of x = {}", x);
}
