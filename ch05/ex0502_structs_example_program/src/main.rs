/// example program that calculates the area of a rectangle
/// let's start with using single variables and then refactor the program to use structs
/// the rectangles program will take the width and height of a rectangle 
/// specified in pixels and calculate the area of the rectangle

/// a struct to calculate a rectangle from
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/// definining an area method on the Rectangle struct itself
/// defining methods for a struct starts with an impl (implementation) block
/// all functions defined within an impl block are called associated functions
/// multiple impl blocks are allowed too
impl Rectangle {
    // we can define associated functions that don’t have self as their first parameter 
    // (and thus are not methods) 
    // functions that aren’t methods are often used for constructors
    // that will return a new instance of the struct
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // the &self in the signature is short for self: &Self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // a method can have the same name as one of the struct’s fields
    fn width(&self) -> bool {
        self.width > 0
    }

    // implementing a method on Rectangle that can take another Rectangle instance as a parameter
    fn can_hold(&self, other_one: &Rectangle) -> bool {
        self.width > other_one.width && self.height > other_one.height
    }
}

fn main() {
    // (1) calculating the area of a rectangle in a very simple way
    let width: u32 = 30;
    let height: u32 = 40;
    println!("The area of the rectangle is {} square pixel.", calculate_area(width, height));

    // (2) refactoring the calculate area functionality in a way that only one parameter needs to be passed
    // (to show that the parameters are related)
    let rect: (u32, u32) = (40, 50);
    // the disadvantage here is that tuples don’t name their elements 
    // and we don't know what each element is about
    println!("The area of the 2nd rectangle is {} square pixel.", calculate_area_by_tuple(rect));

    // now for something completely different: structs
    let rect2 = Rectangle {
        width: 130,
        height: 256,
    };
    println!("{:?}", rect2);
    // println!("The area of the 3rd rectangle is {} square pixel.", calculate_area_by_struct(&rect2));
    // calculating the area with the structs very own method instead
    // using method syntax to call the area method on the Rectangle instance
    println!("The area of the 4th rectangle is {} square pixel.", rect2.area());

    let scale: u32 = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 256,
    };
    // printing out debug information
    dbg!(&rect3);
    // println!("The area of the 4th rectangle is {} square pixel.", calculate_area_by_struct(&rect3));
    // calculating the area with the structs very own method instead
    // using method syntax to call the area method on the Rectangle instance
    println!("The area of the 4th rectangle is {} square pixel.", rect3.area());

    if rect3.width() {
        println!("This rectangle has a nonzero width of {} pixel", rect3.width);
    }
    
    // using the can_hold method
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));

    // calling an associated function that works like a constructor with the :: syntax
    let sq = Rectangle::square(3);
    println!("The area of the sq rectangle is {} square pixel.", sq.area());

}

fn calculate_area(width: u32, height: u32) -> u32 {
    width * height
}

fn calculate_area_by_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

/* no longer needed because the struct has its own area method now
fn calculate_area_by_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
*/