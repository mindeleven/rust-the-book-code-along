/// example: a media aggregator library crate that can display summaries of data 
/// that might be stored in different structs like a NewsArticle or Tweet instance
/// to achive this we'll create a summary from each type by calling a summarize method
/// 
/// a summary trait that consists of the behavior provided by a summarize method
/// declared as pub so that crates depending on this crate can make use of it too

pub mod aggregator {
    
    pub trait Summary {
        fn summarize(&self) -> String;
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
    }

}
