#![allow(dead_code, unused_variables)]
/// an Arc is essentially the same thing as an Rc
/// it is used primarily when there is multi threading involved
/// an Arc guarantees a reference when used across multiple threads
use std::sync::{Arc, Weak};
use std::thread;

struct Owner {
    name: String,
    tools: Vec<Weak<Tool>>
}

struct Tool {
    owner: Arc<Owner>
}

fn main() {
    println!("ARC BEGIN");
    let brad = Arc::from(Owner { 
        name: "Brad".to_string(), tools: vec![]
    });
    
    // creating 10 threads with a loop
    for i in 0..10 {
        // defining a new refrence to brad in each iteration
        // otherwise we get a move error because `brad` has type `Arc<Owner>`
        // which does not implement the `Copy` trait
        let brad = Arc::clone(&brad);
        let child = thread::spawn(move || {
            let pliers = Arc::from(Tool { owner: Arc::clone(&brad) });
            let wrench = Arc::from(Tool { owner: Arc::clone(&brad) });

            println!("Pliers owner: {}", pliers.owner.name);

            println!("Thread {} END", i);
        });

        // getting the thread to run
        let res = child.join();
    }

    println!("ARC END");

    println!("cats like boxes");
}
