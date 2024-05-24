/// Advanced Functions and Closures
/// 
/// Function Pointers
/// syntax for specifying that a parameter is a function pointer
fn add_one(x: i32) -> i32 {
    x+ 1
}
/// following function takes two parameters: 
/// (1) a function pointer to any function that takes an i32 parameter and returns an i32
/// (2) one i32 value
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let res = do_twice(add_one, 332);

    println!("If we do it twice we get {}", res);
}
