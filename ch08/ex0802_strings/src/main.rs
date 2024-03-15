fn main() {
    // Creating a New String
    // String is actually implemented as a wrapper around a vector of bytes
    // the new function to create an instance of string
    let s = String::new();

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

}
