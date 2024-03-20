/// example: a media aggregator library crate that can display summaries of data 
/// that might be stored in different structs like a NewsArticle or Tweet instance
/// to achive this we'll create a summary from each type by calling a summarize method
/// 
/// a summary trait that consists of the behavior provided by a summarize method
/// declared as pub so that crates depending on this crate can make use of it too

pub mod aggregator {

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

}
