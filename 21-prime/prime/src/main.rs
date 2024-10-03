/*
 * Program Name : prime
 * Description  : This program takes number and calculates the pairs of prime numbers that compose the inputted number.
 * Author       : Parallaxis (Parallaxes)
 * Date         : 10-02-2024
 * Notes        : None
 */

use std::io;

// Check if a number is prime
// Frankly, I'm not exactly sure how this works. Copilot autocompleted this for me.
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    println!("Enter a number: ");

    // Wow, you finally get one line inputs!
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input: u32 = input.trim().parse().expect("Please type a number!");

    // Best way to do this??
    for i in 0..input {
        if is_prime(i) && is_prime(input - i) {
            println!("{} + {} = {}", i, input - i, input);
        }
    }
}

// :3