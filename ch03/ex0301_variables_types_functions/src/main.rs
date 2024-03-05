fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
fn main() {
    println!("------------------------------------------------------------");
    println!("Variables and Mutability");
    println!("------------------------------------------------------------");
    // variables are immutable by default
    // to make them mutable the mut keyword is needed
    let mut x = 5;
    println!("The value of x = {}", x);
    x = 6;
    println!("Now the value of x = {}", x);
    
    println!("------------------------------------------------------------");
    println!("Constants");
    println!("------------------------------------------------------------");
    // declaring constants using the const keyword
    // the type of the value must be annotated
    // naming convention for constants: all uppercase with underscores between words
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    println!("------------------------------------------------------------");
    println!("Shadowing");
    println!("------------------------------------------------------------");
    // shadowing: reassiging a variable with using the let keyword
    let x = 5;
    let x = x + 1;
    println!("value of x before new scope: {}", x);
    {
        let x = x * 2;
        println!("value of x inside inner scope: {}", x);
    }
    println!("value of x after new scope has ended: {}", x);
    // common example: string to integer conversion
    let spaces = "     ";
    let spaces = spaces.len();
    print!("type of spaces: ");
    print_type_of(&spaces);

    println!("------------------------------------------------------------");
    println!("Data Types");
    println!("------------------------------------------------------------");

}
