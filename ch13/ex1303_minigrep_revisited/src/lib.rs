use std::{
    env,
    error::Error, 
    fs, 
};

// extracting logic from main
// return type of the run function is Result<(), Box<dyn Error>>
// unit type () is returned in case of Ok case
// in error case the trait object Box<dyn Error> is returned
// Box<dyn Error> means the function will return a type that implements the Error trait
// allows us to return error values that may be of different type
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // let contents = fs::read_to_string(config.file_path)
    //    .expect("Couldn't read the file from the path you provided.");
    // the ? operator will return the error value from the current function 
    // for the caller to handle
    let contents = fs::read_to_string(config.file_path)?;
    // examples that print a line:
    // cargo run -- frog data/poem.txt
    // cargo run -- body data/poem.txt
    // examples that don't find a line:
    // cargo run -- monomorphization data/poem.txt
    // search with environment variable:
    // IGNORE_CASE=1 cargo run -- to data/poem.txt
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // signature of the Config::build function
    // -> the parameter args has a generic type 
    // with the trait bounds impl Iterator<Item = String> instead of &[String]
    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        // args implements the Iterator trait so we can call the next method on it
        // first value in the return value of env::args is the name of the program
        // which we don't need and therefore skip
        args.next();
        
        // second we call next to get the value we want to put in the query field of Config
        // if Some we use a match to extract the value
        // if None it means not enough arguments were given 
        // -> we return early with an Err value
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        
        // env::var returns a Result that will be the successful Ok variant 
        // if the environment variable is set to any value
        // it will return the Err variant if the environment variable is not set
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        
        // returning Ok with a Config wrapped inside
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // rewriting the code in a more concise way using iterator adaptor methods
    // doing so lets us avoid having a mutable intermediate results vector
    // the purpose of the search function is to return all lines in contents 
    // that contain the query
    // here we use the filter adaptor to keep only the lines 
    // that line.contains(query) returns true for
    // the matching lines are collected into another vector with collect
    contents
        .lines()
        .filter(|line| { line.contains(query) })
        .collect()
}

// case insensitive search function
pub fn search_case_insensitive<'a>(
    query: &str, 
    contents: &'a str
) -> Vec<&'a str> {
    // calling to_lowercase creates new data rather than referencing existing data
    // query is becomes a String rather than a string slice
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| { line.to_lowercase().contains(&query) })
        .collect()

}

/// test-driven development (TDD):
/// (1) write a test that fails and run it to make sure it fails for the reason you expect
/// (2) write or modify just enough code to make the new test pass
/// (3) refactor the code you just added or changed and make sure the tests continue to pass
/// (4) repeat from step 1

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    // test for case insensitive search
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."], 
            search_case_insensitive(query, contents)
        );
    }
}