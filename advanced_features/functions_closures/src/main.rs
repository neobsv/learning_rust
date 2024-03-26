// Advanced Functions and Closures

// Function Pointers

// This technique is useful when you want to pass a function you’ve already defined rather than defining a new closure. 
// Functions coerce to the type fn (with a lowercase f), not to be confused with the Fn closure trait. 
// The fn type is called a function pointer.

// Passing functions with function pointers will allow you to use functions as arguments to other functions.

// The function do_twice() takes two parameters: a function pointer: fn takes an i32 parameter and returns an i32, and one i32 value. 
// The do_twice() function calls the function f twice, passing it the arg value, then adds the two function call results together.

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    // Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce), meaning you can always pass a function pointer as an argument for a function that expects a closure.
    // It’s best to write functions using a generic type and one of the closure traits so your functions can accept either fn (function pointers) or closures.
    // Where you would want to only accept fn and not closures is when interfacing with external code that doesn’t have closures: C functions can accept functions as arguments, but C doesn’t have closures.

    // To use the map function to turn a vector of numbers into a vector of strings, we could use a closure, like this:
    let list_of_numbers = vec![1, 2, 3];
    let _list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    // Or we could name a function as the argument to map instead of the closure, like this:
    let _list_of_strings_: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    // NOTE: We must use the fully qualified syntax that we talked about earlier in Advanced Traits, because there are multiple functions available named to_string

    // The name of each enum variant that we define also becomes an initializer function. We can use these initializer functions as function pointers that 
    // implement the closure traits, which means we can specify the initializer functions as arguments for methods that take closures:
    enum Status {
        Value(u32),
        _Stop,
    }

    let _list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    // Here we create Status::Value instances using each u32 value in the range that map is called on by using the initializer function of Status::Value. 
    // Some people prefer this style, and some people prefer to use closures.

    // Returning Closures

    // Closures are represented by traits, which means you can’t return closures directly beacuse the size of a closure is not known at compile time.

    /*  ERROR: Rust doesn’t know how much space it will need to store the closure
        fn returns_closure() -> dyn Fn(i32) -> i32 {
            |x| x + 1
        } 
    */

    // We saw a solution to this problem earlier. We can use a trait object:
    fn _returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }


}

