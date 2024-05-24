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

    // example where you could use either a closure defined inline or a named function
    // using the map function to turn a vector of numbers into a vector of strings
    // closure approach
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("The list of strings we got with a closure {:?}", list_of_strings);
    // function pointer approach
    let list_of_numbers_2 = vec![1, 2, 3];
    let list_of_strings_2: Vec<String> =
        list_of_numbers_2.iter().map(ToString::to_string).collect();
        println!("The list of strings we got with a function pointer {:?}", list_of_strings_2);

}
