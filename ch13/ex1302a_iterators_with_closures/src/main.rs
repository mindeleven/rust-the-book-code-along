/// Using closures that capture their environment
/// 
/// commonly the closures we’ll specify as arguments to iterator adapters 
/// are closures that capture their environment
/// 
/// example: using the filter method that takes a closure
/// the closure gets an item from the iterator and returns a bool
/// -> if closure returns true, the value will be included in the iteration produced by filter
/// -> if the closure returns false, the value won’t be included
/// 

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String
}

/// using filter with a closure that captures the shoe_size variable from its environment 
/// to iterate over a collection of Shoe struct instances
/// -> it will return only shoes that are the specified size
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter() takes ownership of shoes and returns owned values

    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn main() {
    let my_shoes = vec![
        Shoe {
            size: 43,
            style: "trekking".to_string()
        },
    ];

    println!("Shoes in my list: {:?}", my_shoes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // cargo test filters_by_size -- --nocapture
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let shoes_in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            shoes_in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );

        dbg!(shoes_in_my_size);
    }
}