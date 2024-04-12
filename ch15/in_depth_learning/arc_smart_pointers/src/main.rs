#![allow(dead_code, unused_variables)]
/// an Arc is essentially the same thing as an Rc
/// it is used primarily when there is multi threading involved
/// an Arc guarantees a reference when used across multiple threads
use std::sync::{Arc, Weak, Mutex};
use std::thread;

struct Owner {
    name: String,
    tools: Vec<Weak<Tool>>
}

struct Tool {
    owner: Arc<Owner>
}

/// a Mutex provides memory safety when accessing and mutating data across threats
/// example like above, just with Mutex
struct Owner2 {
    name: String,
    tools: Mutex<Vec<Weak<Tool2>>>
}

struct Tool2 {
    owner: Arc<Owner2>
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

    println!("ARC MUTEX BEGIN");
    let brad2 = Arc::from(Owner2 { 
        name: "Brad".to_string(), tools: Mutex::new(vec![])
    });
    let pliers2 = Arc::from(Tool2 { owner: Arc::clone(&brad2) });
    let wrench2 = Arc::from(Tool2 { owner: Arc::clone(&brad2) });

    // creating 10 threads with a loop
    for i in 0..10 {
        // defining a new refrence to brad in each iteration
        // otherwise we get a move error because `brad` has type `Arc<Owner>`
        // which does not implement the `Copy` trait
        let brad2 = Arc::clone(&brad2);
        // defining the tools for each iteration like the owner
        let pliers2 = Arc::clone(&pliers2);
        let wrench2 = Arc::clone(&wrench2);

        let child2 = thread::spawn(move || {
            // pushing the tools to brads tool vector
            // guard is the Mutex guard generated when accessing a Mutex
            // requesting access to the location in memory and locking it until we're done using it
            // locking it prevents other pieces of code from accessing the same memory
            // lock becomes unlocked once it goes out of scope
            // lock returns a result
            let mut guard = brad2.tools.lock().unwrap();
            guard.push(Arc::downgrade(&pliers2));
            guard.push(Arc::downgrade(&wrench2));
            // accessing first tool
            println!("Pliers owner: {}", guard[0].upgrade().unwrap().owner.name);

            println!("Thread {} END", i);
        });

        // getting the thread to run
        let res = child2.join();
    }
    println!("ARC MUTEX END");

    println!("cats like boxes");
}
