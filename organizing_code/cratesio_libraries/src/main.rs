// Cargo and Crates.io


// Customizing builds with release profiles

// Release Profiles: are predefined and customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code.
    // Dev Profile: When you run cargo build it is defined with good defaults for development, fast compile times
    // Release Profile: When you run cargo build --release ,  it comes with optimizations like for the binary size, however with penalities like slow compile time

// In the project’s Cargo.toml file, by adding [profile.*] sections for any profile you want to customize, you override any subset of the default settings
// The opt-level setting controls the number of optimizations Rust will apply to your code; in the dev profile opt-level is set to 0 meaning no optimizations
// So release mode trades longer compile time for code that runs faster. That is why the default opt-level for the release profile is 3.
// Full list of profiles and optimizations: https://doc.rust-lang.org/cargo/reference/profiles.html

// Publishing a Crate to Crates.io

// You can publish your own packages in crates.io and the crate registry distributes it worldwide, open source.

// Making useful documentation comments

// Rust also has a particular kind of comment for documentation, known conveniently as a documentation comment, that will generate HTML documentation. 
// The HTML displays the contents of documentation comments for public API items intended for programmers interested in knowing how to use your crate as opposed to how your crate is implemented.

// Documentation comments use three slashes, ///, instead of two and support Markdown notation for formatting the text. 
// Place documentation comments just before the item they’re documenting. An add_one function in a crate named my_crate:

/* See example in lib.rs
/// Adds one to a given number
/// 
/// # Example:
/// 
/// ```
/// let arg = 5;
/// let answer = cratesio_libraries::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
/// 
pub fn add_one(x: i32) -> i32 {
    x + 1
}

*/

// We can generate the HTML documentation from this documentation comment by running cargo doc. 
// This command runs the rustdoc tool distributed with Rust and puts the generated HTML documentation in the target/doc directory.
// Running cargo doc --open will build the HTML for your current crate’s documentation (as well as the documentation for all of your crate’s dependencies) and open the result in a web browser

// Commonly Used Sections

// We used the # Examples Markdown heading in Listing 14-1 to create a section in the HTML with the title “Examples.” 
    // Panics: The scenarios in which the function being documented could panic.
    // Errors: If the function returns a Result, describing the kinds of errors that might occur and what conditions might cause those errors to be returned can be helpful to callers
    // Safety: If the function is `unsafe` to call, there should be a section explaining why the function is unsafe and covering the invariants that the function expects callers to uphold.

// Documentation Comments as Tests

// Example code blocks in your documentation comments can help demonstrate how to use your library, and doing so has an additional bonus: running cargo test will run the code examples in your documentation as tests! 
// Nothing is better than documentation with examples, but make sure they are well maintained, the doc tests catch that the test example and the code are out of sync with each other

// Commenting Contained Items

// The style of doc comment //! adds documentation to the item that contains the comments rather than to the items following the comments
// We use these doc comments inside the crate root file src/lib.rs by convention or inside a module to document the crate, this is used in to describe
// the title of the crate and the purpose and it is added to the beginning of lib.rs

// Documentation comments within items are useful for describing crates and modules especially. Use them to explain the overall purpose of the crate/module.

// Exporting a Convinient Public API with pub use

// The structure of your public API is a major consideration when publishing a crate
// Users may be annoyed at having to enter use my_crate::some_module::another_module::UsefulType; rather than use my_crate::UsefulType;.
// If the structure isn’t convenient for others to use from another library, you don’t have to rearrange your internal organization: instead, you can re-export items to make a public structure 
// that’s different from your private structure by using pub use. Re-exporting takes a public item in one location and makes it public in another location, as if it were defined in the other location instead.

// To explain this, a library named art has been created in this folder, it contians structs PrimaryColor, SecondaryColor inside a module named kinds, and a method named mix() inside a module named utils. art/src/lib.rs
// Another crate that depends on this library would need use statements that bring the items from art into scope, as shown in art/src/main.rs
// The author of the code had to figure out that PrimaryColor is in the kinds module and mix is in the utils module.
// To remove the internal organization from the public API, we can modify the art crate code to add pub use statements to re-export the items at the top level.

/* art/src/lib.rs

//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    // --snip--
}

pub mod utils {
    // --snip--
}

*/

// And now, the code in main.rs can simply import the structs PrimaryColor, SecondaryColor and the method mix() without worrying about the internal structure of the modules

/* art/src/main.rs

use art::mix;
use art::PrimaryColor;

fn main() {
    // --snip--
}

*/

// In cases where there are many nested modules, re-exporting the types at the top level with pub use can make a significant difference in the experience of people who use the crate. 
// NOTE: Another common use of pub use is to re-export definitions of a dependency in the current crate to make that crate's definitions part of your crate’s public API.

// Setting up a Crates.io Account

// Login to crates.io using your Github account and then obtain an API token, this can be found under crates.io/me
// Next, run `cargo login <API token>` ; subsequently credentials get stored locally at ~/.cargo/credentials

// Adding Metadata to a New Crate

// Before publishing, you’ll need to add some metadata in the [package] section of the crate’s Cargo.toml file.
// crate names on crates.io are allocated on a first-come, first-served basis. Once a crate name is taken, no one else can publish a crate with that name
// You also need to mention a description and a license identifier value under the license field.
// If you want to use a license that doesn’t appear in the SPDX, you need to place the text of that license in a file, include the file in your project, and then use 
// license-file to specify the name of that file instead of using the license key.
// Rust default: license = "MIT OR Apache-2.0"

// Publish to crates.io

// A publish is PERMANENT. The version can never be overwritten, and the code cannot be deleted. 
// One major goal of crates.io is to act as a permanent archive of code so that builds of all projects that depend on crates from crates.io will continue to work

// Publishing a new version of an Existing Crate

// When you’ve made changes to your crate and are ready to release a new version, you change the version value specified in your Cargo.toml file and republish. 
// Use the Semantic Versioning rules to decide what an appropriate next version number is.
// cargo publish to upload the new version

// Deprecating Versions from Crates.io with cargo yank

// Although you can’t remove previous versions of a crate, you can prevent any future projects from adding them as a new dependency. 
// This is useful when a crate version is broken for one reason or another. In such situations, Cargo supports yanking a crate version.

// A yank means that all projects with a Cargo.lock will not break, and any future Cargo.lock files generated will not use the yanked version

/*
cargo yank --vers 1.0.1
    Updating crates.io index
        Yank guessing_game@1.0.1
*/

// You can also undo a yank with the --undo flag

/*
$ cargo yank --vers 1.0.1 --undo
    Updating crates.io index
      Unyank guessing_game@1.0.1
*/

// IMPORTANT: A yank does not delete any code. It cannot, for example, delete accidentally uploaded secrets. If that happens, you must reset those secrets immediately.


fn main() {
    println!("Hello, world!");
}

// Cargo Workspaces

// Cargo offers a feature called workspaces that can help manage multiple related packages that are developed in tandem.
// A workspace is a set of packages that share the same Cargo.lock and output directory. Let’s make a project using a workspace—we’ll use trivial code so we can concentrate on the structure of the workspace.

// Example implemented in organizing_code/workspace_examples/add

// We’ll have a workspace containing a binary and two libraries. The binary, which will provide the main functionality, will depend on the two libraries. These three crates will be part of the same workspace.
// Next, in the add directory, we create the Cargo.toml file that will configure the entire workspace.
// It will start with a [workspace] section that will allow us to add members to the workspace by specifying the path to the package with our binary crate

/*

// Cargo.toml
[workspace]

members = [
    "adder",
]

$ cargo new adder
     Created binary (application) `adder` package

// Filesystem
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target

*/

// The workspace has one target directory at the top level that the compiled artifacts will be placed into; the adder package doesn’t have its own target directory. 
// Even if we were to run cargo build from inside the adder directory, the compiled artifacts would still end up in add/target rather than add/adder/target. 
// By sharing one target directory, the crates can avoid unnecessary rebuilding, meaning they don't each have their own target dirs so there is only one binary that is built at the root level.

// Creating the second package

/*

// Cargo.toml
[workspace]

members = [
    "adder",
    "add_one",
]

$ cargo new add_one --lib
     Created library `add_one` package

// Filesystem
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target

// add_one/src/lib.rs
pub fn add_one(x: i32) -> i32 {
    x + 1
}

*/

// Now we can have the adder package with our binary depend on the add_one package that has our library. First, we’ll need to add a path dependency on add_one to adder/Cargo.toml.

/*

// Filename: adder/Cargo.toml
[dependencies]
add_one = { path = "../add_one" }

*/

// Let’s use the add_one function (from the add_one crate) in the adder crate. Open the adder/src/main.rs file and add a use line at the top to bring the new add_one library crate into scope

/*

// Filename: adder/src/main.rs

use add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}

*/

// Build the workspace by running cargo build in the top-level add directory

/*
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.68s
*/

// To run the binary crate from the add directory, we can specify which package in the workspace we want to run by using the -p argument and the package name with cargo run:

/*
$ cargo run -p adder
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
*/

// Depending on External Packages in a Workspace

// IMPORTANT: Notice that the workspace has only one Cargo.lock file at the top level, rather than having a Cargo.lock in each crate’s directory. 
// IMPORTANT: This ensures that all crates are using the same version of all dependencies. If we add the rand package to the adder/Cargo.toml and add_one/Cargo.toml files, 
// Cargo will resolve both of those to one version of rand and record that in the one Cargo.lock.
// Making all crates in the workspace use the same dependencies means the crates will always be compatible with each other. Let’s add the rand crate to the [dependencies] section in the add_one/Cargo.toml

/*

// Filename: add_one/Cargo.toml

[dependencies]
rand = "0.8.5"

*/

// The top-level Cargo.lock now contains information about the dependency of add_one on rand. However, even though rand is used somewhere in the workspace, 
// we can’t use it in other crates in the workspace unless we add rand to their Cargo.toml files as well. 

/* IMPORTANT: 

Building the adder package will add rand to the list of dependencies for adder in Cargo.lock, but no additional copies of rand will be downloaded. 
Cargo has ensured that every crate in every package in the workspace using the rand package will be using the same version, saving us space and ensuring that the crates in the workspace will be compatible with each other.

*/

// Adding a Test to a Workspace

// Lets add a test of the add_one::add_one function within the add_one crate:

/*

// Filename: add_one/src/lib.rs

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}

*/

// Now run cargo test in the top-level add directory. Running cargo test in a workspace structured like this one will run the tests for all the crates in the workspace!
// We can also run tests for one particular crate in a workspace from the top-level directory by using the -p flag and specifying the name of the crate we want to test

/* IMPORTANT

If you publish the crates in the workspace to crates.io, each crate in the workspace will need to be published separately. Like cargo test. 
We can publish a particular crate in our workspace by using the -p flag and specifying the name of the crate we want to publish.

*/


// Installing binaries with cargo install

// The cargo install command allows you to install and use binary crates locally. This isn’t intended to replace system packages; 
// it’s meant to be a convenient way for Rust developers to install tools that others have shared on crates.io.
// A binary target is the runnable program that is created if the crate has a src/main.rs file or another file specified as a binary
// Usually, crates have information in the README file about whether a crate is a library, has a binary target, or both.

// All binaries installed with cargo install are stored in the installation root’s bin folder. 
// If you installed Rust using rustup.rs and don’t have any custom configurations, this directory will be $HOME/.cargo/bin.

/*

$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v13.0.0
  Downloaded 1 crate (243.3 KB) in 0.88s
  Installing ripgrep v13.0.0
--snip--
   Compiling ripgrep v13.0.0
    Finished release [optimized + debuginfo] target(s) in 3m 10s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v13.0.0` (executable `rg`)

*/

// Extending Cargo with Custom Commands

// Cargo is designed so you can extend it with new subcommands without having to modify Cargo. 
// If a binary in your $PATH is named cargo-something, you can run it as if it was a Cargo subcommand by running `cargo something`
// Custom commands like this are also listed when you run cargo --list.



