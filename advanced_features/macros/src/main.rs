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

    // Procedural Macros for Generating Code from Attributes

    // The second form of macros is the procedural macro, which acts more like a function. 
    // Procedural macros accept some code as an input, operate on that code, and produce some code as an output rather than matching against patterns.
    // The three kinds of procedural macros are custom derive, attribute-like, and function-like, and all work in a similar fashion.

    // When creating procedural macros, the definitions must reside in their own crate with a special crate type.
    
    /*
        use proc_macro;

        #[some_attribute]
        pub fn some_name(input: TokenStream) -> TokenStream {}
    */
    // The function that defines a procedural macro takes a TokenStream as an input and produces a TokenStream as an output. 
    // The TokenStream type is defined by the proc_macro crate that is included with Rust and represents a sequence of tokens.
    // The source code that the macro is operating on makes up the input TokenStream, and the code the macro produces is the output TokenStream. 
    // The function also has an attribute attached to it that specifies which kind of procedural macro we’re creating. We can have multiple kinds of procedural macros in the same crate.


    // How to Write a Custom derive Macro

    // Let’s create a crate named hello_macro that defines a trait named HelloMacro with one associated function named hello_macro()
    // Rather than making our users implement the HelloMacro trait for each of their types, we’ll provide a procedural macro so users can annotate their type with #[derive(HelloMacro)]
    // It will print Hello, Macro! My name is TypeName! where TypeName is the name of the type on which this trait has been defined.

    // The next step is to define the procedural macro, the naming convention is for a crate named foo, a custom derive procedural macro crate is called foo_derive. Let’s start a new crate called hello_macro_derive.


    use hello_macro::HelloMacro;
    use hello_macro_derive::HelloMacro;

    #[derive(HelloMacro)]
    struct Pancakes;

    Pancakes::hello_macro();

    // Refer hello_macro/src/lib.rs for the trait definition

    // Our two crates are tightly related, so we create the procedural macro crate within the directory of our hello_macro crate. If we change the trait definition in hello_macro, we’ll have to change the implementation of the procedural macro in hello_macro_derive as well.

    // Refer hello_macro_derive/src/lib.rs for the procedural macro definition and some comments

    // The hello_macro_derive function will be called when a user of our library specifies #[derive(HelloMacro)] on a type. 
    // This is possible because we’ve annotated the hello_macro_derive function here with proc_macro_derive and specified the name HelloMacro, which matches our trait name; this is the convention most procedural macros follow.

    // Some abstract syntax tree parsing logic has been explained which I'm going to omit for brevity.

    // The procedural macro to generates an implementation of our HelloMacro trait for the type the user annotated, which we can get by using #name. 
    // The trait implementation has the one function hello_macro, whose body contains the functionality we want to provide: printing Hello, Macro! My name is and then the name of the annotated type.

    // Attribute Like Macros

    // Attribute-like macros are similar to custom derive macros, but instead of generating code for the derive attribute, they allow you to create new attributes. 
    // They’re also more flexible: derive only works for structs and enums; attributes can be applied to other items as well, such as functions.

    /*  This is an example from a web framework like Rocket:

        #[route(GET, "/")]
        fn index() {)
    
    */

    // This #[route] attribute would be defined by the framework as a procedural macro.
    /*
        
        #[proc_macro_attribute]
        pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}

    */
    // Here, we have two parameters of type TokenStream: the GET, "/" part and the fn index(), other than that attribute like macros work the same way as custom macros,
    // you create a proc-macro crate and implement a function that generates the code you want.


    // Function Like Macros

    // Function-like macros define macros that look like function calls. Similarly to macro_rules! macros, they’re more flexible than functions; for example, they can take an unknown number of arguments. (varargs!)
    // macro_rules! macros can be defined only using the match-like syntax, However, function-like macros take a TokenStream parameter and their definition manipulates that TokenStream using Rust code as the other two types of procedural macros do.

    //  let sql = sql!(SELECT * FROM posts WHERE id=1);

    // This macro would parse the SQL statement inside it and check that it’s syntactically correct, which is much more complex processing than a macro_rules! macro can do.
    
    /*  Possible definition for the sql! macro:
        
        #[proc_macro]
        pub fn sql(input: TokenStream) -> TokenStream {}

    */

    // This definition is similar to the custom derive macro’s signature: we receive the tokens that are inside the parentheses and return the code we want to generate.

    // Fin.


}
