/// traits
/// a trait defines functionality a particular type has and can share with other types
/// traits can be used to define shared behavior in an abstract way
/// defining a trait
/// trait definitions are a way to group method signatures together to define a set of behaviors
use ex1002_traits::aggregator::{Summary, Tweet, NewsArticle};

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

    let article1 = NewsArticle {
        headline: String::from("Bitcoin Technical Analysis: Bearish Signals Amid Market Turbulence Put Traders on High Alert"),
        location: String::from("news.bitcoin.com, Markets and Prices"),
        author: String::from("Kevin Helms"),
        content: String::from(
            "Bitcoin, with its intraday low and high at $60,760 and $66,382 respectively, and currently trading above the $63,500 mark, showcases its durability and unpredictability. The price movements of bitcoin persist in the lead-up to the U.S. Federal Reserve's gathering on Wednesday afternoon.",
        ),
    };
    println!("1 new news article: {}", article1.summarize());

}

