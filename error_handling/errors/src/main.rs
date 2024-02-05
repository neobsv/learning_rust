// Error Handling

// Rust groups errors into two major categories: recoverable and unrecoverable errors.

// Recoverable error: Something like a 'file not found' error where we just display the error and ask for a new path
// Unrecoverable error: Such as an invalid memory access in an array and so we want to stop the program

// Most languages don't distinguish between these and handle both in the same way, using exceptions.
// Rust doesn't have exceptions, it has Result<T, E> for recoverable errors and the panic! macro that stops execution when an unrecoverable error is encountered

use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    println!("Hello, world!");

    // Unrecoverable errors with panic!

    // Two ways a piece of code panics, one is by encountering an unrecoverable error (invalid memory access) and another is by explicitly calling the panic! macro.

    // These panics will print a failure message, unwind, clean up the stack, and quit.
    // By setting RUST_BACKTRACE=1, you can also have Rust display the call stack when a panic occurs to make it easier to track down the source of the panic.

    // When a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters
    // You can choose to immidiately abort, which ends the program without cleaning up.

    // NOTE: If you need to make the resulting binary as small as possible, you can switch from unwinding to aborting upon a panic by adding "panic = 'abort'" to the appropriate [profile] sections in your Cargo.toml file

    /* filename: Cargo.toml
    [profile.release]
    panic = 'abort'
    */

    // This is how you call panic,
    // panic!("crash and burn");

    let _v = vec![1, 2, 3];
    // _v[99]; // panic! : invalid memory access, This is called a buffer overread and can lead to security vulnerabilities
    // To protect your program from this sort of vulnerability, if you try to read an element at an index that doesn’t exist, Rust will stop execution and refuse to continue

    // Using a panic! backtrace

    // RUST_BACKTRACE=1 cargo run
    // A backtrace is a list of all the functions that have been called to get to this point.
    // Backtraces in Rust work as they do in other languages: the key to reading the backtrace is to start from the top and read until you see files you wrote. That’s the spot where the problem originated.

    // Debug symbols are enabled by default when using cargo build or cargo run without the --release flag

    // We’ll come back to panic! and when we should and should not use panic!

    main2();
}

// Recoverable Errors with Result

fn main2() {
    // Recoverable errors are those that you can easily interpret and respond to
    // Example: When you open a file and that operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process

    enum _Result<T, E> {
        Ok(T),
        Err(E),
    }

    // T and E are generic type parameters (discussed later), T is returned during success within the Ok() variant
    // E represents error and is returned within the Err() variant, lets see a function that fails,

    let _greeting_file_result = File::open("hello.txt");

    // T has been filled in by the implementation of File::open with the type of the success value, std::fs::File, which is a file handle. E used in the error value is std::io::Error.

    // IMPORTANT: In the case where File::open succeeds, the value in the variable greeting_file_result will be an instance of Ok that contains a file handle. In the case where it fails, the value in greeting_file_result will be an instance of Err that contains more information about the kind of error that happened.

    // Using match expressions to unpack a Result enum,

    // the Result enum and its variants have been brought into scope by the prelude, so we don’t need to specify Result::

    /*
    let _greeting_file = match greeting_file_result {
        Ok(file) => file, // returns std::fs::File the file handle
        Err(error) => panic!("Problem opening file: {:?}", error) // panics and aborts the program
    };
    */

    // Matching on different errors

    // we want to take different actions for different failure reasons: if File::open failed because the file doesn’t exist, we want to
    // create the file and return the handle to the new file. If File::open failed for any other reason we panic!

    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file, // success, file exists, return its handle
        Err(error) => match error.kind() {
            // nested match expression to switch on error.kind() to handle different kinds of errors
            ErrorKind::NotFound => match File::create("hello.txt") {
                // another nested match expression to create the file and read its Result
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // The first Err variant in the above example returns an io::Error and it has a kind() method which returns an enum io::ErrorKind and represents
    // the different types of errors that may result from an io operation. The variant we want to use is ErrorKind::NotFound, which indicates the file we’re trying to open doesn’t exist yet.
    // We write an inner match which creates a file when the ErrorKind::NotFound is encountered and the Result type returned by the File::create() method is run through another inner match expr.
    // Finally, we panic! if we cannot create this file. And for any other errors in error.kind() we panic! as well.

    // Alternatives to using match with Result<T, E>

    // match expressions are useful but primitive, the ideal way to handle Result<T, E> is closures (discussed later). Closures can be more concise than match to handle Result<T, E>.
    // Example using closures and unwrap_or_else(), this is cleaner to read,
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // NOTE: look up the unwrap_or_else method in the standard library documentation. Many more of these methods can clean up huge nested match expressions.

    // Shortcuts for panic on error: unwrap and expect

    // unwrap(): If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us.

    let _greeting_file = File::open("hello.txt").unwrap();

    // expect(): lets you choose the panic! error message and you can provide good messages for easy debugging

    let _greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    // NOTE: In production-quality code, most Rustaceans choose expect rather than unwrap and give more context about why the operation is expected to always succeed, for better debugging.

    // Propagating Errors

    // you can return the error to the calling code so that it can decide what to do. This is known as propagating the error and gives more control to the calling code, where there might be more information or logic
    // that dictates how the error should be handled than what you have available in the context of your code.

    let _result = read_username_from_file().unwrap_or(String::from("undefined"));

    // The code that calls this code will then handle getting either an Ok value that contains a username or an Err value that contains an io::Error.
    // It’s up to the calling code to decide what to do with those values. If the calling code gets an Err value, it could call panic! and crash the program, use a default username, or look up the username from somewhere other than a file.
    // We propagate all the success or error information upward for it to be handled appropriately.

    // Propagating errors using the ? operator
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username), // returns String if Ok()
        Err(e) => Err(e),      // returns io::Error
    }

    // NOTE: We chose the type io::Error because it is the type of the error value returned from both of the operations we’re calling in this function’s body that might fail: the File::open function and the read_to_string method.
}
