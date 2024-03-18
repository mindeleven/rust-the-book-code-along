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

fn main() {
    let number_list = vec![23, 76, 33, 44, 96, 15, 28];
    let largest_number = find_largest(&number_list);
    println!("The largest number in the i32 list is {}.", largest_number);

    let char_list = vec!['c', 'h', 'l', 'q', 'p', 'y', 'r'];
    let largest_char = find_largest(&char_list);
    println!("The largest char in the char list is {}.", largest_char);
}
