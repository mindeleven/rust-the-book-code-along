/// Advanced Traits
/// Specifying Placeholder Types in Trait Definitions with Associated Types
/// Associated types => connect a type placeholder with a trait 
///     such that the trait method definitions 
///     can use these placeholder types in their signatures

/// Iterator trait -> example of a trait with an associated type 
pub trait Iterator {
    // Item stands for the type of the values 
    // that the type implementing the Iterator trait is iterating over
    type Item; // associated type

    fn next(&mut self) -> Option<Self::Item>;
}

/// Default Generic Type Parameters and Operator Overloading
/// specifying a default concrete type for the generic type
/// declaring a generic type with the <PlaceholderType=ConcreteType> syntax
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
/// overloading the operations and corresponding traits listed in std::ops 
/// by implementing the traits associated with the operator
/// example: overloading the + operator to add two Point instances together
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    println!("Nothing to see here, please move on");
}
