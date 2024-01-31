fn main() {
    println!("Hello, world!");

    another_function();
    yet_another(5);

    // These are expressions
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = return_values();
    print!("{x}");

    plus_one(x);

}

fn another_function() {
    println!("another function");
}

fn yet_another(x: i32) {
    println!("value of x is: {x}");
}

fn return_values() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}