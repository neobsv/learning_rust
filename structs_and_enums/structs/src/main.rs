// Structs

use std::borrow::Borrow;

struct User {
    active: bool, // fields
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    println!("Hello, world!");

    // Instance of a struct

    // note that the entire instance is mutable, rust doesn't allow parital
    // mutability.
    let mut user1 = User {
        active: true,
        username: String::from("xyz"),
        email: String::from("nobody@gmail.com"),
        sign_in_count: 1,
    };

    // Use the dot notation to get a specific value
    user1.email = String::from("somebody@gmail.com");
    user1.active = false;
    user1.sign_in_count = 2;
    user1.username = String::from("ssszz");

    let user2 = build_user(String::from("avc@bc.com"), String::from("xne"));

    println!("User2: {:?}", user2.email);

    // Creating struct instances from other instances using the struct update syntax
    let user3 = User {
        email: String::from("sssi@gmail.com"), //override any fields you want, then
        ..user2 // specifies that the value for all other fields must be copied from user2
    };

    println!("User2: {:?}", user2.sign_in_count);
    println!("User3: {:?}", user3.email);

    main2();

    main3();

    main4();
}

fn build_user(email: String, username: String) -> User {
    // User {
    //     active: true,
    //     username: username,
    //     email: email,
    //     sign_in_count: 1
    // }

    // Field init shorthand
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Tuple structs without named fields to create different types

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main2() {
    let black = Color(1, 2, 3);
    let origin = Point(3, 5, 6);

    println!("Tuple structs: {:#?} {:#?}", black.1, origin.0);

    // Unit like structs without any fields
    // They behave similar to the return value () , these are useful when you want
    // to implement a trait on some type but dont have any data you want to store in the struct

    struct AlwaysEqual;

    let sub = AlwaysEqual;

    sub.borrow();

    // Ownership of struct data

    // It is possible for struct fields to store the actual value (own) or just store references (not own)
    // Storing references requires the use of lifetimes, they ensure that the data referenced by a struct
    // is valid as long as the struct is.

    // This wont work, can't store refs in a struct without lifetimes:
    // struct User {
    //     active: bool,
    //     username: &str,
    //     email: &str,
    //     sign_in_count: u64,
    // }

    // let user1 = User {
    //     active: true,
    //     username: "someusername123",
    //     email: "someone@example.com",
    //     sign_in_count: 1,
    // };
}

// Example program using structs

#[derive(Debug)] // optionally print debugging info for each struct field
struct Rectangle {
    width: u32,
    height: u32,
}

fn main3() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("area: {:#?}", area(&rect1));

    // Adding functionality with derived traits

    // println!("rect1 is {}", rect1); //ERROR: doesn't implement std::fmt::Display

    // println!("rect1 is {:?}", rect1);
    // error[E0277]: `Rectangle` doesn't implement `Debug`
    // add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`

    println!("rect1 is {:?}", rect1);

    println!("pretty rect1 is {:#?}", rect1);

    // Another way to print out a value using the Debug format is to use the dbg! macro,
    //  takes ownership of an expression (as opposed to println!, which takes a reference)
    // dbg! prints to stderr vs println! to stdout

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    // refactor this code by turning the area function into an area method
    // defined on our Rectangle type, because it doesn't work with any other type
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height // accessing fields of a borrowed struct instance does not move the field values, which is why you often see borrows of structs
}

#[derive(Debug)]
struct Square {
    side: u32,
}

impl Square {
    // Everything within this impl block will be associated with the Square type

    // The &self is actually short for self: &Self. Within an impl block, the type Self is
    // an alias for the type that the impl block is for
    fn area(&self) -> u32 {
        // Methods can take ownership of self, borrow self immutably, as we’ve done here,
        // or borrow self mutably, just as they can any other parameter
        self.side * self.side
    }

    // Note that we can choose to give a method the same name as one of the struct’s fields
    fn side(&self) -> bool {
        self.side > 0
    }
    // Often, but not always, when we give a method the same name as a field we want it to only return the value
    // in the field and do nothing else. Methods like this are called getters

    // Methods with more params
    fn can_hold(&self, other: &Square) -> bool {
        self.side > other.side
    }

    // Associated functions
    // The Self keywords in the return type and in the body of the function are aliases for
    // the type that appears after the impl keyword, which in this case is Square.
    fn circle(size: f32) -> Self {
        Self {
            side: (size * 3.14 * 2.0) as u32,
        }
    }
}

// Each struct is allowed to have multiple impl blocks!!
// Useful for traits and generics
impl Square {
    fn say_side(&self) {
        println!("my side is {}", self.side);
    }
}

fn main4() {
    let sqr = Square { side: 30 };

    println!("area of the square: {}", sqr.area());
    println!("non zero side? {}", sqr.side());

    // Automatic referencing or dereferencing

    // Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called automatic referencing and dereferencing.
    // Calling methods is one of the few places in Rust that has this behavior.
    // Here’s how it works: when you call a method with object.something(),
    // Rust automatically adds in &, &mut, or * so object matches the signature of the method. In other words, the following are the same:
    // p1.distance(&p2);
    // (&p1).distance(&p2);

    // Methods with more params

    // We will implement another function called can_hold for Square, lets us know if we can
    // fit one Square into another

    let sqr2 = Square { side: 15 };

    println!("Can sqr hold sqr2? {}", sqr.can_hold(&sqr2));

    // Associated Functions

    // All functions defined within an impl block are called associated functions
    // because they’re associated with the type named after the impl
    // We can define associated functions that don’t have self

    let circle = Square::circle(10.1);

    println!("circle: {:#?}", circle);

    circle.say_side();
}
