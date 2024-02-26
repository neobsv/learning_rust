use adder;

// Call the common module defined in the folder common/mod.rs
mod common;


#[test]
fn it_adds_two() {
    common::setup(); // use functions defined in common/mod.rs
    assert_eq!(4, adder::add_two(2));
}