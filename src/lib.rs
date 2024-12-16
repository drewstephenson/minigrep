use std::error::Error; // module for error handling
use std::fs; // file system module
use std::env; // environment module

// Struct that can hold command line arguments
pub struct Config {
    pub text: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // Constructor for Config struct
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments given.");
        }
        let text = args[1].clone();
        let file_path = args[2].clone();
        
        // This is cool, env::var allows you to create a flag, in this case IGNORE_CASE, that can be used to set the ignore_case Config field to true or false.
        // is_ok() checks if the variable is set and returns the boolean value. If it is not set, defaults to false (IGNORE_CASE=0).
        let ignore_case = env::var("IGNORE_CASE").is_ok(); // Check if the IGNORE_CASE environment variable is set

        Ok(Config { text, file_path, ignore_case, })
    }
}

// Function that reads the file and prints its contents
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path)?; // Read the file, turn it to a string. If there is an error, ? will return the error value from the function for handling.

        let search = if config.ignore_case {
            search_case_insensitive(&config.text, &contents)  // If ignore_case is true, call this one
        } else {
            search_case_sensitive(&config.text, &contents)  // If ignore_case is false, call this one
        };

        // perhaps add a if search does not return empty vector here
        for line in search { // Print the lines from whichever search function was called
            println!("{line}");
        }

    Ok(()) // When the function is successful, return an empty Ok value (will move on)
}

// Function that searches for the given text in the file. We define a lifetime because the return value needs to last as long as the file's contents.
pub fn search_case_sensitive<'a>(text: &str, contents: &'a str) -> Vec<&'a str> {
    let mut lines_with_text = Vec::new();

    for line in contents.lines() {
        if line.contains(text) {
            lines_with_text.push(line);  // Add the line to the vector we return if it contains the text
        }
    }
    
    lines_with_text
}

pub fn search_case_insensitive<'a>(text: &str, contents: &'a str) -> Vec<&'a str> {
    let text = text.to_lowercase();  // convert segment to be searched to lowercase (this makes text a String)
    let mut lines_with_text = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&text) { // this uses &text since contains expects a string slice
            lines_with_text.push(line); 
        }
    }

    lines_with_text
}

// The cargo test 🤓
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let text = "duct";

        // This is ugly but its split into lines to simulate a text file with multiple lines. The \ tells Rust to not put a newline character.
        let contents = "\
Rust:
safe, fast, productive.
Pick three. 🤓
I love Duct tape."; // Duct tape should not be included in the return result.

        // Check if the vector of str slices is equal to the result of the case sensitive search function.
        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(text, contents));  
    }

    #[test]
    fn case_insensitive() {
        let text = "rUSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three. 🤓
Trust me. 🤓🤓🤓";

        // Check if the vector of str slices is equal to the result of the case insensitive search function.
        assert_eq!(vec!["Rust:", "Trust me. 🤓🤓🤓"], search_case_insensitive(text, contents));
    }
}


