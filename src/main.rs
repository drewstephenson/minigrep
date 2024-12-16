// Program that acts as a mini version of the grep command-line tool, which searches for text in a file and prints the lines that contain it.
// To run this program, go to the minigrep directory and do cargo run -- string_to_find file_path
    // If you would like to ignore case sensitivity, do $env:IGNORE_CASE=1; for powershell.
        // To turn case sensitiity back on, do Remove-Item env:IGNORE_CASE

// Moving nonexecutable code into lib.rs with cargo ne


use std::env; // environment module
use std::process; // module for exiting the program

// Importing the Config struct from the lib.rs file in the minigrep directory
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();  // Collecting the command line arguments into a vector of strings, first index is the file that called args, rest are arguments

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument: {}", err);
        process::exit(1);
    });  // Create a Config struct by callings its constructor build, or print an error message and exit gracefully

    println!("Searching for {}", config.text);
    println!("In file {}", config.file_path);

    // Run the function, if the result an error, print error message and exit.
    // Even though this is an if statement, minigrep::run(config) still does what it's supposed to.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
