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
    mod garden {}

    // 2) in the file src/garden.rs if you did 'mod garden;' in src/main.rs

    // 3) in the file src/garden/mod.rs if you did 'mod garden;' in src/main.rs

    // Declaring submodules: In any file OTHER THAN THE CRATE ROOT, you can declare submodules
    // Example, you can declare 'mod vegetables;' in src/garden.rs (the module created above).
    // compiler looks for the submodule's code in these places:

    // 1) Inline, directly following 'mod vegetables' within curly braces defined in src/garden.rs (the parent module)
    mod vegetables {}

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

    main2();

    main3();

    main4();

    main5();
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

// Paths for Referring to an Item in the Module Tree

fn main2() {
    println!("Hello, world!");

    // A path can take two forms:
    // 1. absolute path - full path starting from crate root (crate::) and for external crates it begins with the crate name
    // 2. relative path - starts from the current module and uses self, super, or an identifier in the current module

    // Example, lets say we want to call the add_to_waitlist function from the mod tree above

    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    // Refer lib.rs for the actual example

    // However, for this to work we need to make the module hosting public, and the fn eat_at_restaurant public as well
    // Preference is in general is to specify absolute paths because it’s more likely we’ll want to move code definitions
    // and item calls independently of each other.

    // NOTE: in rust, all items (fn, methods, structs, enums, modules, constants) are private to the parent modules by default
    // Items in the parent module cant use the private items in their child modules. However, child modules can use items from their parent modules.
    // Hiding inner implementation details is the default.

    // Exposing paths with the 'pub' keyword

    // Even after marking the module hosting as 'pub', the contents that it holds are still private
    // The pub keyword on a module only lets code in its ancestor modules refer to it, not access its inner code
    // NOTE: making the module public doesn't make its contents public too!

    // If you plan to author library crates in rust then refer to these guidelines: https://rust-lang.github.io/api-guidelines/

    // A package can contain both a src/main.rs binary crate root as well as a src/lib.rs library crate root, and both crates will have the package name by default
    // This pattern of containing both a library and a binary crate will have just enough code in the binary crate to start an executable that calls code with the library crate

    // The module tree should be defined in src/lib.rs. Then, any public items can be used in the binary crate by starting paths with the name of the package.
    // The binary crate becomes a user of the library crate just like a completely external crate would use the library crate: it can only use the public API

    // Starting Relative Paths with 'super'

    // We can construct relative paths that begin in the parent module, rather than the current module, by using super at the start of the path.
    // Example shown below:
}

fn deliver_order() {}

mod back_of_house {
    #[allow(dead_code)]
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // this goes back to the crate root/ back_of_house module and looks for the deliver_order fn
    }

    fn cook_order() {}

    #[allow(dead_code)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

// Making Structs and Enums public

// Can use pub to make structs and enums public.

// Structs:

// For a struct, if we use pub before its definition, we make the struct public but all its fields will be private
// We can make each field pub , as shown above.
// Because the toast field in the back_of_house::Breakfast struct is public, in eat_at_restaurant we can write and read to the toast field using dot notation.

// IMPORTANT:
// Note that because back_of_house::Breakfast has a private field, the struct needs to provide a public associated function
// that constructs an instance of Breakfast (we’ve named it summer here). If Breakfast didn’t have such a function, we couldn’t
// create an instance of Breakfast in eat_at_restaurant because we couldn’t set the value of the private seasonal_fruit field in eat_at_restaurant.

// Enums:

// If we make an enum public then all the 'variants' (values) it contains become public too, unlike in structs.
// Since enum 'variants' are all related constants, they should be public by default

mod back_of_house_e {
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_e() {
    let order1 = back_of_house_e::Appetizer::Soup;
    let order2 = back_of_house_e::Appetizer::Salad;

    dbg!("order1: {:?} order2: {:?}", order1, order2);
}

fn main3() {
    eat_at_restaurant();
    eat_at_restaurant_e();
}

// The 'use' keyword (bringing paths into scope)

// The use keyword is used to create a shortcut to a path, meaning once specified, just the relative path
// is sufficient to access a particular path after what is mentioned in the use statement.

mod front_of_house2 {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// By adding use crate::front_of_house::hosting in the crate root, hosting is now a valid name in that scope,
// just as though the hosting module had been defined in the crate root.

use crate::front_of_house2::hosting;

// IDOMATIC WAY:  the idiomatic way to bring a function into scope with use. Bringing the function’s parent module into scope with use means we have to specify the parent module when calling the function.
// Specifying the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path

fn main4() {
    pub fn eat_at_restaurant_ii() {
        hosting::add_to_waitlist();
    }

    eat_at_restaurant_ii();

    // If the function is moved into another module, then the use statement won't work because the
    // scope changes.  Move the function into a new child module named customer, which is then a different scope
    // than the use statement, so the function body won’t compile.

    // mod customer1 {
    //     pub fn eat_at_restaurant_ii() {
    //         hosting::add_to_waitlist(); // ERROR: mod hosting is not found in mod customer1
    //     }
    // }

    // Brining a hashmap into scope in an idiomatic way
    use std::collections::HashMap;

    fn hash() {
        let mut map = HashMap::new();
        map.insert(1, 2);
    }

    hash();

    // Types with the same name from two different parent modules
    // As you can see, using the parent modules distinguishes the two Result types. If instead we specified use std::fmt::Result and use std::io::Result,
    // we’d have two Result types in the same scope and Rust wouldn’t know which one we meant when we used Result.

    /*
    use std::fmt;
    use std::io;

    fn function1() -> fmt::Result {
        // --snip--
    }

    fn function2() -> io::Result<()> {
        // --snip--
    }
    */

    // Another solution to the problem of bringing two types of the same name into the same
    // scope with use: after the path, we can specify as and a new local name, or alias, for the type.

    /*
    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function1() -> Result {
        // --snip--
    }

    fn function2() -> IoResult<()> {
        // --snip--
    }
    */

    // Re-exporting names with pub use

    // This technique is called re-exporting, because we are bringing an item into scope and also making that
    // item available for others to bring into their scope.

    /*
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    pub use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
    */

    // Before this change, external code would have to call the add_to_waitlist function by using the path restaurant::front_of_house::hosting::add_to_waitlist().
    // Now that this pub use has re-exported the hosting module from the root module,
    // external code can now use the path restaurant::hosting::add_to_waitlist() instead.

    // This is useful if you want users of your library to not worry about the internal structure of

    // Using External Packages

    // Adding rand as a dependency in Cargo.toml tells Cargo to download the rand package and any dependencies
    // from crates.io and make rand available to our project.

    // In Cargo.toml,
    // rand = "0.8.5"

    // to bring rand definitions into the scope of our package, we added a use line starting with the name of the crate,
    // rand, and listed the items we wanted to bring into scope.

    // NOTE:  The standard std library is also a crate that’s external to our package.

    // Using Nested paths to clean up

    // Don't list multiple items from the same crate or same module vertically,

    /*
    use std::cmp::Ordering;
    use std::io;
    */

    // Instead, do this:

    // use std::{cmp::Ordering, io};

    // Two use statements where one is a subpath of the other

    // Don't do this:
    // use std::io;
    // use std::io::Write;

    // Instead, do this:
    // use std::io::{self, Write};

    // The Glob Operator (*)

    // Bringing all public items defined in a path into scope, we can specify a path followed
    // by the glob (*) operator.

    // use std::collections::*;

    // CAUTION: Glob can make it harder to tell what names are in scope and where a name used in your program was defined
}

// Separating modules into different files

fn main5() {

    // When modules get large, you might want to move their definitions to a separate file to make the code easier to navigate

    // We can use the example above,
    // filename: src/lib.rs
    /*
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    pub use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
    */

    // Reorganize this file, leaving only the function definition in src/lib.rs and the src/front_of_house.rs

    // filename: src/lib.rs

    /*
    mod front_of_house;

    pub use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
    */

    // filename: src/front_of_house.rs

    /*
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
    */

    // NOTE: that you only need to load a file using a mod declaration once in your module tree

    // Next, we’ll extract the hosting module to its own file:

    // filename: src/lib.rs

    /*
    mod front_of_house;

    pub use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
    */

    // The process is a bit different because hosting is a child module of front_of_house, not of the root module. We’ll place the file for hosting in a new
    // directory that will be named for its ancestors in the module tree, in this case src/front_of_house/.

    // filename: src/front_of_house.rs

    /*
    pub mod hosting;
    */

    // filename: src/front_of_house/hosting.rs

    /*
    pub fn add_to_waitlist() {}
    */

    // If we instead put hosting.rs in the src directory, the compiler would expect the hosting.rs code to be in a hosting module
    // declared in the crate root, and not declared as a child of the front_of_house module.

    // Note that the pub use crate::front_of_house::hosting statement in src/lib.rs also hasn’t changed,
    // nor does use have any impact on what files are compiled as part of the crate.
}
