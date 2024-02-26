// Introduction

// Generics were introduced to tackle code duplication issues, they are placeholders for passing
// any concrete type to a struct, function or implementation. For example, Vec<T>, here T denotes a generic type and HashMap<K, V>.

// We can extract code to make functions to avoid code duplication and we can use generics by replacing two functions that only differ in the types of their parameters
// with just one function which has a generic type.

// Traits define behavior, in a generic way, and you can combine traits with generic types in order to constrain a function that uses generic types to
// accept only those that implement all the functions that is specified by the trait (behavior).

// Lifetimes: A variety of generics that tell the compiler about how references relate to each other, and how long references should be valid. This ensures that we have user
// defined behavior for references.

// Generics

// Removing code duplication by Extracting a Function

// Generics: replace a specific type with a placeholder which represents multiple types in order to reduce code duplication

use std::fmt::{Debug, Display};

fn main() {
    // Generics example:

    // Here is an example of code duplication, in order to find the largest in a vector,
    // there is duplicate code in this main function.

    /*
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
    */

    // However we can extract this code into a function named largest, in order to avoid duplication, function described below

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_non_generic(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_non_generic(&number_list);
    println!("The largest number is {}", result);

    // Steps taken so far:
    // Identify duplicate code.
    // Extract the duplicate code into the body of the function and specify the inputs and return values of that code in the function signature.
    // Update the two instances of duplicated code to call the function instead.

    // Suppose we wanted to extend this largest function to operate on vectors that store both char and int, then we need to write the functions largest_char() and largest_i32()
    // as mentioned below:

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // The code below uses a generic largest function that accepts any type so both Vec<i32> and Vec<char> can be operated upon:

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // We see that we had to specify that the 'PartialOrd' trait has to be implemented by T, otherwise it will throw an error

    /* ERROR: fn largest() without partial ord trait for T specified:

        fn largest<T>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest { // ERROR: the PartialOrd trait needs to be implemented by T, in order for it to use comparison operators
                largest = item;
            }
        }

        largest
        }

    */

    main2();

    main3();

    main4();

    main5();

    main6();
}

// When we use a parameter in the body of the function, we have to declare the parameter name in the signature so the compiler knows what that name means.
// Similarly, when we use a type parameter name in a function signature, we have to declare the type parameter name before we use it.

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// We read this definition as: the function largest is generic over some type T.
// The function has one parameter named list, which is a slice of values of type T.
// The function will return a reference to a value of the same type T.

fn largest_non_generic(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Generics in Struct Definitions

// We’ve used only one generic type to define Point<T>.
// This definition says that the Point<T> struct is generic over some type T, and the fields x and y are both that same type, whatever that type may be.

#[allow(dead_code)]
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main2() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("i32 struct: {:#?}", integer);
    println!("float struct: {:#?}", float);

    // If x and y are of different types, then this won't work, for example:

    // let wont_work = Point { x: 5, y: 4.0 };

    // To define a Point struct where x and y are both generics but could have different types, we can use multiple generic type parameters
    // For example, consider a struct Point<T, U>

    #[allow(dead_code)]
    #[derive(Debug)]
    struct PointII<T, U> {
        x: T,
        y: U,
    }

    let both_integer = PointII { x: 5, y: 7 };
    let both_float = PointII { x: 3.0, y: 2.0 };
    let int_and_float = PointII { x: 4, y: 1.0 };

    dbg!(
        "structs: {:#?} {:#?} {:#?}",
        both_integer,
        both_float,
        int_and_float
    );

    // Generics with Enum Definitions

    // Like structs, enums can also take generic parameters
    /*
    enum Option<T> {
        Some(T),
        None,
    }
    */
    // The Option<T> enum is generic over type T and has two variants:
    // Some, which holds one value of type T, and a None variant that doesn’t hold any value

    // Enums can use multiple generic types as well
    /*
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    */
    // The Result enum is generic over two types, T and E, and has two variants: Ok, which holds a value of type T, and Err, which holds a value of type E.
    // This definition makes it convenient to use the Result enum anywhere we have an operation that might succeed (return a value of some type T) or fail (return an error of some type E).

    // Generics in Method Definitions

    #[allow(dead_code)]
    #[derive(Debug)]
    struct PointIII<T> {
        x: T,
        y: T,
    }

    // In this method definition of method x(), we are using the return type &T. By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.
    // Methods written within an impl that declares the generic type will be defined on any instance of the type.
    impl<T> PointIII<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = PointIII { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    // We could implement methods only on Point<f32> instances rather than on Point<T> instances with any generic type.
    // Like, just define an impl block for a particular concrete type, instead of the generic type T.

    impl PointIII<f32> {
        fn distance(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p2 = PointIII { x: 1.0, y: 4.0 };
    println!("p.distance= {}", p2.distance());

    // Generic type parameters in a struct definition aren’t always the same as those you use in that same struct’s method signatures.
    // The generic types X1 and Y1 for the Point struct and X2 Y2 for the mixup method signature to make the example clearer.

    struct PointIV<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> PointIV<X1, Y1> {
        fn mixup<X2, Y2>(self, other: PointIV<X2, Y2>) -> PointIV<X1, Y2> {
            PointIV {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = PointIV { x: 5, y: 10.4 };
    let p2 = PointIV { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Here, the generic parameters X1 and Y1 are declared after impl because they go with the struct definition.
    // The generic parameters X2 and Y2 are declared after fn mixup, because they’re only relevant to the method.
}

// *** The main selling point of Rust ***
// Zero Cost Abstractions

// Performance of Code Using Generics

// Using generic types won't make your program run any slower than it would with concrete types.

/*

Rust accomplishes this by performing monomorphization of the code using generics at compile time.
Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.
In this process, the compiler does the opposite of the steps we used to create the generic function in Listing 10-5:
The compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.

*/

fn main3() {
    // Lets say that we have the Option<T> enum, during compile time, the rust compiler identifies two types being used, one is Option<i32> and the other Option<f64>

    let integer = Some(5);
    let float = Some(5.0);

    println!("int= {} float= {}", integer.unwrap(), float.unwrap());

    // Now it applies monomorphization, which is a flattening of the of the generic types, meaning, it creates new concrete types and replaces the generic type with it
    // So the actual code after compilation looks as if we had written the following code block:

    #[allow(dead_code)]
    #[derive(Debug)]
    enum OptionI32 {
        Some(i32),
        None,
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    enum OptionF64 {
        Some(f64),
        None,
    }

    let integer = OptionI32::Some(5);
    let float = OptionF64::Some(5.0);

    dbg!("int= {} float= {}", integer, float);
}

// Traits: Defining Shared Behavior

// NOTE: Trait is an Interface in Java, and an Abstract Base Class in C++.

// A trait defines functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way.
// We can use trait bounds to specify that a generic type can be any type that has certain behavior.

// A type’s behavior consists of the methods we can call on that type.
// Different types share the same behavior if we can call the same methods on all of those types.
// Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

fn main4() {
    // Lets create a trait named Summary

    /* implemented in lib.rs
    pub trait Summary {
        fn summarize(&self) -> String; // Semicolon after method signature, instead of an implementation block in {}
    }
    */

    // A trait can have multiple methods in its body: the method signatures are listed one per line and each line ends in a semicolon.

    // We declare a trait using the trait keyword and then the trait’s name, Summary.
    // We’ve also declared the trait as pub so that crates depending on this crate can make use of this trait too.
    // We declare the method signatures that describe the behaviors of the types that implement this trait, which in this case is fn summarize(&self) -> String

    // Each type implementing this trait must provide its own custom behavior for the body of the method.
    // The compiler will enforce that any type that has the Summary trait will have the method summarize() defined with this signature exactly.

    // Implementing a Trait on a Type

    // Implementing a trait on a type is similar to implementing regular methods.
    // The difference is that after impl, we put the trait name we want to implement, then use the for keyword, and then specify the name of the type we want to implement the trait for.

    // Lets say we have two types called NewsArticle and Tweet, and we want to create a method summarize() that will be common to both of them.
    // In fact, we want to enforce that the types Tweet and NewsArticle both implement summarize()

    /* implementation in lib.rs
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    */

    // The user must bring the trait into scope as well as the types:
    use gtl::{Summary, Tweet};

    let tweet = Tweet {
        username: String::from("onebsv"),
        content: String::from("some random musings"),
        reply: false,
        retweet: false,
    };

    println!("One new tweet: {}", tweet.summarize());

    // Other crates that depend on the gtl crate can also bring the Summary trait into scope to implement Summary on their own types
    // We can implement a trait on a type only if either the trait or the type is locally defined in our crate gtl
    // Example: We can implement Summary on Vec<T> in our gtl crate, because the trait Summary is local to our gtl crate
    // Example: We can’t implement the Display trait on Vec<T> within our gtl crate, because Display and Vec<T> are both defined in the standard library and aren’t local to our gtl crate.
    // NOTE: This is called coherence, and more specifically the orphan rule.

    // Default Implementations

    // It’s useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all methods on every type
    // The default implementation gets overridden as we implement the trait with custom implementations for each type.

    /*
    pub trait SummaryII {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }
    */

    use gtl::{NewsArticle, SummaryII};

    // To use a default implementation to summarize instances of NewsArticle, we specify an empty impl block with impl SummaryII for NewsArticle {}.

    let article = NewsArticle {
        headline: String::from("Nothing much, the world is as it was yesterday"),
        location: String::from("Bangalore, India"),
        author: String::from("onebsv"),
        content: String::from("Penguins once again are the best."),
    };

    println!("New article available! {}", article.summarize_ii());

    /*
    Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation.
    In this way, a trait can provide a lot of useful functionality and only require implementors to specify a small part of it.
    For example, we could define the Summary trait to have a summarize_author method whose implementation is required, and then define a
    summarize method that has a default implementation that calls the summarize_author method:
    */

    pub trait SummaryIII {
        fn summarize_author(&self) -> String;

        fn summarize_iii(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct TweetII {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl SummaryIII for TweetII {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }

        // NOTE: You CANNOT call the default implementation once you have overridden it with your own custom implementation!
        fn summarize_iii(&self) -> String {
            String::from("Overridden!")
        }
    }

    let tweet_ii = TweetII {
        username: String::from("onebsv"),
        content: String::from("some random musings"),
        reply: false,
        retweet: false,
    };

    println!("One new tweet: {}", tweet_ii.summarize_author());

    // Traits as Parameters

    // We specify the impl keyword and the trait name. This parameter accepts any type that implements the specified trait.
    // In the body of notify, we can call any methods on item that come from the Summary trait, such as summarize()
    // Code that calls the function with any other type, such as a String or an i32, won’t compile because those types don’t implement Summary.

    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    notify(&tweet);

    // Trait Bound Syntax

    // The '&impl Trait' syntax for the item passed into the notify function is actually sugar for a longer form known as trait bound
    pub fn notify_ii<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    notify_ii(&tweet);

    // For example, we can have two parameters that implement Summary. Doing so with the '&impl Trait' syntax looks like this, and in this case
    // the two parameters can have different types, like Summary and SummaryII
    #[allow(dead_code, unused_variables)]
    pub fn notify_iii(item1: &impl Summary, item2: &impl SummaryII) {}

    // If we want to force both parameters to have the same type, however, we must use a trait bound, like this:
    #[allow(dead_code, unused_variables)]
    pub fn notify_iv<T: Summary>(item1: &T, item2: &T) {}

    // Specifiying Multiple Trait Bounds with the + Syntax

    // We can also specify more than one trait bound, say we wanted to use display formatting along with summarize() on the item, we can do so by:
    #[allow(dead_code, unused_variables)]
    pub fn notify_v<T: Summary + Display>(item: &T) {}

    // Less preferred way but can use different generic types for each parameter this way:
    #[allow(dead_code, unused_variables)]
    pub fn notify_vi(item: &(impl Summary + Display)) {}

    // Clearer Trait Bounds with where Clauses

    // READABILITY IMPROVEMENT

    // Having multiple parameters with multiple trait bounds can make the function signature very hard to read, so rust has syntax to specify this information
    // after the function signature using the where clause

    // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

    #[allow(dead_code, unused_variables)]
    fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        0
    }

    // Returning Types That Implement Traits

    // Use the impl Trait syntax in the return position to return a value of some type

    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }

    returns_summarizable();

    // Example: The returns_summarizable function returns some type that implements the Summary trait without naming the concrete type

    // This is very useful in the context of Closures and Iterators, The impl Trait syntax lets you concisely specify that a function returns some type that implements the
    // Iterator trait without needing to write out a very long type.

    // NOTE: However, you can only use impl Trait if you’re returning a single type. For example, this code that returns either a NewsArticle or a Tweet with the return type specified as impl Summary wouldn’t work:

    /* ERROR: Can't return either or of two types even though they both implement the Summary trait.
    fn returns_summarizable(switch: bool) -> impl Summary {
        if switch {
            NewsArticle {
                headline: String::from("Penguins"),
                location: String::from("An"),
                author: String::from("Iceburgh"),
                content: String::from("xya."),
            }
        } else {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("o"),
                reply: false,
                retweet: false,
            }
        }
    }
    */

    // Using Trait Bounds to Conditionally Implement Methods

    //  The type Pair<T> in always implements the new function to return a new instance of Pair<T>. But in the next impl block,
    // Pair<T> only implements the cmp_display() method if its inner type T implements the PartialOrd trait that enables comparison and the Display trait that enables printing.

    #[allow(dead_code, unused_variables)]
    struct Pair<T> {
        x: T,
        y: T,
    }

    #[allow(dead_code, unused_variables)]
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    #[allow(dead_code, unused_variables)]
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    // Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library.
    // For example, the standard library implements the ToString trait on any type that implements the Display trait.
    // impl<T: Display> ToString for T {}

    // Because the standard library has this blanket implementation, we can call the to_string method defined by the ToString trait on any type that implements the Display trait.
    // For example, we can turn integers into their corresponding String values like this because integers implement Display:

    let s = 3.to_string();
    print!("s: {}", s);

    // NOTE: In dynamically typed languages, we would get an error at runtime if we called a method on a type which didn’t define the method.
    // But Rust moves these errors to compile time so we’re forced to fix the problems before our code is even able to run!
}

// Lifetimes

// Every reference in Rust has a lifetime, which is the scope for which that reference is valid.
// It is another kind of generic which ensures that a reference is valid as long as we want it to be.
// We must only annotate types when multiple types are possible. In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways.
// Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

fn main5() {
    // Preventing Dangling References with Lifetimes

    // The main aim of lifetimes is to prevent dangling references, meaning a reference to some variable/data that has already gone out of scope and has been deleted.
    /* Dangling Reference
    fn main() {
        let r;

        {
            let x = 5;
            r = &x;
        } -> end of scope for x

        println!("r: {}", r); -> r references x whose scope has already ended
    }
    */

    // If Rust allowed this code to work, r would be referencing memory that was deallocated when x went out of scope, and anything we tried to do with r wouldn’t work correctly.

    // The Borrow Checker

    // The borrow checker compares scopes f
    /*
        fn main() {
            let r;                // ---------+-- 'a
                                  //          |
            {                     //          |
                let x = 5;        // -+-- 'b  |
                r = &x;           //  |       |
            }                     // -+       |
                                  //          |
            println!("r: {}", r); //          |
        }
    */

    // The inner 'b block is much smaller than the outer 'a lifetime block. At compile time, Rust compares the size of the two lifetimes and sees that r has a
    // lifetime of 'a but that it refers to memory with a lifetime of 'b. Therefore, this program is rejected by the borrow checker.

    // Here is a valid code block with lifetimes annotated, this is valid because the lifetime of x 'b is greater than the lifetime of r 'a, therefore r can reference x.

    /*
    fn x() {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
    */

    // Generic Lifetimes in functions

    // This function will take two string slices and return a single string slice, longest of the two:
    // The function takes string slices, which are references, rather than strings, because we don’t want the longest function to take ownership of its parameters

    /* Won't compile because it can return either x or y, and the lifetime of the return value depends on which one is returned
        fn longest(x: &str, y: &str) -> &str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
    */

    fn xyz() {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }
    xyz();

    // Lifetime annotations don’t change how long any of the references live. Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
    // Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter.

    /* General syntax
        &i32        // a reference
        &'a i32     // a reference with an explicit lifetime
        &'a mut i32 // a mutable reference with an explicit lifetime
    */

    // Lifetime annotations in function signatures

    // We need to declare the generic lifetime parameters inside angle brackets between the function name and the parameter list, just as we did with generic type parameters.
    // We want the signature to express the following constraint: the returned reference will be valid as long as both the parameters are valid. This is the relationship between lifetimes of the parameters and the return value.

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    /*  IMPORTANT:
        The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at
        least as long as lifetime 'a. The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a.
        In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the values referred to by the function arguments

        When we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any values passed in or returned. We’re specifying that the borrow checker should reject any
        values that don’t adhere to these constraints. The lifetime annotations become part of the contract of the function, much like the types in the signature. Having function signatures contain the lifetime
        contract means the analysis the Rust compiler does can be simpler.
    */

    // Here are two examples, one positive and one negative using the same longest() function mentioned above

    fn xyz1() {
        let string1 = String::from("long string is long");

        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {}", result);
        }
    }

    xyz1();

    /* ERROR: An example that shows that the lifetime of the reference in result must be the smaller lifetime of the two arguments

        fn xyz2() {
            let string1 = String::from("long string is long");

            {
                let string2 = String::from("xyz");
                let result = longest(string1.as_str(), string2.as_str());
            } -> end of lifetime of result, set to lifetime of string2: smallest(string1, string2)

            println!("The longest string is {}", result);
        }

        This does not compile because the lifetime of result is set to the lifetime of string2 even though it is string1 that gets
        returned from the function.

        Explanation:
        As humans, we can look at this code and see that string1 is longer than string2 and therefore result will contain a reference to string1.
        Because string1 has not gone out of scope yet, a reference to string1 will still be valid for the println! statement. However, the compiler can’t see that
        the reference is valid in this case. We’ve told Rust that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of
        the references passed in. Therefore, the borrow checker disallows the code.

    */

    // Thinking in Terms of Lifetimes

    // If we chaned the function longest() to always return the first input parameter, then we can omit specifying the lifetime on the second parameter.

    #[allow(dead_code, unused_variables)]
    fn longest_ii<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }

    // When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.
    // If the reference returned does not refer to one of the parameters, it must refer to a value created within this function.
    // However, this would be a dangling reference because the value will go out of scope at the end of the function.

    /* ERROR: Dangling reference
        fn longest<'a>(x: &str, y: &str) -> &'a str {
            let result = String::from("really long string");
            result.as_str()
        }

        This implementation will fail to compile because the return value lifetime is not related to the lifetime of the parameters at all.
    */

    /* IMPORTANT NOTE:
        Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.
        Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create
        dangling pointers or otherwise violate memory safety.
    */

    // Lifetime Annotations in Struct Definitions

    // We can define structs to hold references, but in that case we would need to add a lifetime annotation on every reference in the struct’s definition.

    #[allow(dead_code, unused_variables)]
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    // We declare the name of the generic lifetime parameter inside angle brackets after the name of the struct so we can use the lifetime parameter in the body of the struct definition.
    // This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.

    let novel = String::from("Call me. Some years"); // --> needs to be valid until i is valid, doesn't go out of scope before i
    let first_sentence = novel.split('.').next().expect("Can't split");
    let i = ImportantExcerpt {
        part: first_sentence,
    }; // --> this holds a reference to novel, and is valid

    println!("the excrept: {:#?}", i);

    // Lifetime Elision

    // Some functions that you write do not need lifetime annotations, this is because the rust developers found that programmers were entering the same lifetime
    // annotations again and again. The rust devs then coded these predictable patterns into the compiler, and thus eliminated the need to enter lifetime annotaitons for simple functions.

    // The patterns programmed into Rust’s analysis of references are called the lifetime elision rules. These aren’t rules for programmers to follow;
    // They’re a set of particular cases that the compiler will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.

    // The elision rules don’t provide full inference, if there is ambiguity the compiler will throw an error requiring you to enter explicit lifetime annotations.
    // Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.

    // There are three rules which the compiler uses to figure out lifetimes, these rules apply to fn definitions as well as impl blocks:

    // 1. The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference.
    // Example, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32);
    // A function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

    // 2. The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

    // 3. The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.

    /*
        Examples:

        IMPLICIT:
        i) fn first_word(s: &str) -> &str {}

        In this case we use the first rule, and each parameter gets its own lifetime, like so:
        fn first_word<'a>(s: &'a str) -> &str {}

        Now, the second rule applies, because there is only one input parameter:
        fn first_word<'a>(s: &'a str) -> &'a str {}

        EXPLICIT:
        ii) fn longest(x: &str, y: &str) -> &str {}

        Apply the first rule, each input parameter gets its own lifetime:
        fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}

        Now, we notice that we can't apply either the second rule (there are multiple input params), or the third rule (there is no &self or &mut self).
        Therefore, we need to explicitly specify lifetime annotations for this function signature! The compiler will throw an error to let us know that we need to do this.

    */

    // Lifetime Annotations in Method Definitions

    // When we implement methods on a struct with lifetimes, we use the same syntax as that of generic type parameters.
    // Lifetime names for struct fields always need to be declared after the impl keyword and then used after the struct’s name, because those lifetimes are part of the struct’s type.

    /*  IMPORTANT:
        In method signatures inside the impl block, references might be tied to the lifetime of references in the struct’s fields, or they might be independent.
        In addition, the lifetime elision rules often make it so that lifetime annotations aren’t necessary in method signatures.
    */

    // Meaning, we need to annotate the impl blocks for structs that have explicit lifetime annotations for their references, then the methods inside those impl blocks need not have lifetimes explicitly mentioned if they follow the lifetime elision rules.

    // Example, function with one parameter inside impl block of ImportantExcerpt, lifetime annotations are specified on the impl block but not in the method signature because of elision rules

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }

    // Example, function with two parameters inside the impl block, again, lifetimes are specified on the impl block but not in the method signature here because it contains &self so it follows the third elision rule.

    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    let ss = String::from("abc");

    let il = ImportantExcerpt { part: &ss };
    println!("level= {}", il.level());
    println!("announce_return= {}", il.announce_and_return_part("hola"));

    // Special Lifetime: Static Lifetime

    // IMPORTANT:
    // 'static, which denotes that the affected reference can live for the entire duration of the program!

    let _s: &'static str = "I have a static lifetime.";
    // The text of this string is stored directly in the program’s binary, which is always available. Therefore, the lifetime of all string literals is 'static.

    // NOTE:
    // Use Static Lifetimes Sparingly: Most of the time, an error message suggesting the 'static lifetime results from attempting to create a dangling reference or a mismatch of the available lifetimes.
    // In such cases, the solution is fixing those problems, not specifying the 'static lifetime.ß
}

// Putting it All Together! : Generic Type Parameters, Trait Bounds, Lifetimes (GTL)

fn longest_with_an_announcement<'a, T>(
    // here we mention the lifetime annotation 'a along with the generic type T
    x: &'a str, // input lifetime: 'a
    y: &'a str, // input lifetime: 'a
    ann: T, // generic type without an input lifetime, This extra parameter will be printed using {}, which is why the Display trait bound is necessary.
) -> &'a str
where
    // where clause, to make the function signature more readable, by moving trait bounds outside of the angle brackets
    T: Display, // trait bounds
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main6() {
    longest_with_an_announcement(
        &String::from("abc"),
        &String::from("zzzcys"),
        String::from("hello world!"),
    );
}

// Summary:
// Generic type parameters let you apply the code to different types.
// Traits and trait bounds ensure that even though the types are generic, they’ll have the behavior the code needs.
// You learned how to use lifetime annotations to ensure that this flexible code won’t have any dangling references.
// And all of this analysis happens at compile time, which doesn’t affect runtime performance!
