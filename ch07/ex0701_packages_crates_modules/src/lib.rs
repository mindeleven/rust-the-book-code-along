/// grouping related code in modules
/// example: a library crate that provides the functionality of a restaurant
/// "front of house" part of a restaurant is where customers are
/// "back of house" part is where the chefs and cooks work in the kitchen
/// to structure our crate in this way we organize its functions into nested modules

/// creating a shortcut to a path with the use keyword
/// bringing the crate::front_of_house::hosting module into scope
/// use only creates the shortcut for the particular scope in which the use occurs
use crate::front_of_house::hosting;

/// if we want to bring two types of the same name into the same scope with use
/// we can specify as and a new local name, or alias, for the type
/* example:
use std::fmt::Result;
use std::io::Result as IoResult;
*/

/// front of house module section
/// the entire module tree is rooted under the implicit module named crate
mod front_of_house {
    // front of house is (1) where customers are hosted
    // hosting nests inside front_of_house
    // hosting is the child of module front_of_house 
    // and that module front_of_house is the parent of module hosting
    // inner parts of child modules’ code can be exposed to outer ancestor modules 
    // by using the pub keyword to make an item public
    pub mod hosting {
        // making the module public doesn’t make its contents public
        // the items within the module need to be made public as well
        // to expose them to the outside world
        pub fn add_to_waitlist() {}

        fn _seat_at_table() {}
    }
    
    // front of house is (2) where  servers take orders
    // orders are served and payment are taken
    // hosting and serving are siblings to each other
    mod serving {
        fn _take_order() {}

        fn _serve_order() {}

        fn _take_payment() {}
    }
}

pub fn eat_at_reastaurant() {
    // calling add_to_waitlist() with an absolute path
    // an absolute path is the full path starting from a crate root
    crate::front_of_house::hosting::add_to_waitlist();
    
    // calling add_to_waitlist() with an relative path
    // a relative path starts from the current module 
    // and uses self, super or an identifier in the current module
    // starting with a module name means that the path is relative
    // front_of_house::hosting::add_to_waitlist();
    // we brought the module into scope with use so we can shortcut the path
    hosting::add_to_waitlist();
}

pub fn eat_at_reastaurant_in_summer() {
    // order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // changing our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    
    // ordering appetizers from the enum list
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}

mod back_of_house {
    // pub before the struct definition makes the struct public
    // but not the fields inside the struct
    pub struct Breakfast {
        // each field can be made public or not on a case-by-case basis
        pub toast: String,
        _seasonal_fruit: String,
    }
    
    // making an enum public makes all of its variants public too
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        // because back_of_house::Breakfast has a private field 
        // the struct needs to provide a public associated function 
        // that constructs an instance of Breakfast (we’ll name it summer)
        // functionality allows a customer to pick the type of bread
        // but not the fruit that accompanies the meal
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn _fix_incorrect_order() {
        _cook_order();
        // starting relative path with super
        // it's like starting a filesystem path with the .. syntax
        // using super allow to reference an item that is in the parent module
        // the assumption is that the back_of_house module and the deliver_order function
        // will stay in the same relationship to each other and get moved together
        // if the crate’s module tree gets reorganized
        // otherwise "super" wouldn't work anymore to reference the function
        super::_deliver_order();
    }

    fn _cook_order() {}
}

fn _deliver_order() {}
