#![allow(dead_code)]
/// Pattern Syntax
/// gathering all the syntax valid in patterns 
/// plus examples for why and when to use each one

/// struct for the destructuring structs example
struct Point {
    x: i32,
    y: i32
}

fn main() {
    // matching patterns against literals directly
    let x = 66;
    match x {
        1 => println!("x is one"),
        2 => println!("x is two"),
        3 => println!("x is three"),
        _ => println!("x is something else")
    }

    // matching named variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // pattern in the second match arm introduces a new variable named y 
        // that will match any value inside a Some value
        // this new y binds to the inner value of the Some in x which is 5
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x)
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    // Multiple Patterns
    // multiple patterns can be matched using the | syntax
    let x = 1;
    match x {
        1 | 2 => println!("x is one or two, who knows"),
        3 => println!("x is three"),
        _ => println!("x is something else")
    }

    // Matching Ranges of Values with ..=
    let x = 5;
    match x {
        1..=5 => println!("x is in one through five"),
        _ => println!("x is something else")
    }

    // example using ranges of char values
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter between a and j"),
        'k'..='z' => println!("late ASCII letter between k and z"),
        _ => println!("c is something else")
    }

    // Destructuring to Break Apart Values
    // destructuring structs, enums, and tuples to use different parts of these values

    // Destructuring Structs
    let my_point = Point { x: 7, y: 12};
    // pattern to match struct fields
    // code creates variables that match the x and y fields
    let Point { x: a, y: b } = my_point;
    assert_eq!(a, 7);
    assert_eq!(b, 12);
    // shorter when naming variables like struct fields
    let Point { x, y } = my_point;
    assert_eq!(x, 7);
    assert_eq!(y, 12);
    // destructuring with literal values as part of the struct pattern
    let another_point = Point { x: 0, y: 7 };
    // destructuring and matching literal values in one pattern
    match another_point {
        // first arm matches any point that lies on the x axis by specifying what the y field matches
        Point { x, y: 0 } => println!("y is on the x axis at {x}"),
        // second arm matches any point on the y axis by specifying what the x field matches 
        Point { x: 0, y } => println!("x is on the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // Destructuring Enums
    // the pattern to destructure an enum corresponds to the way 
    // the data stored within the enum is defined
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    // let msg_quit = Message::Quit;
    // let msg_write = Message::Write(String::from("a long long way from home"));
    // let msg_move = Message::Move{ x: 36, y: 89 };

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        },
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        },
        Message::Write(text) => {
            println!("Text message: {text}");
        },
        Message::ChangeColor(r, b, g) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }

}
