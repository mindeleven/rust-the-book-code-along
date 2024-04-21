#![allow(dead_code, unused_mut, unused_variables)]
/// starting out with a copy of ex1703_state_design_pattern
/// to make some modifications like encoding states and behavior as types
/// 
/// alternate approach: encoding the states into different types

pub struct Post {
    content: String,
}
impl Post {
    pub fn new() -> Draft {
        Draft {
            content: String::new(),
        }
    }
    
    pub fn content(&self) -> &str {
        &self.content
    }
    
}

pub struct Draft {
    content: String,
}
// draft posts don’t have the content method 
// so we can't print a draft post’s content accidentally 
impl Draft {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

// puttig a Draft post into a review state
// a PendingReviewPost gets created by calling request_review() on DraftPost 
pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    // approve method that turns a PendingReviewPost into a published Post
    // the only way to get a published Post instance is 
    // to call the approve method on a PendingReviewPost
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    // assert_eq!("", post.content()); // compiler error:
    // -> no method named `content` found for struct `Draft` in the current scope
    
    // creating a review state by turning a Draft into a PendingReviewPost
    let post = post.request_review();
    
    // calling the approve method to turn the PendingReviewPost into a published Post
    let post = post.approve();

    println!("Our post so far: {}", post.content);
}
