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

    println!("scores: {:?}", scores);

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

    // Updating a Hash Map
    // each unique key can only have one value associated with it at a time
    // three different ways of handling the change of data in a hash map
    // (1) Overwriting a Value
    // using scores from above
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("White"), 66);
    println!("scores of team 2: {:?}", scores2); // {"White": 66}
    scores2.insert(String::from("White"), 2);
    println!("scores of team 2: {:?}", scores2); // {"White": 2}

    // (2) Adding a Key and Value Only If a Key Isn’t Present
    // Checking if a key exists with entry (takes the key you want to check as a parameter)
    // or_insert() on Entry is defined to return a mutable reference 
    // to the value for the corresponding Entry key if that key exists
    // if not it inserts the parameter as the new value for this key
    // and returns a mutable reference to the new value
    scores2.entry(String::from("White")).or_insert(100); // doesn't get overwritten
    scores2.entry(String::from("Blue")).or_insert(100);
    println!("scores of team 2: {:?}", scores2); // {"White": 2, "Blue": 100}

    // (3) Updating a Value Based on the Old Value
    let text = "whisper our dried voices when we whisper our together";
    let mut whisper_map = HashMap::new();
    // split_whitespace() returns an iterator over sub-slices
    for word in text.split_whitespace() {
        // or_insert() returns a mutable reference (&mut V) to the value for the specified key
        // we store that mutable reference in the count variable
        let count = whisper_map.entry(word).or_insert(0);
        // so in order to assign to that value we must first dereference count
        *count += 1;
    }
    println!("{:?}", whisper_map);


}
