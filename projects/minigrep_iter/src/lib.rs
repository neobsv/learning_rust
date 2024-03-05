use std::{env, fs, error::Error};

// Making Code Clearer with Iterator Adaptors

// We can write this code in a more concise way using iterator adaptor methods. Doing so also lets us avoid having a mutable intermediate results vector.
// Removing the mutable state might enable a future enhancement to make searching happen in parallel, because we wouldn’t have to manage concurrent access to the results vector.

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    // let mut res: Vec<&str> = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         res.push(line);
    //     }
    // }
    // res

    contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

// Making Code Clearer with Iterator Adaptors

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
    .lines()
    .filter(|line| line.to_lowercase().contains(&query))
    .collect()

}


pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}


// Removing a clone() Using an Iterator

impl Config {

    // Updated the signature of the Config::build function so the parameter args has a generic type with the trait bounds impl Iterator<Item = String>

    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        
        args.next(); // The first arg is the filepath, so we just call next and ignore it.

        // Instead of using clone() to make a copy to allow the Config struct to own the arg values,
        // we can use the iterator provided by env::args() in the main.rs file directly!

        // let query = args[1].clone();
        // let file_path = args[2].clone();

        // Using Iterator Trait Methods Instead of Indexing

        // The next() trait method returns an Option enum, which can be passed to a match block and switched into either returning the arg or retuning an Err.

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };



        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}


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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

}