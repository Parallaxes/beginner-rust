/*
 * Program Name : factorial
 * Description  : This program calculates the factorial of a number.
 * Author       : Parallaxis (Parallaxes)
 * Date         : 10-01-2024
 * Notes        : None
*/

use std::io;

// Simple recursive function to calculate the factorial of a number.
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    println!("Enter a number to find the factorial of: ");

    let mut input = String::new();

    // Read input, check for empty input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Extra debugging line
    let input = input.trim();
    println!("You entered: {}", input);

    // Who needs error handling anyway!
    // Failsafe against negative numbers and decimal numbers.
    match input.parse::<f64>() {
        Ok(num) if num < 0.0 => {
            println!("Error: Factorial is not defined for negative numbers.");
        }
        Ok(num) if num.fract() != 0.0 => {
            println!("Error: Factorial is not defined for decimal numbers.");
        }
        Ok(num) => {
            let num: u64 = num as u64;
            println!("The factorial of {} is {}", num, factorial(num));
        }
        Err(_) => {
            println!("Error: Invalid input. Please enter a valid number.");
        }
    }
}

// :3