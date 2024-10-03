/*
 * Program Name : fibo
 * Description  : This program generates the Fibonacci sequence up to a certain number.
 * Author       : Parallaxis (Parallaxes)
 * Date         : 10-02-2024
 * Notes        : Rust is so freakin' fast!
*/

use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Convert input to u64 + data sanitization!! (u32 also works for this use case)
    let input: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number!");
            return;
        }
    };

    // Generate the Fibonacci sequence
    let mut a = 0;
    let mut b = 1;

    while a <= input {
        print!("{} ", a);
        let c = a + b;
        a = b;
        b = c;
    }

    // New line for readability
    println!();
}

// :3