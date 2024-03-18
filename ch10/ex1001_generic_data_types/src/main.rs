/// continuing with the largest function example from the previous section
/// we've now two functions that both find the largest value in a slice
/// one takes a reference to a list of i32s, the other one to a list of chars

fn find_largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn find_largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![23, 76, 33, 44, 96, 15, 28];
    let largest_number = find_largest_i32(&number_list);
    println!("The largest number in the i32 list is {}.", largest_number);

    let char_list = vec!['c', 'h', 'l', 'q', 'p', 'y', 'r'];
    let largest_char = find_largest_char(&char_list);
    println!("The largest char in the char list is {}.", largest_char);
}
