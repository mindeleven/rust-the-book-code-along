#![allow(dead_code)]
/// coding along with the youtube video 
/// "Box / Rc / Arc / Mutex - Smart Pointers Simplified - Rust" by Bocksdin Coding
/// @ https://www.youtube.com/watch?v=mNHdD69iLzA

/// let's begin with a box 
/// a box takes a piece of data and places it on the heap
/// especially helpful when you don't know the amount of memory you need
/// at the time you're writing and compiling your code
/// example: link list that can have a potentially infinite size
/// a linked list is a series of nodes that are linked with each other via memory reference
/// defining a linked list in Rust like this returns an error:
/* 
struct LinkListNode {
    value: i32,
    next: LinkListNode // "recursive type `LinkListNode` has infinite size"
}
*/
/// to avoid this we have to give it a defined size
/// Box<> has a defined size that varies by the data it is given
/// a Box is a reference with no guarantee that the value is still there if you try to access it
/// therefore we need to wrap it in an Option
#[derive(Debug)]
struct LinkListNode {
    value: i32,
    next: Option<Box<LinkListNode>>
}

/// let's define Rc<>
/// Rc stands for reference counter but it's more like a "reference copier"
/// an Rc creates a new reference to an existing peace of data
/// you can have two refrences that are owned separately and point both at the same structure
use std::rc::Rc;

/// example: one owner can have many tools
/// a tool can have only one owner
struct Owner {
    name: String,
    tools: Vec<Tool>
}

struct Tool {
    // to create a reference around brad we define owner as type Rc<>
    owner: Rc<Owner>
}

fn main() {
    // let brad = Owner { name: "Brad".to_string(), tools: vec![] };
    // to define the brad is a Rc we need to wrap it
    let brad = Rc::from(Owner { name: "Brad".to_string(), tools: vec![] });
    // let pliers = Tool { owner: brad }; // brad has been moved to pliers
    // let wrench = Tool { owner: brad}; // error: use of moved value: `brad`
    // the way around is is to create a reference around brad
    // creating a new reference with Rc::clone()
    let wrench = Tool { owner: Rc::clone(&brad) };
    // another way to do it:
    let pliers = Tool { owner: brad.clone() };


    println!("cats like boxes");
}
