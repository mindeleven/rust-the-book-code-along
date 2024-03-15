/// Storing Keys with Associated Values in Hash Maps
/// type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function
/// this function determines how it places these keys and values into memory
/// you can look up the stored data by using a key that can be of any type
/// hash maps are homogeneous
/// -> all of the keys must have the same type as each other
/// -> and all of the values must have the same type

/// importimg the HashMap from the collections portion of the standard library 
use std::collections::HashMap;

fn main() {
    // creating a hash map by using new and putting values in it with insert
    // example: scores V of players K is a game -> HashMap<player's name, player's score>
    let mut scores = HashMap::new();
    // with the first insert the compiler can infer the <K, V> of the HashMap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("White"), 100);

    println!("{:?}", scores);

    // accessing values in a HashMap for a given key
    let team_member = String::from("Blue");
    // get method of HashMap returns an Option<&V>
    // if there’s no value get will return None
    // here the Option gets handeled by calling copied to get an Option<i32>
    // rather than an Option<&i32>
    // then unwrap_or in case of None
    let score = scores.get(&team_member).copied().unwrap_or(0);
    println!("the score of {} is {} points", team_member, score);

    // iterating over each key/value pair in a hash map
    for (k, v) in scores {
        println!("the score of player \"{}\" is {} points", k, v);
    }

    // Hash Maps and Ownership
    // types that implement the Copy trait are copied into the hash map
    // owned values like String will be moved and the hash map will become the owner
    // example: hash map takes ownership of String values
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // field_name and field_value are invalid at this point
    println!("{:?}", map);



}
