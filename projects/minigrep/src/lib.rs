// Developing the library's functionality with TDD

// Steps to perform to do Test Driven Development
    // 1. Write a test that fails and make sure it fails for the reason you expect.
    // 2. Write code such that the test passes, and refactor the code and make sure the test continues to pass
    // 3. Repeat.

use std::{env, fs, error::Error};


/*
pub struct Config {
    pub query: String,
    pub file_path: String,
}
*/

/*
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
*/


/*
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}
*/

// TDD Iteration 1: Write a failing test

/*

IMPORTANT:

We tell Rust that the data returned by the search function will live as long as the data passed into the search function in the contents argument.

Because contents is the argument that contains all of our text and we want to return the parts of that text that match, we know contents is the argument 
that should be connected to the return value using the lifetime syntax.

This is important! The data referenced by a slice needs to be valid for the reference to be valid; if the compiler assumes we’re making string slices of 
query rather than contents, it will do its safety checking incorrectly.

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

*/

// TDD Iteration 2: Write code to pass the test

// Rust has a helpful method to handle line-by-line iteration of strings, conveniently named string.lines() which returns an Iterator
// Add a call to the string.contains() method in the search function.
// Store the results in a mut vector and return them.

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut res: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

}

// Using the search function inside the run function

// The search function is working and tested, we need to call search from our run function

/*
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}
*/

// Tests:
// Search for a word that returns one line
// $ cargo run -- frog sample.txt

// Search for a word that returns multiple lines
// $ cargo run -- body sample.txt

// Search for a word that doesn't return anything
// $ cargo run -- cc sample.txt

// Working with env variables

// Feature: An option for case-insensitive searching that the user can turn on via an environment variable

// TDD Iteration 1: Write a failing test for new function

// A new search_case_insensitive() function that will be called when the environment variable has a value
// Add a new failing test called case_insensitive() which searches for 'rUsT' in the contents.

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }


}

// TDD Iteration 2: Write code to pass the test

// The only difference is that we’ll lowercase the query and each line so whatever the case of the input arguments, 
// they’ll be the same case when we check whether the line contains the query


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// This passed all the tests, now lets integrate this into the exisiting run function

// Before that, we need to add a variable to Config in order to get the state of the environment variable
// Lets add an ignore_case boolean to the Config struct,

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        // Read this value from the env variable
        /*
        The env::var function returns a Result that will be the successful Ok variant that contains the value of the environment variable if 
        the environment variable is set to any value. It will return the Err variant if the environment variable is not set.

        We’re using the is_ok method on the Result to check whether the environment variable is set, which means the program should do a case-insensitive search. 
        If the IGNORE_CASE environment variable isn’t set to anything, is_ok will return false and the program will perform a case-sensitive search.
        */
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

// We added the ignore_case field that holds a Boolean. Next, we need the run function to check the ignore_case field’s 
// value and use that to decide whether to call the search function or the search_case_insensitive function.


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// Tests

// Search for the word 'to' without ignore case:
// $ cargo run -- to sample.txt

// Search for the word 'to' in a case insensitive manner:
// $ IGNORE_CASE=1 cargo run -- to sample.txt

// Some programs allow arguments and environment variables for the same configuration.
// In those cases, the programs decide that one or the other takes precedence.

// Writing Error Messages to stderr instead of stdout

// The println!() macro is only capable of printing to stdout, eprintln!() prints to stderr
// Command line programs are expected to send error messages to the standard error stream so we can 
// still see error messages on the screen even if we redirect the standard output stream to a file.

// Printing errors to stderr

// Thanks to the refactoring we did earlier in this chapter, all the code that prints error messages is in one function, main. 
// The standard library provides the eprintln! macro that prints to the standard error stream, so let’s change the two places we were calling println! to print errors to use eprintln! instead.
// ==> Check main.rs for the modifications!


