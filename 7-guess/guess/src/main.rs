/*
 * Program Name : random
 * Description  : This program generates a random number and has the player guess it. Tells player high or low.
 * Author       : Parallaxis (Parallaxes)
 * Date         : 10-02-2024
 * Notes        : None
*/

use std::io;
use rand::Rng;

fn main() {
    // Generate a random number
    let num = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");

    // Infinite loop until user guesses correct number
    loop {
        let mut guess = String::new();

        // Read user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert user input to u32 + data sanitization!!
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        // Compare user input to random number
        if guess > num {
            println!("Too high!");
        } else if guess < num {
            println!("Too low!");
        } else {
            println!("You win!");
            break;
        }
    }
}

// :3