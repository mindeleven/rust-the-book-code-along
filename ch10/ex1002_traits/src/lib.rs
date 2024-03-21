/// example: a media aggregator library crate that can display summaries of data 
/// that might be stored in different structs like a NewsArticle or Tweet instance
/// to achive this we'll create a summary from each type by calling a summarize method
/// 
/// a summary trait that consists of the behavior provided by a summarize method
/// declared as pub so that crates depending on this crate can make use of it too

pub mod aggregator {
    use std::fmt::{
        Debug, 
        Display
    };

    pub trait Summary {
        fn summarize(&self) -> String;

        fn summarize_author(&self) -> String;
        
        // default implementation, relies on summarize_author() being implemented
        // summarize method with specified default string
        fn summarize_with_default(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    /// Implementing a Trait on a Type
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    // implementation of the Summary trait on the NewsArticle struct
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }

        fn summarize_author(&self) -> String {
            format!("{}", self.author)
        }

        fn summarize_with_default(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    /// Implementing a Trait on a Type
    /// another type of NewsArticle
    pub struct NewsArticleType2 {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    // implementation of the Summary trait on the NewsArticle struct
    impl Summary for NewsArticleType2 {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }

        fn summarize_author(&self) -> String {
            format!("{}", self.author)
        }
    }
    
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    // implementation of the Summary trait on the Tweet struct
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }

        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }

    }

    // Traits as Parameters
    // use traits to define a function that accepts many different types
    // the following function calls the summarize method on its item parameter
    // this item parameter is of some type that implements the Summary trait
    // and accepts any type that implements the specified trait
    pub fn notify(item: &impl Summary) {
        println!("Breaking news: {}", item.summarize());
    }

    // the same function with the "trait bound syntax"
    pub fn notify2<T: Summary>(item: &T) {
        println!("Breaking news: {}", item.summarize());
    }
    
    // examples with two parameters
    // impl Trait syntax:
    // item1 and item2 can have different types that both have to implement Summary
    pub fn notify3(_item1: &impl Summary, _item2: &impl Summary) {
        unimplemented!()
    }
    
    // two parameters with trait bound syntax
    // value passed as an argument for item1 and item2 must be of the same type
    pub fn notify4<T: Summary>(_item1: &T, _item2: &T) {
        unimplemented!()
    }

    // Specifying Multiple Trait Bounds with the + Syntax
    //  example: our notify function should use summarize and display formatting on item
    // the + syntax tells notify that item has to implement both Display and Summary
    pub fn notify5(_item: &(impl Summary + Display)) {
        unimplemented!()
    }
    // same example with trait bounds on generic types
    // the function body now can call summarize and use {} to format item
    pub fn notify6<T: Summary + Display>(_item: &T) {
        unimplemented!()
    }

    // Clearer trait bounds with where clauses
    // with too many trait bounds function signature can become hard to read
    // therefore alternate syntax for specifying trait bounds inside a where clause after
    // instead of 
    fn _some_function<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) -> i32 {
        unimplemented!()
    }
    // we can use     
    fn _some_function2<T, U>(_t: &T, _u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        unimplemented!()
    }

}
