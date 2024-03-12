/// the option enum and its advantages over null values
/// Option is an enum defined by the standard library
/// it encodes the very common scenario in which a value could be something or it could be nothing
/// Rust does not have nulls but it does have an enum that can encode
/// the concept of a value being present or absent
/// definition of the enum Option<T> by the standard library:
/* 
enum Option<T> {
    None,
    Some(T),
}
*/
/// the Option<T> enum and its variants Some and None are included in the prelude

#[derive(Debug)]
enum MyOption<T> {
    None,
    Some(T),
}

#[derive(Debug)]
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    println!("------------------------------------------------------------");
    println!("The Option enum");
    println!("------------------------------------------------------------");
    // examples of using Option values to hold number types and string types
    // the two following types can be infered 
    // because a value has been specified inside the Some variant
    let _some_char = Some('e');
    let _some_number = Some(5);
    // Rust can infer the type from a none value 
    // therefore a type annotation is required
    let _absent_number: Option<i32> = None;

    let some_other_number = MyOption::Some(5);
    let some_other_absent_number: MyOption<i32> = MyOption::None;
    println!("{:?}, {:?}", some_other_number, some_other_absent_number);

    println!("------------------------------------------------------------");
    println!("The match Control Flow Construct");
    println!("------------------------------------------------------------");
    // match allows you to compare a value against a series of patterns
    // and then execute code based on which pattern matches
    // all possible cases are handled otherwise the compiler will complain
    let my_coin = Coin::Penny;
    println!("My coin has the value {}", value_in_cents(my_coin));

}

// function that takes an unknown US coin and returns its value in cents
fn value_in_cents(coin: Coin) -> u8 {
    // match expression that has the variants of an enum as its patterns
    // when the match expression executes 
    // it compares the value of coin against the pattern of each arm
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 20,
        Coin::Quarter => 25,
    }
}