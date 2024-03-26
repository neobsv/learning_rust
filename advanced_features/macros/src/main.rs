// Macros

// The term macro refers to a family of features in Rust: declarative macros with macro_rules! and three kinds of procedural macros:
    // 1. Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
    // 2. Attribute-like macros that define custom attributes usable on any item
    // 3. Function-like macros that look like function calls but operate on the tokens specified as their argument

// The Difference Between Macros and Functions

// Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming.
// We discuss the derive attribute, which generates an implementation of various traits for you. We’ve also used the println! and vec! macros throughout the book. 
// All of these macros "expand to produce more code" than the code you’ve written manually.

// Metaprogramming is useful for reducing the amount of code you have to write and maintain, which is also one of the roles of functions, but macros have additional powers:

// Positives:
// 1. Macros can take a variable number of arguments but functions have a concrete signature. ex: println!("hello") with one argument or println!("hello {}", name) with two arguments.
// 2. Macros are expanded at compile time, macros are expanded before the compiler interprets the meaning of the code, so a macro can, for example, implement a trait on a given type. A function can’t, because it gets called at runtime and a trait needs to be implemented at compile time.

// Downsides:
// 1. Macro definitions are more complex than function definitions because you’re writing Rust code that writes Rust code. Harder to maintain.
// 2. You must define macros or bring them into scope before you call them in a file, as opposed to functions you can define anywhere and call anywhere.


fn main() {
    // Declarative Macros with macro_rules! for General Metaprogramming

    // The most widely used form of macros in Rust is the declarative macro. These are also sometimes referred to as “macros by example,” “macro_rules! macros,” or just plain “macros.”
    // Declarative macros allow you to write something similar to a Rust match expression. The value is the literal Rust source code passed to the macro; the patterns are compared with the structure of that source code; and the code associated with each pattern.

    // To define a macro, you use the macro_rules! construct. Let’s explore how to use macro_rules! by looking at how the vec! macro is defined.

    /* SIMPLIFIED VEC MACRO

        #[macro_export]
        macro_rules! vec {
            ( $( $x:expr ),* ) => {
                {
                    let mut temp_vec = Vec::new();
                    $(
                        temp_vec.push($x);
                    )*
                    temp_vec
                }
            };
        }

    */

    // Skipping macro syntax explanation, not important
    // When we call this macro with vec![1, 2, 3];, the code generated that replaces this macro call will be the following:

    let _v = {
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec.push(2);
        temp_vec.push(3);
        temp_vec
    };


}
