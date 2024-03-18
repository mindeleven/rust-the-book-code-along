/// Removing Duplication by Extracting a Function
/// example: removing duplication of code without involving generic types
/// the goal here is to extract a function that replaces specific values with a placeholder
/// that represents multiple values

fn main() {
    // short program the finds the largest number in a list
    let number_list = vec![23, 76, 33, 44, 96, 15, 28];

    let mut largest_number = &number_list[0];

    for num in &number_list {
        if num > largest_number {
            largest_number = num;
        }
    }
   
    // duplicating code to find number in a second list
    let number_list = vec![88, 129, 654, 729, 12, 59, 109];

    let mut largest_number = &number_list[0];

    for num in &number_list {
        if num > largest_number {
            largest_number = num;
        }
    }

    println!("The largest number in the list is {}.", largest_number);

}
