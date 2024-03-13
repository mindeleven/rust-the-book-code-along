/// grouping related code in modules
/// example: a library crate that provides the functionality of a restaurant
/// "front of house" part of a restaurant is where customers are
/// "back of house" part is where the chefs and cooks work in the kitchen
/// to structure our crate in this way we organize its functions into nested modules

/// front of house module section
/// the entire module tree is rooted under the implicit module named crate
mod front_of_house {
    // front of house is (1) where customers are hosted
    // hosting nests inside front_of_house
    // hosting is the child of module front_of_house 
    // and that module front_of_house is the parent of module hosting
    mod hosting {
        fn _add_to_waitlist() {}

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

