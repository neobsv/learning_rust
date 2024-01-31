// Crates

// Crate is the smallest amount of code that a rust compiler considers at a time
// It can be a single file passed to rustc, that would also be a crate
// Crates can contain modules and the modules may be defined in other files which get compiled with the crate

// Binary Crate: compile to executables that can be run, has a main function

// Library Crate: dont have a main function and dont compile to executables, and generally, a "crate"
// means library crate.

// Crate Root: source file that rust compiler starts from and makes the root module

// Package: bundle of one or more crates that provide a set of functionality. It contains a Cargo.toml
// file that describes how to build those crates. A package can contain several binary crates and several library crates.

// Cargo follows the convention that src/main.rs is the crate root of a binary crate with the same
// name as the package. Likewise, for a library crate, the crate root is src/lib.rs

// If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a library, both with the same name as the package. 
// A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate


fn main() {
    println!("Hello, world!");

    // Module system:

    // Paths: Allow you to name the items (functions, modules, structs, enums, constants, traits) and the 'use' keyword brings a path into scope
    // The 'pub' keyword makes it public

    // Start from the crate root: when compiling a crate, the compiler looks in the crate root, src/main.rs
    // or src/lib.rs for code to compile

    // Declaring modules: In the CRATE ROOT file, you can declare new modules say a garden module with 'mod garden;'
    // This is usually placed in a file src/garden.rs which is a sibling of src/main.rs
    
    // compiler will look in the module's code in these places: 
    // 1) inline within curly braces after 'mod garden' if defined in the same file: (like this)
    mod garden { 

    }

    // 2) in the file src/garden.rs if you did 'mod garden;' in src/main.rs

    // 3) in the file src/garden/mod.rs if you did 'mod garden;' in src/main.rs

    // Declaring submodules: In any file OTHER THAN THE CRATE ROOT, you can declare submodules
    // Example, you can declare 'mod vegetables;' in src/garden.rs (the module created above).
    // compiler looks for the submodule's code in these places:

    // 1) Inline, directly following 'mod vegetables' within curly braces defined in src/garden.rs (the parent module)
    mod vegetables {

    }

    // 2) in the file src/garden/vegetables.rs if you wrote 'mod vegetables;' in src/garden.rs

    // 3) in the file src/garden/vegetables/mod.rs if you wrote 'mod vegetables;' in src/garden.rs

    // Paths to code in modules: Once a module is part of your crate, you can refer to code in that module 
    // from anywhere else in that same crate. For example, lets say you have a struct Asparagus inside the
    // submodule vegetables, in the module garden.
    // You can access Asparagus like: crate::garden::vegetables::Asparagus

    // Private vs Public: Code within a module is ALWAYS private from its parent modules by default.
    // To make a module public, declare it with 'pub mod' instead of just 'mod'
    // Everything inside a 'pub mod' is also private by default. To make a function inside a module public, you will need to prefix it with 'pub' as well.

    // The 'use' keyword: Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths.
    // Example, just write 'use crate::garden::vegetables::Asparagus;' at the top of src/main.rs and you'll be able to use
    // the Asparagus struct by just referring to it as 'Asparagus' in src/main.rs instead of its full path.


}


// Grouping Related Code in Modules

// Modules help organize code for readability and easy reuse, and help us control the privacy of items,
// all code within a module is private by default. We can choose to make them public.

// Example, write a library crate that provides the functionality of a restaurant. Just the skeleton code.
// In restaurants, some parts are referred to as front_of_house and some back_of_house, front is where the customers are
// and this is where hosts seat customers, servers take payments, and bartenders make drinks.
// Back is where the chefs cooks in the kitchen, dishwashers clean, and managers do admin work.

// To create a library crate run, 'cargo new restaurant --lib' (refer the top level directory for the restaurant folder)

// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

// Inside modules, we can place other modules, like 'hosting' and 'serving' in the above example.
// Modules can also hold definitions for other items, such as structs, enums, constants, traits and functions

// A module named 'crate' forms the root of the module tree. The crate root is the contents of rc/main.rs or src/lib.rs which is simply
// referred to as the module 'crate'.

// Here is the module tree for the above example,

// crate
//  └── front_of_house
//      ├── hosting // hosting is a child to front_of_house
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order // take_order and serve_order are siblings
//          ├── serve_order
//          └── take_payment

