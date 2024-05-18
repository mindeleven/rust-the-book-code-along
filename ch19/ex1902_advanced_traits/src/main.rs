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

fn main() {
    println!("Hello, world!");
}
