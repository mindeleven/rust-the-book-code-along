/// Refutability: Whether a Pattern Might Fail to Match
/// patterns that will match for any possible value passed are irrefutable
/// patterns that can fail to match for some possible value are refutable
/// example of a refutable pattern: Some(x) in the expression `if let Some(x) = a_value`
/// if the value in the a_value variable is None the Some(x) pattern will not match
/// if let and while let expressions accept refutable and irrefutable patterns

fn main() {
    // example code that uses a refutable pattern
    let some_option_value = Some(String::from("some option value"));
    if let Some(x) = some_option_value {
        println!("1) x: {}", x);
    } else {
        println!("1) found None");
    }

    let some_none: Option<i32> = None;
    if let Some(y) = some_none {
        println!("2) y: {}", y);
    } else {
        println!("2) found None");
    }

    println!("Hello, Refutability!");
}
