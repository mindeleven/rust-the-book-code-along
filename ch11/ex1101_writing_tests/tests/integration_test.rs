use ex1101_writing_tests as adder;

#[test]
// an integration test of a function in the adder crate
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}