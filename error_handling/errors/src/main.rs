// Error Handling

// Rust groups errors into two major categories: recoverable and unrecoverable errors.

// Recoverable error: Something like a 'file not found' error where we just display the error and ask for a new path
// Unrecoverable error: Such as an invalid memory access in an array and so we want to stop the program

// Most languages don't distinguish between these and handle both in the same way, using exceptions.
// Rust doesn't have exceptions, it has Result<T, E> for recoverable errors and the panic! macro that stops execution when an unrecoverable error is encountered

use std::error::Error;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read};
use std::net::IpAddr;

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

    let _r = main3();
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

    // Error values that have the ? operator called on them go through the from function, defined in the From trait in the standard library,
    // which is used to convert values from one type into another. When the ? operator calls the from function, the error type received is converted into the error type defined in the return type of the current function.

    // We could change the read_username_from_file_ii function to return a custom error type named OurError that we define.
    // Then the ? operator calls in the body of read_username_from_file_ii will call 'from' and convert the error type to OurError.

    let _res = read_username_from_file_ii();

    // Method chaining and ? chaining (we can chain ? after each method that we chain)

    // NOTE: We return an Ok() value containing username when both File::open and read_to_string succeed rather than returning errors
    let _res = read_username_from_file_iii();

    // Using fs::read_to_string()
    // fs::read_to_string doesn’t give us the opportunity to explain all the error handling, so we did it the longer way first

    let _res = read_username_from_file_iv();

    // Where the ? operator can be used

    // Only in functions where the return type is compatible with the value ? is used on. This is because the ? operator is defined to perform an early return of a value out of the function.
    // In the examples above, the return type of the function has to be a Result so that it’s compatible with this return

    // let _greeting_file = File::open("hello.txt")?; ERROR: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`

    // The error message also mentioned that ? can be used with Option<T> values as well. As with using ? on Result, you can only use ? on Option in a function that returns an Option

    let _option = last_char_of_first_line("ccc");
    // This function returns Option<char> because it’s possible that there is a character there, but it’s also possible that there isn’t

    // NOTE: you can use the ? operator on a Result in a function that returns Result, and you can use the ? operator on an Option in a function that returns Option. The ? operator won’t automatically convert a Result to an Option or vice versa

    // About Result<(), Box<dyn Error>> and the main function returning a Result

    // main can also return a Result<(), E>. refer main3() -> Result<(), Box<dyn Error>>

    // In main3(), the Box<dyn Error> type is a trait object (discussed later).  For now, you can read Box<dyn Error> to mean “any kind of error.” Using ? on a Result value in a main function with the error type Box<dyn Error> is allowed,
    // because it allows any Err value to be returned early. This signature will continue to be correct even if more code that returns other errors is added to the body of main.

    // NOTE: When a main function returns a Result<(), E>, the executable will exit with a value of 0 if main returns Ok(()) and will exit with a nonzero value if main returns an Err value
    // C language also retuns an integer 0 on success and others on failure, rust also returns the same integers on failure to be compatible with this convention.

    // NOTE: The main function may return any types that implement the std::process::Termination trait, which contains a function report() that returns an ExitCode.
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

fn read_username_from_file_ii() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    // IMPORTANT:  If an error occurs, the ? operator will return early out of the whole function and give any Err value to the calling code
    username_file.read_to_string(&mut username)?;
    // If the value of the Result is an Ok, the value inside the Ok will get returned from this expression.
    // If the value is an Err, the Err will be returned from the whole function, and gets propagated to the calling code.
    Ok(username)
}

fn read_username_from_file_iii() -> Result<String, io::Error> {
    let mut username = String::new();

    // Method chaining with ? error handling in the middle for each function
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_iv() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    // If text is the empty string, this call to next will return None, in which case we use ? to stop and return None, otherwise 'next' will return a Some value containing a string slice of the first line in text
    text.lines().next()?.chars().last()
    // NOTE: This is an Option because it’s possible that the first line is the empty string, for example if text starts with a blank line but has characters on other lines, as in "\nhi".
    // However, if there is a last character on the first line, it will be returned in the Some variant.
    // The ? operator in the middle gives us a concise way to express this logic, allowing us to implement the function in one line
}

fn main3() -> Result<(), Box<dyn Error>> {
    let _greeting_file = File::open("hello.txt")?;

    Ok(())
}

// To panic! or Not to panic!

fn main4() {
    // When you should call panic! and when you should return Result:

    // When you call panic!, you are making the decision that the error is Unrecoverable, not even by the calling code
    // Result on the other hand you give options to the calling code, therefore returning Result is the defaut good choice.

    // However, in situations such as examples, prototype code, and tests, it’s more appropriate to write code that panics instead of returning a Result.

    // IMPORTANT: unwrap() and expect() methods will call panic! and they are not good ways to handle errors.

    // 1. Examples, prototypes and tests

    // the unwrap() and expect() methods are very handy when prototyping, examples, when you are unsure of how to handle errors, or do not want to, for the sake of clarity.
    // For tests, panic! is how a test is marked as a failure, and unwrap() and expect() call panic!, calling unwrap() or expect() is exactly what should happen.

    // 2. Cases in which you have more information than the compiler

    // It would also be appropriate to call unwrap() or expect() when you have some other logic that ensures the Result will have an Ok value, but the logic isn’t something the compiler understands.
    // Meaning, If you can ensure by manually inspecting the code that you’ll never have an Err variant, it’s perfectly acceptable to call unwrap(), and even better to document the reason you think you’ll never have an Err variant in the expect() text.
    // Example, We can see that 127.0.0.1 is a valid IP address, so it’s acceptable to use expect here. However, having a hardcoded, valid string doesn’t change the return type of the parse() method: we still get a Result value, and the compiler will still
    // make us handle the Result as if the Err variant is a possibility because the compiler isn’t smart enough to see that this string is always a valid IP address. If the IP address string came from a user rather than being hardcoded into the program and
    // therefore did have a possibility of failure, we’d definitely want to handle the Result in a more robust way instead

    let _home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");

    // Guidelines for Error Handling

    // Bad state: when some assumption, guarantee, contract or invariant has been broken.

    // The bad state is something that is unexpected, as opposed to something that will likely happen occasionally, like a user entering data in the wrong format.
    // Your code after this point needs to RELY on not being in this bad state, rather than checking for the problem at every step.
    // There’s not a good way to encode this information in the types you use. (discussed later)

    // Either return the error so that the user can decide what to do at the top level, OR call panic!() if it is unsafe to continue. panic!() is appropriate when you have external code that is out of your control
    // and it returns an invalid state that you cannot fix.

    //  When failure is expected, it’s more appropriate to return a Result than to make a panic! call

    // panic! should be called after input data validation if the values aren't valid or even dangerous. Ex: the standard library panics if you attempt out of bounds memory access, because this could be a security risk
    // Functions have 'contracts' and if the input to the function isn't valid, it violates this contract and it should panic since there is no reasonable way that the calling code can recover.

    // If your function has a particular type as a parameter, you can proceed with your code’s logic knowing that the compiler has already ensured you have a valid value
    // Code trying to pass nothing to your function won’t even compile, so your function doesn’t have to check for that case at runtime. Another example is using an unsigned integer type such as u32, which ensures the parameter is never negative.

    // Creating Custom Types for Validation

    // In the guessing game example, we never validated that the 'guess' input was between 1 to 100, we just ensured that the number was positive
    // One way to do this would be to parse the guess as an i32 instead of only a u32 to allow potentially negative numbers, and then add a check for the number being in range

    /*
    loop {
        // --snip--

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        // --snip--
    }
    */

    // However, this is not an ideal solution: if it was absolutely critical that the program only operated on values between 1 and 100, and it had many functions with this requirement, having a check like this in every function would be tedious and impact performance!

    // The solution is to create a new type called 'Guess' which moves the above validations into the constructor of this type
    // We implement a method named value that borrows self, doesn’t have any other parameters, and returns an i32. This kind of method is sometimes called a getter, because its purpose is to get some data from PRIVATE fields and return it
    // It’s important that the value field be private so code using the Guess struct is not allowed to set value directly.

    /*
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
    */
}
