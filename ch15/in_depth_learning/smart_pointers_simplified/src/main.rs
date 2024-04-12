#![allow(dead_code, unused_variables)]
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
use std::{
    rc::{
        Rc, 
        Weak
    }, 
    cell::RefCell
};

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

/// and now: the Weak version of Rc<>
/// Weak guarantees the reference but it does not guarantee the value
/// example same as above
/// BUT if we want the list of tools to be owned by brad we need to create a list of Weak tools
/// we don't use Rc because Rc can lead to a memory leak
/// AND we define the Tools vector as a RefCell so we can borrow a mutable version of it
struct Owner2 {
    name: String,
    tools: RefCell<Vec<Weak<Tool2>>>
}

struct Tool2 {
    // to create a reference around brad we define owner as type Rc<>
    owner: Rc<Owner2>
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

    // example with the Weak version of Rc
    let brad2 = Rc::from(Owner2 { 
        name: "Brad".to_string(), tools: RefCell::new(vec![]) 
    });
    let pliers2 = Rc::from(Tool2 { owner: Rc::clone(&brad2) });
    let wrench2 = Rc::from(Tool2 { owner: Rc::clone(&brad2) });
    // borrow a mutable version of brad.tools and push tools
    // to do this downgrade the tool from a strong reference to a weak reference
    brad2.tools.borrow_mut().push(Rc::downgrade(&pliers2));
    brad2.tools.borrow_mut().push(Rc::downgrade(&wrench2));

    println!("Pliers owner: {}", pliers2.owner.name);
    // printing a tool out
    // we have to upgrade the weak references to Rcs in order to have actual access to the value
    // upgrade() returns an Option so we gotta unwrap()
    println!("Brad pliers owner: {}", brad2.tools.borrow()[0].upgrade().unwrap().owner.name);
    
    // hidden message
    println!("cats like boxes");
}
