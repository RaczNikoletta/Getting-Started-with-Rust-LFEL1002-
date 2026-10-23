use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;

// Begin with fn main() and an opening curly brace
fn main() {
// Print a message to the standard out enclosed with parentheses and ended with a semicolon
    println!("Hello, open source world!");
// Close the main function
}

// Annotated Basic Rust Example: Factorial Calculation 

// Define a function to calculate the factorial of a number.
fn factorial(n: u64) -> u64 {
    // Base case: If n is 0 or 1, the factorial is 1.
    if n == 0 || n == 1 {
        1
    } else {
        // Recursive case: Calculate factorial by calling the function recursively.
        n * factorial(n - 1)
    }
} 

fn main_2() {
    // Define the number for which we want to calculate the factorial.
    let num: u64 = 5; 

    // Call the factorial function and store the result.
    let result = factorial(num); 

    // Print the result to the console.
    println!("Factorial of {} is: {}", num, result);
}
 

fn main_sec() {
    let file_path = "input.txt"; 

    // Open the input file and print to standard error
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening the file: {}", error);
            return;
        }
    };

    // Create a BufReader to efficiently read the file line by line
    let reader = BufReader::new(file); 

    // Define a regular expression to match potential passwords or API keys
    let password_regex = Regex::new(r"(?i)(password|api[_\s]?key)[:=]\s*(\w+)").unwrap();    

    // Perform the security check by scanning each line of the file
    for (line_number, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                eprintln!("Error reading line {}: {}", line_number + 1, error);
                continue;
            }
        }; 

        // Search for matches in the current line using the password_regex
        if password_regex.is_match(&line) {
            println!("Potential security issue found in line {}: {}", line_number + 1, line);
        }
    }
}
