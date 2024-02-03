use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    // Read the comments in the packages_crates/src/main.rs file
    // that is used to explain the modules and paths created in this binary crate

    println!("Hello, world!");

    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
