/// Closures are 
/// -> a function-like construct you can store in a variable
/// -> anonymous functions that capture their environment
/// -> can be passed as arguments to other functions
/// -> closures can capture values from the scope in which they’re defined
use std::{
    thread, 
    time::Duration
};

/// Capturing the Environment with Closures
/// example: how to use closures to capture values from the environment for later use
/// scenario: t-shirt giveaway as a promotion
/// random person on mailing list gets limited edition t-shirt
/// people on the mailing list can add their favorite color to their profile
/// randow winner gets favourite color if set, otherwise t-shirt with largest stock

/// defining enum for the available shirt colors
#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red, 
    Blue,
}

/// defining struct to represent the company’s inventory
#[derive(Debug)]
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    // giveaway gets the user preference as an Option
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // if the user doesn't have any preference he gets the "most stocked"
        // if the Option<T> is the Some variant, unwrap_or_else returns the Some value
        // is it the None variant a closure gets called & returns the value returned by the closure
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }

        if num_blue > num_red {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }
 }

fn main() {
    let store = Inventory {
        shirts: vec![
            ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue,
            ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue,
            ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue,
        ]
    };
    println!("{:?}", store);

    let user_preference1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_preference1);
    println!("The user (1) with preference {:?} gets {:?}", user_preference1, giveaway1);

    let user_preference2 = Some(ShirtColor::Blue);
    let giveaway2 = store.giveaway(user_preference2);
    println!("The user (2) with preference {:?} gets {:?}", user_preference2, giveaway2);

    let user_preference3 = None;
    let giveaway3 = store.giveaway(user_preference3);
    println!("The user (3) with preference {:?} gets {:?}", user_preference3, giveaway3);

    // another closure example
    // closures are relevant only within a narrow context 
    // therefore the compiler can infer the types of the parameters and the return type
    // type annotations can be added if we want to be more verbose than is strictly necessary
    let _expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // similarities between closure and function syntax:
    /* 
    // function definition
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    // fully annotated closure definition
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // removing type annotations from the closure definition
    let add_one_v3 = |x|             { x + 1 };
    // removing the optional brackets
    let add_one_v4 = |x|               x + 1  ;
    */

    // Capturing references or moving ownership
    // (1) borrowing immutably 
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    // defining closure, binding variable to closure definition
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    // calling the closure by using the variable name and parentheses
    only_borrows();
    println!("After calling closure: {:?}", list);

    // (2) borrowing mutably
    // defining and calling a closure that captures a mutable reference
    let mut list2 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list2);
    // closure captures a mutable reference
    let mut borrows_mutably = || list2.push(7);
    // mutable borrow has happened so no other borrows are allowed
    // and not call to println! between closure definition and call
    borrows_mutably();
    println!("After calling closure: {:?}", list2);

    // (3) taking ownership
    // forcing the closure to take ownership of the values it uses in the environment
    // with the move keyword before the parameter list
    // technique mostly useful when passing a closure to a new thread to move the data
    let list3 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list3);
    // move keyword before the parameter list:
    thread::spawn(move || println!("From thread: {:?}", list3))
        .join()
        .unwrap();

    // Moving captured values out of closures and the Fn traits
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
    
    // example for moving captured values out of closures and the Fn traits
    // described below
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    // sort_by_key uses FnMut for the trait bound
    // closure gets one argument in the form of a reference to the current item
    // useful when you want to sort a slice by a particular attribute of each item
    // the closure doesn’t capture, mutate, or move out anything from its environment
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

}

/// Moving captured values out of closures and the Fn traits
/// a closure body can
/// -> move a captured value out of the closure
/// -> mutate a captured value
/// -> neither move nor mutate a captured value
/// -> capture nothing from the environment at all
/// 
/// depending on the way a closure captures and handles values from the environment 
/// it can implement different traits
/// (1) FnOnce  -> applies to closures that can be called once
/// all closures implement at least this trait
/// a closure that moves captured values out of its body will only implement FnOnce
/// (2) FnMut -> applies to closures that don’t move captured values out of their body
/// this closures might mutate the captured values
/// can be called more than once
/// (3) Fn -> applies to closures that don’t move captured values out of their body 
/// AND that don’t mutate captured values
/// as well as closures that capture nothing from their environment
/// can be called more than once without mutating their environment

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}
