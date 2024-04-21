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
        // ""
        // updating the content method on Post
        // the value returned from content should depend on the current state of the Post
        // so we’re going to have the Post delegate to a content method defined on its state
        // as_ref() is called on the Option because we want 
        // a reference to the value inside the Option
        // when we call as_ref() an Option<&Box<dyn State>> is returned
        // we know thet unwrap() will never panic
        // -> the methods on Post ensure that state will always contain a Some value 
        // with calling content() deref coercion will take effect on the & and the Box
        // -> content() will be called on the type that implements the State trait
        self.state.as_ref().unwrap().content(self)
    }

    // functionality to request a review of a post
    // -> requesting the review should change its state from Draft to PendingReview
    pub fn request_review(&mut self) {
        // the request_review method needs to take ownership of the state value
        // to consume the old state, 
        // we call the take method to take the Some value out of the state field 
        // and leave a None in its place
        // this lets us move the state value out of Post rather than borrowing it
        // then we’ll set the post’s state value to the result of this operation
        if let Some(s) = self.state.take() {
            // calling an internal request_review method on the current state of Post
            // this second request_review method consumes the current state 
            // and returns a new state
            self.state = Some(s.request_review())
        }

    }

    // the approve method will set state to the value that the current state says 
    // it should have when that state is approved,
    pub fn approve(&mut self) {
        // functionality same as above, just with callíng approve
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

}

// the State trait defines the behavior shared by different post states
// the state objects are Draft, PendingReview, and Published
// and they will all implement the State trait
trait State {
    // adding the request_review method to the State trait
    // the self: Box<Self> syntax means the method is only valid 
    // when called on a Box holding the type
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    // the approve method will set state to the value that the current state says 
    // it should have when that state is approved,
    fn approve(self: Box<Self>) -> Box<dyn State>;
    
    // Because the goal is to keep all these rules inside the structs that implement State, 
    // adding a method to call a content method on the value in state 
    // and passing the post instance as an argument
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }

}

// the Draft state is the state we want a post to start in
struct Draft {}

impl State for Draft {
    // the request_review method here returns a new, boxed instance of a new PendingReview
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // it returns itself
        // because when we request a review on a post already in the PendingReview state
        // it should stay in the PendingReview state
        self
    }
}

// the PendingReview struct represents the state when a post is waiting for a review
struct PendingReview {}
// PendingReview implements the request_review method but doesn’t do any transformations
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // it returns itself
        // because when we request a review on a post already in the PendingReview state
        // it should stay in the PendingReview state
        self
    }
    
    // when calling approve on PendingReview it 
    // returns a new boxed instance of the Published struct
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// adding a new struct that implements the Published state for State
struct Published {}
// Published implements the request_review method but doesn’t do any transformations
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

}

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    println!("Our post so far: {}", post.content);
}
