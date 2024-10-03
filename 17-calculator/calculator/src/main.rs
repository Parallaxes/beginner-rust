/*
 * Program Name : calculator
 * Description  : This program performs simple mathematical calculations.
 * Author       : Parallaxis (Parallaxes)
 * Date         : 10-02-2024
 * Notes        : Only accepts two fields for calculation.
*/

use std::io;

fn main() {
    println!("Calculator!");

    // Loop until the user exits
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Exit if the user types "exit"
        if input.trim() == "exit" {
            break;
        }

        // Check for the operations
        let operations = vec!["+", "-", "*", "/"];

        // Loop to find the operation, match to case, and perform the calculation
        for op in operations {

            // What the hell is this?
            match input.find(op) {
                Some(_) => {
                    let parts = input.split(op).collect::<Vec<&str>>();

                    if parts.len() < 2 {
                        println!("Invalid input");
                        continue;
                    }

                    // There's got to be a better way to do this
                    let a: f64 = match parts[0].trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Invalid input");
                            continue;
                        }
                    };

                    // And this. I don't like how this looks.
                    let b: f64 = match parts[1].trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Invalid input");
                            continue;
                        }
                    };

                    // Match the operation and perform the calculation
                    // Maybe there's a better, more modular way to do this (with structs?)
                    match op {
                        "+" => println!("{}", a + b),
                        "-" => println!("{}", a - b),
                        "*" => println!("{}", a * b),
                        "/" => println!("{}", a / b),
                        _ => println!("Invalid input"),
                    };
                },

                // If there's no operation, do nothing
                None => {}
            };
        }
    }
}

// :3