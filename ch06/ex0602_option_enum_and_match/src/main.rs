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
    Quarter(UsState),
}

#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
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

    println!("------------------------------------------------------------");
    println!("Patterns That Bind to Values");
    println!("------------------------------------------------------------");
    // match arms can bind to the parts of the values that match the pattern
    let my_quarter = Coin::Quarter(UsState::Alaska);
    println!("My querter has the value {}", value_in_cents(my_quarter));

    println!("------------------------------------------------------------");
    println!("Matching with Option<T>");
    println!("------------------------------------------------------------");
    let five = Some(5);
    let six = match_and_add_one(five);
    let none: Option<i32> = match_and_add_one(None);
    println!("{:?}", six);
    println!("{:?}", none);


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
        Coin::Quarter(state) => {
            // we can extract the binding for the value state here
            // and use it in a println expression
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

// function to match an option of type i32
fn match_and_add_one(x: Option<i32>) -> Option<i32> {
    match x {
        // matches are exhaustive so some and none need to be handled
        None => None,
        Some(x) => Some(x + 1),
    }
}