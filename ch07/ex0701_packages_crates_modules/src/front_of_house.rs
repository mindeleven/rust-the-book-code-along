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