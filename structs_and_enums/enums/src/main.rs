// Enums

// Set of related values where we can enumerate all possible variants,
// which is where enumeration gets its name.
//

use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("enums: {:?} {:?}", four, six);

    // Enum Values

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // To store actual ip address data, we can create a struct which holds
    // the ip address kind enum and then another string for the actual value.

    #[allow(dead_code)]
    #[derive(Debug)]
    struct IPAddr1 {
        kind: IpAddrKind,
        address: String,
    }

    let home = IPAddr1 {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IPAddr1 {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("struct with enum: {:?} {:?}", home, loopback);

    // Instead, we can put data directly into each enum variant
    #[derive(Debug)]
    enum IPAddr {
        V4(String),
        V6(String),
    }

    let home = IPAddr::V4(String::from("127.0.0.1"));
    let loopback = IPAddr::V6(String::from("::1"));

    println!("just enum: {:?} {:?}", home, loopback);

    // each kind of IP can have different types and amounts of associated data
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("enum different types: {:?} {:?}", home, loopback);

    // you can put any kind of data inside an enum variant: strings,
    // numeric types, or structs, for example. You can even include another enum!

    #[derive(Debug)]
    enum IpAddr2 {
        // enum of enums
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    let ex = IpAddr2::V4(Ipv4Addr::new(127, 0, 0, 1));
    let ex2 = IpAddr2::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    println!("addrs: {:?} {:?}", ex, ex2);

    // enum variant to store different amounts and types of values
    #[allow(dead_code)]
    enum Message {
        Quit,                       //unit struct
        Move { x: i32, y: i32 },    // normal struct
        Write(String),              // tuple struct
        ChangeColor(i32, i32, i32), // tuple struct
    }

    //  just as we’re able to define methods on structs using
    //  impl, we’re also able to define methods on enums
    impl Message {
        fn call(&self) {
            println!("inside call");
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    main2();

    main3();

    main4();

    main5();

    main6();
}

fn route(ip_kind: IpAddrKind) {
    println!("IP KIND: {:?}", ip_kind);
}

// Option Enum (to avoid null values)
// Interesting Read: Null References: The Billion Dollar Mistake

// An enum that can encode the concept of a value being present or absent. This enum is Option<T>
// <T> syntax: it is a generic type parameter meaning it can hold any data type
// enum Option<T> {
//     None,
//     Some(T)
// }

fn main2() {
    let some_num = Some(5);
    let some_char = Some('r');

    let no_num: Option<i32> = None; // Compiler can't infer type of 'None'

    dbg!(some_num, some_char, no_num);

    // the compiler won’t let us use an Option<T> value as if it were definitely a valid value
    // This code is invalid, can't add Option<T> with T

    // A regular i8 CANNOT be null, you can safely assume that the value isn’t null.
    // Only when we have an Option<i8> do we have to worry about possibly not having a value,
    // and the compiler will make sure we handle that case before using the value.
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; ERROR

    // You have to convert an Option<T> to a T before doing something with it

    // unwrap_or(<default value if the Option is None>)
    let sum = x + y.unwrap_or(0); // unwrap_or is one way of converting an Option<T> to T
    dbg!(sum);
}

// match Control Flow (like switch in other languages)

// match allows you to compare a value against a
// series of patterns and then execute code based on which pattern matches

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main3() {
    let c = Coin::Dime;
    println!("value of c: {:?}", value_in_cents(c));

    // The arms of a match statement can also have a block expression

    fn value_in_cents2(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    let c2 = Coin::Penny;
    dbg!(value_in_cents2(c2));

    // Patterns that bind to values

    // Consider this code, we can have a match statement with an arm which has a pattern that binds
    // to a possible struct/enum value of that arm, for example:

    #[allow(dead_code)]
    #[derive(Debug)]
    enum UsState {
        California,
        Massachusetts,
    }

    #[allow(dead_code)]
    enum CoinII {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    // We add a variable called state to the pattern that matches values of the variant Coin::Quarter.
    // When a Coin::Quarter matches, the state variable will bind to the value of that quarter’s state.
    // Then we can use state in the code for that arm, like so:

    fn value_in_cents3(coin: CoinII) -> u8 {
        match coin {
            CoinII::Penny => 1,
            CoinII::Nickel => 5,
            CoinII::Dime => 10,
            CoinII::Quarter(state) => {
                println!("The quarter is from: {:?}", state);
                25
            }
        }
    }

    let c3 = CoinII::Quarter(UsState::California);
    value_in_cents3(c3);
}

// Matching with Option<T>

fn main4() {
    // we can handle Option<T> using match
    // You’ll see this pattern a lot in Rust code: match against an enum,
    // bind a variable to the data inside, and then execute code based on it

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            None => None,
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    dbg!("five six none: ", five, six, none);
}

// Matches are exhaustive

fn main5() {
    // There’s one other aspect of match we need to discuss: the arms’ patterns must cover all possibilities.
    // Consider this version of our plus_one function, which has a bug and won’t compile:

    // error[E0004]: non-exhaustive patterns: `None` not covered
    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         Some(i) => Some(i + 1),
    //     }
    // }

    // This is because we did not handle the None case

    // Catch-all patterns and the _ placeholder

    // special actions for a few particular values, but for all other values take one default action
    // this is like the 'default' block in a switch statement in other languages

    let dice_roll = 9;
    match dice_roll {
        // Even though we haven’t listed all the possible values a u8 can have, this code compiles
        // This catch-all pattern meets the requirement that match must be exhaustive.
        // Note that we have to put the catch-all arm (other) last because the patterns are evaluated in order.
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(_num_spaces: u8) {}

    // Rust also has a pattern we can use when we want a catch-all but don’t want to use the value
    // in the catch-all pattern: _ is a special pattern that matches any value and does not bind to
    // that value. This tells Rust we aren’t going to use the value, so Rust won’t warn us about an
    // unused variable. The proper way to write the above code example is this:

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn reroll() {}

    // In order to make a no-op happen, when we match the default case _
    // we can use this: ()

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), //NO-OP, no code will run in this case
    }
}

// Concise control flow with if let

fn main6() {
    // The if let syntax lets you combine if and let into a less verbose thing that
    // can handle one match pattern while ignoring the rest

    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("The max is configured: {}", max),
        _ => (), // NO-OP when the Option<T> is None
    }

    // To shorten this, we can use 'if let' :

    if let Some(max) = config_max {
        // takes a pattern and an expression separated by an equal sign
        println!("The max is configured: {}", max);
    }

    // We can include an else with an if let, then the block of code that goes into else is the
    // same as the block of code which would go into the _ case for the match expression.

    #[allow(dead_code)]
    #[derive(Debug)]
    enum UsState {
        California,
        Massachusetts,
    }

    #[allow(dead_code)]
    enum CoinII {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    let coin = CoinII::Quarter(UsState::Massachusetts);

    let mut count = 0;
    // match coin {
    //     CoinII::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    if let CoinII::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("Number of non quarters: {}", count);
}
