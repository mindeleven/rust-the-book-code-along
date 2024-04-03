/// all iterators implement a trait named Iterator 
/// this trait is defined in the standard library and looks like this:
/*
pub trait Iterator {
    type Item; // the Item type will be the type returned from the iterator

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
 */

fn main() {
    // iterators are lazy
    // they have no effect until you call methods that consume them
    let v1 = vec![1, 2, 3];
    // iterator is just stored in the v1_iter variable
    let v1_iter = v1.iter();
    // using each element of the iterator in a for loop
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // Methods that produce other iterators
    // Iterator adaptors are methods that don’t consume the iterator
    // they produce different iterators by changing some aspect of the original iterator
    let v2 = vec![1, 2, 3];
    // closure creates new iterator in which each item will be incremented by 1
    // collect() consumes the iterator 
    // and collects the resulting values into a collection data type
    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect();
    println!("{:?}", v3);
}

#[test]
// calling the next method on an iterator created from a vector
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
// Methods that consume the iterator
// methods that call next (like the sum method) are called consuming adaptors
// calling them uses up the iterator
// sum takes ownership of the iterator and iterates through the items by repeatedly calling next
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum(); // sum takes ownership of the iterator

    assert_eq!(total, 6);
}