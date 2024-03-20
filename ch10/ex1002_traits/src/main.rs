/// traits
/// a trait defines functionality a particular type has and can share with other types
/// traits can be used to define shared behavior in an abstract way
/// defining a trait
/// trait definitions are a way to group method signatures together to define a set of behaviors
/// example: a media aggregator library crate that can display summaries of data 
/// that might be stored in different structs like a NewsArticle or Tweet instance
/// to achive this we'll create a summary from each type by calling a summarize method
/// 
/// a summary trait that consists of the behavior provided by a summarize method
/// declared as pub so that crates depending on this crate can make use of it too
pub trait Summary {
    fn summarize(&self) -> String;
}


fn main() {
    println!("Hello, world!");
}
