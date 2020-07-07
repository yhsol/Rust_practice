extern crate rust_adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup()();
    assert_eq!(4, rust_adder::add_two(2));
}
