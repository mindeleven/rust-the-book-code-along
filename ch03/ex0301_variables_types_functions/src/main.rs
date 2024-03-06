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
    // (1) scalar data types
    // integer types 
    // no annotation, compiler infers signed integer
    let x = 10;
    // declaring an integer variable with type annotation
    let y: u64 = 5428;
    println!("x: {}, y: {}", x, y);
    // floating point number types 
    // two primitive types for floating-point numbers -> numbers with decimal points
    // default type is f64
    let z = 2.0; // compiler infers f64
    // f32 floating-point number with annotation
    let a: f32 = 3.0;
    println!("z: {}, a: {}", z, a);
    // numeric Operations
    let sum = 51 + 32; // addition
    let difference = 92.8 - 12.3; // subtraction
    let product = 13 * 13; // multiplication
    let quotient = 22.7 / 4.9; // division
    let truncated = -5 / 3; // results in -1
    let remainder = 43 % 5; // remainder, modulo
    println!("{} {} {} {} {} {}", sum, difference, product, quotient, truncated, remainder);
    // boolean types 
    // a boolean type has two possible values -> true and false
    let t = true;
    let f: bool = false; // boolean with annotation
    println!("{} is not {}", t, f);
    // character types 
    // the char type is Rust’s most primitive alphabetic type
    // char literals are specified with single quotes
    let c = 'z';
    let z: char = 'Z'; // character type with annotation
    let ninja = '🥷';
    println!("{} {} {}", c, z, ninja);
    // (2) compound data types 
    // dompound data types group multiple values into one type
    // (2.1) tuples
    // values with a variety of types
    let tup: (i32, u64, String, bool) = (3, 257, "foo".to_string(), false);
    println!("{:?}", tup);
    // accessing element wit dot notation
    let foo = tup.2;
    println!("{}", foo);
    // (2.2) arrays
    // every element of an array must have the same type
    let arr = [1, 2, 3, 4, 5, 6];
    println!("{:?}", arr);
    // array with type annotation using square brackets 
    // with the type of each element, a semicolon, the number of elements
    let ary: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", ary);
    // initializing an array that contains the same value for each element
    let ayr = [3; 5];
    println!("{:?}", ayr);
    // accessing array elements by indexing
    // trying to access an invalid value will result in a runtime error 
    let elem_a = arr[0];
    let elem_b = ary[1];
    let elem_c = ayr[2];
    println!("{} {} {}", elem_a, elem_b, elem_c);

}
