fn main() {
    // Creating a New String
    // String is actually implemented as a wrapper around a vector of bytes
    // the new function to create an instance of string
    let mut s = String::new();

    // string with a string literal initial content
    let s1 = "good afternoon".to_string();
    // same as
    let init_with = "good_night";
    let s3 = init_with.to_string();

    // creating a String from a string literal
    let s4 = String::from("good morning");
    
    // printing the strings we have so far
    println!("'{}', {}, {}, {}", s, s1, s3, s4);

    // strings are UTF-8 encoded
    // data included needs to be properly encoded
    // here are some examples from the book
    let hello1 = String::from("السلام عليكم");
    let hello2 = String::from("Dobrý den");
    let hello3 = String::from("Hello");
    let hello4 = String::from("שָׁלוֹם");
    let hello5 = String::from("नमस्ते");
    let hello6 = String::from("こんにちは");
    let hello7 = String::from("안녕하세요");
    let hello8 = String::from("你好");
    let hello9 = String::from("Olá");
    let hello10 = String::from("Здравствуйте");
    let hello11 = String::from("Hola");
    println!("{} {} {} {} {} {} {} {} {} {} {}", hello1, hello2, hello3, hello4, hello5, hello6, hello7, hello8, hello9, hello10, hello11);
   
    // Updating a String
    // Appending to a String with push_str and push
    // appending a string slice with push_str()
    s.push_str("good morning");
    println!("{}", s);
   
    let mut s5 = String::from("foo");
    let s6 = "bar";
    // push_str method takes a string slice 
    // it doesn't take ownership of the parameter
    s5.push_str(s6);
    // s6 (the parameter) can be used afterwards
    println!("{}, {}", s5, s6);

    // push() takes a single character (char) as a parameter and adds it to String
    let mut s7 = String::from("lo");
    s7.push('l');
    println!("{}", s7);

    // Concatenation with the + Operator or the format! Macro
    // (1) combining two existing strings with the + operator
    let s8 = String::from("Hello, ");
    let s9 = String::from("world!");
    // s8 is moved and can no longer be used
    let s10 = s8 + &s9; 
    println!("{}", s10);
    // the + operator uses the add method whose signature looks something like this
    // fn add(self, s: &str) -> String 
    
    // using the format! macro for more complicated string combining
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);
   
}
