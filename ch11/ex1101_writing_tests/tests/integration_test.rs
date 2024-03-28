use ex1101_writing_tests as adder;

// making use of functionality in tests/common/mod.rs
mod common;

#[test]
// an integration test of a function in the adder crate
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}