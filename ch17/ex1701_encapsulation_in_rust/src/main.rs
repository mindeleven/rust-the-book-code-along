#![allow(dead_code)]
/// Encapsulation that Hides Implementation Details
/// 
/// encapsulation means that the implementation details of an object aren’t accessible 
/// to code using that object
/// 
/// in Rust we can control encapsulation by using the pub keyword
/// example: defining a public struct AveragedCollection that contains two private fields
/// (1) a vector of i32 values
/// (2) the average of the values in the vector
/// the struct is marked pub so that other code can use it
#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    // the fields of the struct are public and can only be accessed or modified
    // via public methods
    // whenever an item is added to or removed from list the private update_average method 
    // is called to handle updating the average field as well
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    let mut my_collection = AveragedCollection {
        list: vec![],
        average: 0.0
    };
    
    my_collection.add(14);
    my_collection.add(5);
    my_collection.add(103);
    my_collection.add(34);
    my_collection.add(67);

    println!("My collection of i32s: {:?}", my_collection);
    println!("The average is: {}", my_collection.average());

    my_collection.remove();

    println!("The average (after remove()) is: {}", my_collection.average());

}
