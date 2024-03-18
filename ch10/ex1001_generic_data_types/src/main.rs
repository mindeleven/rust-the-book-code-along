/// continuing with the largest function example from the previous section
/// we've now two functions that both find the largest value in a slice
/// one takes a reference to a list of i32s, the other one to a list of chars

use std::cmp::PartialOrd;

/// defining a generic largest function
/// define a generic function, place type name declarations inside angle brackets <>
/// between the name of the function and the parameter
fn _generic_fn<T>(_list: &[T]) -> &T {
    unimplemented!()
}

/// for the largest function this looks like
/// and it reads like: "the function largest is generic over some type T"
/// it has one parameter named list which is a slice of values of type T
/// and will return a reference to a value of the same type T
/// only types whose values can be ordered can be used her
/// to enable comparisons the standard library has the std::cmp::PartialOrd trait 
/// that we need to implement on these types
/// we restrict the types valid for T to only those that implement PartialOrd
fn find_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/// Generic Types in Struct Definitions
/// defining a struct to use a generic type parameter using the <> syntax
/// in this example both fields have to be of the same type
#[derive(Debug)]
struct Point1<T> {
    x: T,
    y: T
}
/// implementing methods on structs with generic types in their definitions
impl<T> Point1<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}
/// specifying constraints on generic types when defining methods on the type
/// instances of Point<T> where T is not of type f32 will not have this method defined
/// the method uses mathematical operations that are available only for floating point types
impl Point1<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

/// defining a struct with different types as generics
#[derive(Debug)]
#[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U
}

/// Generic Types in Enum Definitions
/// we can define enums to hold generic data types in their variants
/// an example for this is the Option<T> enum comes with the standard library
/// the Option<T> enum is generic over type T and has two variants
/// Some which holds one value of type T
/// and a None that doesn’t hold any value
/// we can use this abstraction no matter what the type of the optional value is
/* enum Option<T> {
    Some(T),
    None,
} */
/// an example for Enums with multiple generic types is the Result<T, E> enum
/// the Result enum is generic over two types, T and E, and has two variants: 
/// Ok, which holds a value of type T
/// and Err which holds a value of type E
/* enum Result<T, E> {
    Ok(T),
    Err(E),
} */

fn main() {
    let number_list = vec![23, 76, 33, 44, 96, 15, 28];
    let largest_number = find_largest(&number_list);
    println!("The largest number in the i32 list is {}.", largest_number);

    let char_list = vec!['c', 'h', 'l', 'q', 'p', 'y', 'r'];
    let largest_char = find_largest(&char_list);
    println!("The largest char in the char list is {}.", largest_char);

    // initiating a struct with only one generic parameters
    let point = Point1 { x: 9, y: 4 }; // both integer
    println!("x in point is {} and y in point is {}", point.x(), point.y());
    let point_f = Point1 { x: 9.4, y: 4.5 }; // both integer
    println!("the point_f's distance from origin is {}", point_f.distance_from_origin());
    // initiating a struct with different types as generic parameters
    let point1 = Point { x: 9, y: 4 }; // both integer
    let point2 = Point { x: 9.3, y: 4.1 }; // both float
    let point3 = Point { x: 9, y: 4.1 }; // integer and float
    println!("{:?}, {:?}, {:?}, {:?}", point, point1, point2, point3);
}