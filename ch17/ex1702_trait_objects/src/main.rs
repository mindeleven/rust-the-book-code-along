#![allow(dead_code)]
/// Using Trait Objects That Allow for Values of Different Types

/// defining a trait named Draw with one method named draw
pub trait Draw {
    fn draw(&self);
}

/// defining a struct named Screen that holds a vector named components
/// type of vector: Box<dyn Draw>
/// Box<dyn Draw> is a trait object
/// -> it’s a stand-in for any type inside a Box that implements the Draw trait
struct Screen {
    // Screen is defined to need values that can call the draw method
    // by specifying Box<dyn Draw> as the type of the values in the components vector
    components: Vec<Box<dyn Draw>>
}
/// define a method named run on the screen struct
/// run() will call the draw method on each of its components
impl Screen {
    // run doesn’t need to know what the concrete type of each component is
    // it just calls the draw method on the component
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

/// difference of this approach to defining a struct that uses a generic type parameter
/// -> a generic type parameter can only be substituted with one concrete type at a time
/// -> a trait objects allows for multiple concrete types to fill in at runtime


/// Implementing the Trait
/// adding some types that implement the Draw trait
#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("Yayyy!!! Today we're drawing a button: {:?}", self);
    }
}

#[derive(Debug)]
pub struct TextField {
    pub width: u32,
    pub height: u32,
    pub label: String,
    pub placeholder: String
}
impl Draw for TextField {
    fn draw(&self) {
        // code to actually draw a text field
        println!("Yayyy!!! Today we're drawing a text field: {:?}", self);
    }
}

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("Yayyy!!! Today we're drawing a select box: {:?}", self);
    }
}

fn main() {
    // creating a Screen instance
    // using trait objects to store values of different types that implement the same trait
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(TextField {
                width: 50,
                height: 10,
                label: String::from("Your name:"),
                placeholder: String::from("Please enter some text"),
            }),
        ],
    };
    
    screen.run();
}
