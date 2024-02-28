// Building a grep clone, a command line program

// NOTE: Read main.rs first, then read lib.rs

// We will be building a simplified version of grep, called minigrep
// The search tool grep (globally search a regular expression and print) searches for a string in a text file,
// find lines that contain that string and print it out.

// The first task is to make minigrep accept its two command line arguments: the file path and a string to search for
// $ cargo run -- searchstring example-filename.txt


// We will build this incrementally, the initial code will be in other functions and the final code will be in main()

// Reading Argument Values

// We’ll need the std::env::args function provided in Rust’s standard library. This function returns an iterator of the command line arguments passed to minigrep. 
// You need to call the collect() method on iterators to get a vector of all the elems it holds.

use std::{env, process, fs, error::Error};

#[allow(dead_code, unused_variables)]
fn main1() {

    // Note that std::env::args will panic if any argument contains invalid Unicode. 
    // If your program needs to accept arguments containing invalid Unicode, use std::env::args_os instead -> This produces a vector of OsString, and this is more complicated to handle.
    let args: Vec<String> = env::args().collect();
    
    // dbg!("Args: {}", args.clone());

    // Saving the arg values in Variables

    // The program’s name takes up the first value in the vector at args[0], so we’re starting arguments at index 1
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    // Add functionality to read the file specified in the file_path argument.
    // The statement fs::read_to_string takes the file_path, opens that file, and returns a std::io::Result<String> of the file’s contents.

    let contents = fs::read_to_string(file_path)
                            .expect("Should have been able to read the file!");

    println!("With text:\n{contents}");

    /*
        At the moment, the main function has multiple responsibilities: generally, functions are clearer and easier to maintain if each function is responsible for only one idea. 
        The other problem is that we’re not handling errors as well as we could. The program is still small, so these flaws aren’t a big problem, but as the program grows, it will be harder to fix them cleanly.
    */

}

// Refactoring to Improve Modularity and Error Handling

// We’ll fix four problems that have to do with the program’s structure and how it’s handling potential errors

    // 1. Main function performs two tasks, parses args and also reads the file, we will refactor this.
    // 2. Group the configuration variables like query and file_path into a struct, to make their purpose clear.
    // 3. Using expect(), regardless of the situation, we’d print the same error message for everything. The file may not exist, or permissions issue or something else, we need to make this clear.
    // 4. Using expect() repeatedly to handle different errors, and if the user runs our program without specifying enough arguments, they’ll get an index out of bounds error that doesn’t clearly explain the problem.

// Separation of Concerns for Binary Projects

// Allocating responsibility for multiple tasks to the main fn is common to many binary project, but this is bad practice,
// need to separate concerns when the main function gets too large
    // 1. Split your program into main.rs and lib.rs
    // 2. As long as the command line parsing logic is small it can stay in main.rs, otherwise extract it to lib.rs


// The responsibilities that remain in the main fn should be limited to the following
    // 1. Calling command line parsing with arg values
    // 2. Setting up any other configuration
    // 3. Calling a run fn in lib.rs
    // 4. Handling the error if run returns an error


// You can’t test the main function directly, this structure lets you test all of your program’s logic by moving it into functions in lib.rs

// Extracting the argument parser

// we will extract the function that reads the args from the args vector into two variables

#[allow(dead_code, unused_variables)]
fn main2() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config_x(&args);

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

}

#[allow(dead_code, unused_variables)]
fn parse_config_x(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}

// Grouping configuration values

// At the moment, we’re returning a tuple, but then we immediately break that tuple into individual parts again, this is not the right abstraction
// We will use a struct named Config in order to group the variables query and file_path, so that we don't return two string refs

#[allow(dead_code, unused_variables)]
fn main3() {
    let args: Vec<String> = env::args().collect();

    let config = parse_configs(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

#[allow(dead_code, unused_variables)]
struct ConfigS {
    query: String,
    file_path: String,
}

#[allow(dead_code, unused_variables)]
fn parse_configs(args: &[String]) -> ConfigS {
    let query = args[1].clone();
    let file_path = args[2].clone();

    ConfigS { query, file_path }
}

// Inefficient route is to call the clone method on the values. This will make a full copy of the data for the Config instance to own, 
// which takes more time and memory than storing a reference to the string data.

// NOTE: Now our code more clearly conveys that query and file_path are related and that their purpose is to configure how the program will work

// Creating a Constructor for Config

// Now that the purpose of the parse_config function is to create a Config instance, we can change parse_config from a plain function to a function named new that is associated with the Config struct. 
// Making this change will make the code more idiomatic


// Fixing the Error Handling

#[allow(dead_code, unused_variables)]
fn main4() {
    let args: Vec<String> = env::args().collect();

    /* IMPORTANT:
    Using unwrap_or_else allows us to define some custom, non-panic! error handling. If the Result is an Ok value, this method’s behavior 
    is similar to unwrap: it returns the inner value Ok is wrapping. However, if the value is an Err value, this method calls the code in the closure, 
    which is an anonymous function we define and pass as an argument to unwrap_or_else.

    unwrap_or_else() will pass the inner value of the Err, which in this case is the static string "not enough arguments", 
    to our closure in the argument err that appears between the vertical pipes. The code in the closure can then use the err value when it runs.
    */

    let config = ConfigK::new(&args).unwrap_or_else(|err| { 
        println!("Problem parsing arguments: {}", err);
        // Take the responsibility of exiting the command line tool with a nonzero error code away from panic!
        process::exit(1); 
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

}

#[allow(dead_code, unused_variables)]
struct ConfigK {
    query: String,
    file_path: String,
}

#[allow(dead_code, unused_variables)]
impl ConfigK {

    // Returning a Result enum instead of calling panic!, so that it can fail gracefully
    #[allow(dead_code, unused_variables)]
    fn new(args: &[String]) -> Result<ConfigK, &'static str> {

        // Bounds checking for the vector, because we don't want the program to fail with a strange error message.
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(ConfigK { query, file_path })
    }
}

// Refactoring done, error handling and putting related values in a struct, using Result instead of panic!


// Extracting Logic from main

// We’ll extract a function named run that will hold all the logic currently in the main function that isn’t involved with setting up configuration or handling errors.

#[allow(dead_code, unused_variables)]
fn main5() {

    let args: Vec<String> = env::args().collect();

    let config = ConfigX::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);


    /* IMPORTANT:
        We use if let rather than unwrap_or_else to check whether run returns an Err value and call process::exit(1) if it does. 
        The run function doesn’t return a value that we want to unwrap in the same way that Config::build returns the Config instance. 
        Because run returns () in the success case, we only care about detecting an error, so we don’t need unwrap_or_else to return the unwrapped value, which would only be ().
    */
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

// The run function will return a Result<T, E> when something goes wrong. 
// This will let us further consolidate the logic around handling errors into main in a user-friendly way
// we used the trait object Box<dyn Error>, Just know that Box<dyn Error> means the function will return a type that implements the Error trait, but we don’t have to specify what particular type the return value will be. This gives us flexibility to return error values that may be of different types in different error cases. 
// The dyn keyword is short for “dynamic."

#[allow(dead_code, unused_variables)]
fn run(config: ConfigX) -> Result<(), Box<dyn Error>> {

    // Rather than panic! on an error, ? will return the error value from the current function for the caller to handle.
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    // The run function now returns an Ok value in the success case. We’ve declared the run function’s success type as () in the signature, 
    // which means we need to wrap the unit type value in the Ok value. 
    Ok(())
}

#[allow(dead_code, unused_variables)]
struct ConfigX {
    query: String,
    file_path: String,
}

impl ConfigX {
    fn build(args: &[String]) -> Result<ConfigX, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(ConfigX { query, file_path })
    }
}

// Done, now let us see the actual implementation:


// Splitting Code into a Library Crate

// Let’s move all the code that isn’t the main function from src/main.rs to src/lib.rs:
    // The run function definition
    // The relevant use statements
    // The definition of Config
    // The Config::new function definition

// Use of the pub keyword: on Config, on its fields and its new method, and on the run function.


// We add a use minigrep::Config line to bring the Config type from the library crate into the binary crate’s scope
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
