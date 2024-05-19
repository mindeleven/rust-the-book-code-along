#![allow(dead_code)]
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

/// the add trait has a default type parameters
/// trait Add<Rhs=Self> {...} // Rhs=Self =>  default type parameter syntax
/// if a concrete type doesn't get specified when we implement the Add trait, 
/// the type of Rhs will default to Self

/// implementing the Add trait for Point with using the default for Rhs
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// another example for overloading the add trait with setting the value of the Rhs type
/// example: two structs, Millimeters and Meters, that are thin wrapping an existing type
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}


fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    
    // implementing different traits on one type that both have methods with the same name
    let person = Human;
    // if called like this the direct implementation wins
    person.fly();
    
    // specifying which fly method we mean by using more explicit syntax
    Pilot::fly(&person);
    Wizard::fly(&person);

    println!("Nothing to see here, please move on");
}

/// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
/// implementing different traits on one type that both have methods with the same name
trait Pilot {
fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
