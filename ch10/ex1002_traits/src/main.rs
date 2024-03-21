/// traits
/// a trait defines functionality a particular type has and can share with other types
/// traits can be used to define shared behavior in an abstract way
/// defining a trait
/// trait definitions are a way to group method signatures together to define a set of behaviors
use ex1002_traits::aggregator::{
    Summary, 
    Tweet, 
    NewsArticle, 
    NewsArticleType2
};
use std::fmt::Display;

fn main() {
    // instantiating the Tweet struct and calling summarize on it
    let tweet = Tweet {
        username: String::from("A. L. Crego"),
        content: String::from(
            "The greatness of gif format is (paradoxically) in its limits.",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("summarize tweet with default: {}", tweet.summarize_with_default());

    // instantiating the NewsArticle struct and calling summarize on it
    let article1 = NewsArticle {
        headline: String::from("Goldman Sachs Sees More Institutions Diving Into Crypto — Says Bitcoin ETFs Prompt a 'Psychological Shift'"),
        location: String::from("news.bitcoin.com, Finance"),
        author: String::from("Kevin Helms"),
        content: String::from(
            "Goldman Sachs is seeing more institutions diving into crypto, the global investment bank's head of digital assets has revealed, noting that until now the bitcoin price action has been driven primarily by retail investors. \"But it's the institutions that we've started to see come in,\" he stressed, adding that the appetite has \"transformed.\"",
        ),
    };
    println!("1 new news article: {}", article1.summarize());
    println!("summarize article with default: {}", article1.summarize_with_default());

    let article1 = NewsArticleType2 {
        headline: String::from("Bitcoin Technical Analysis: Bearish Signals Amid Market Turbulence Put Traders on High Alert"),
        location: String::from("news.bitcoin.com, Markets and Prices"),
        author: String::from("Kevin Helms"),
        content: String::from(
            "Bitcoin, with its intraday low and high at $60,760 and $66,382 respectively, and currently trading above the $63,500 mark, showcases its durability and unpredictability. The price movements of bitcoin persist in the lead-up to the U.S. Federal Reserve's gathering on Wednesday afternoon.",
        ),
    };
    println!("1 new news article: {}", article1.summarize());
    println!("summarize article with default: {}", article1.summarize_with_default());

}

/// Using trait bounds to conditionally implement methods
#[allow(dead_code)]
struct Pair<T> {
    x: T,
    y: T,
}

/// the trait bound can be used with an impl block that uses generic type parameters
impl<T> Pair<T> {
    fn _new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
/// we can implement methods conditionally for types that implement the specified traits
/// the type Pair<T> always implements the new function 
/// in the following impl block Pair<T> only implements the cmp_display method 
/// if its inner type T implements the PartialOrd trait
impl<T: Display + PartialOrd> Pair<T> {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// blanket implementations -> implementations of a trait on any type 
// that satisfies the trait bounds
// example: the standard library implements the ToString trait on any type
// that implements the Display trait
/* impl<T: Display> ToString for T {
    unimplemented!()
} */

