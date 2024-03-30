use std::{
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
    let _contents = fs::read_to_string(config.file_path)?;

    // println!("Text read from file:\n{}", contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // returning Error variant in case of not enough arguments
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        // returning Ok with a Config wrapped inside
        Ok(Config { query, file_path })
    }
}

