use crate::data_types::dtmain1;

mod data_types;

// fn main3() {
//     /*
//         Cannot change the type of a mutable variable!
//     */
//     let mut spaces =  "    ";
//     spaces = spaces.len();
// }

fn main4() {
    // Can change the type of an immutable variable using shadowing

    let spaces = "    ";
    let spaces = spaces.len();
    print!("{spaces}");
}

fn main2() {
    // Shadowing in the inner scope, the value of x returns to the previous
    // value once the scope has finished executing.
    let x = 5;
    let x = x + 1;

    {
        let x = x + 2;
        println!("The value of x in the inner scope: {x}");
    }

    println!("The value of x is: {x}");
}

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    main2();

    // main3();

    main4();

    dtmain1();
}
