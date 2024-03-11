/// Structs can hold multiple related values that can be of different types
/// each piece of data has to be named so it’s clear what the values mean
/// the values of the data an instance can be accessed by the field names
#[derive(Debug)]
#[allow(dead_code)]
struct User {
    active: bool,
    user_name: String,
    email: String,
    sign_in_count: u64,
}

/// Tuple structs
/// tuple structs are structs that look similar to tuples
/// tuple structs have the added meaning of struct name provides
/// but don’t have names associated with their fields
/// defining a tuple struct by with the struct keyword and the struct name 
/// followed by the types in parentheses
#[allow(dead_code)]
struct Color(i32, i32, i32);
#[allow(dead_code)]
struct Point(i32, i32, i32);

/// Unit-like Structs
/// defining structs that don’t have any fields
/// they behave similarly to (), the unit type in tuples
/// useful when defining traits
struct AlwaysEqual;

fn main() {
    println!("------------------------------------------------------------");
    println!("Structs");
    println!("------------------------------------------------------------");
    // creating an instance of the User struct
    // an instance of a struct is created by specifying concrete values for each of the fields
    // if we want to change the data the entire instance has to be mutable
    // Rust doesn’t allow to mark only certain fields as mutable
    let mut user_x = User {
        active: true,
        user_name: String::from("User X"),
        email: String::from("user_x@some_weird_place.io"),
        sign_in_count: 1,
    };

    // accessing and changing a specific value from a struct with dot notation
    // struct has to be mutable to do this
    user_x.email = String::from("user_x@weird_worlds.io");

    let user_y = build_me_an_user(
        String::from("User Y"),
        String::from("the_last_man@example.io")
    );
    
    println!("{:?}", user_x);
    println!("{:?}", user_y);

    println!("------------------------------------------------------------");
    println!("Struct Update Syntax");
    println!("------------------------------------------------------------");
    // creating instances from other instances with struct update syntax
    // creating a new instance of a struct that includes most of the values from another instance
    // creating a new User instance using one of the values from user_x
    let user_x1 = User {
        active: user_x.active,
        user_name: user_x.user_name,
        email: String::from("user_x_sub_z@some_weird_place.io"),
        sign_in_count: user_x.sign_in_count,
    };
    println!("{:?}", user_x1);

    // shortcut: the syntax .. specifies that the remaining fields not explicitly set 
    // should have the same value as the fields in the given instance
    // Struct Update Syntax
    let user_y1 = User {
        email: String::from("y_the_last_man@example.io"),
        // The ..user_y syntax must come last to specify that any remaining fields 
        // should get their values from the corresponding fields in user_y
        // the fields are moved into user_y1 and can no longer be used
        ..user_y
    };
    println!("{:?}", user_y1);

    println!("------------------------------------------------------------");
    println!("Tuple structs");
    println!("------------------------------------------------------------");
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    println!("------------------------------------------------------------");
    println!("Unit-like Structs");
    println!("------------------------------------------------------------");
    let _subject = AlwaysEqual;

}

// function to intantiate a new user
fn build_me_an_user(name: String, email: String) -> User {
    User {
        active: true,
        user_name: name,
        // field init shorthand syntax 
        // because parameter and field have the same name
        email,
        sign_in_count: 1,
    } // function returns the user instance
}