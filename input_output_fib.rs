// Annotated Basic Rust Example: File I/O and Stdout 

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write}; 

fn main() -> io::Result<()> {
    // Open the input file for reading using BufReader
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Read lines from the input file and process them
    for line in reader.lines() {
        let input_line = line?;
        let number: i32 = input_line.trim().parse().expect("Invalid number in file"); 

        // Perform a simple operation (multiply by 2 in this case)
        let result = number * 2; 

        // Print the result to stdout
        println!("Input: {}, Output: {}", number, result); 

        // Open the output file for writing
        let mut output_file = File::create("output.txt")?; 

        // Write the result to the output file
        writeln!(output_file, "{}", result)?;
    } 

    Ok(())
}

// Annotated Advanced Rust Example: Fibonacci Sequence 

// Declare a function to calculate the nth Fibonacci number using dynamic programming.
fn fibonacci_dynamic(n: u64) -> u64 {
    // Create a vector to store the Fibonacci numbers.
    let mut fib_nums: Vec<u64> = Vec::with_capacity(n as usize + 1); 

    // Initialize the first two Fibonacci numbers.
    fib_nums.push(0);
    fib_nums.push(1); 

    // Calculate and store Fibonacci numbers from the 3rd to the nth.
    for i in 2..=n {
        let next_fib = fib_nums[i as usize - 1] + fib_nums[i as usize - 2];
        fib_nums.push(next_fib);
    } 

    // Return the nth Fibonacci number.
    fib_nums[n as usize]
} 

fn main() {
    // Define the value of n for which we want to calculate the Fibonacci number.
    let n: u64 = 10; 

    // Call the fibonacci_dynamic function and store the result.
    let result = fibonacci_dynamic(n); 

    // Print the result.
    println!("The {}th Fibonacci number is: {}", n, result);
}
