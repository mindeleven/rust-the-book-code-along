#![allow(dead_code, unused_mut, unused_variables)]
/// Implementing an Object-Oriented Design Pattern
/// 
/// the state pattern is an object-oriented design pattern
/// -> in this pattern we define a set of states a value can have internally
/// -> the states are represented by a set of state objects
/// -> the value’s (?) behavior changes based on its state
/// 
/// example: a blog post struct that can have the states "draft", "review" or "published"

/// defining a post
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // the fields of Post are private so a new post only can be created via new()
    pub fn new() -> Post {
        Post {
            // when a new Post is created its state field is set to a Some value
            // this Some() holds a Box that points to a new instance of the Draft struct
            // -> every new instance of Post will start out as a draft
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    
    // functionality to add text to the Post
    // it's implemented as a method rather than exposing the content field as public
    // the advantage of this is that we can control how the content field’s data is read
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    
    // implementing the content method
    pub fn content(&self) -> &str {
        // returning an empty string slice as long as the post in the draft state
        ""
    }
}

// the State trait defines the behavior shared by different post states
// the state objects are Draft, PendingReview, and Published
// and they will all implement the State trait
trait State {}

// the Draft state is the state we want a post to start in
struct Draft {}

impl State for Draft {}


fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    // assert_eq!("", post.content());

    println!("Our post so far: {}", post.content);
}
