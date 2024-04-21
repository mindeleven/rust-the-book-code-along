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

struct Draft {
    content: String,
}

impl Draft {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    println!("Our post so far: {}", post.content);
}
