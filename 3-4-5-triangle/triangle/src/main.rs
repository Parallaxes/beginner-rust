/*
 * Program Name : triangle
 * Description  : This program generated a left oriented triangle with a given height and choice of bottom or top.
 * Author       : Parallaxis (Parallaxes)
 * Date         : 10-01-2024
 * Notes        : None
*/

use std::io;

fn main() {
    // Size of the base on bottom or top
    println!("Enter bottom [1] or top [2]: ");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    // Height of the triangle (or size of the base, if you will)
    println!("Enter a number: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Data sanitization!
    let height: usize = input.trim().parse().expect("Please type a number!");

    // If bottom is chosen
    if choice.trim() == "1" {
        for i in 1..=height {
            for _ in 0..i {
                print!("*");
            }
    
            println!();
        }
    }
    // If top is chosen
    else if choice.trim() == "2" {
        for i in (1..=height).rev() {
            for _ in 0..i {
                print!("*");
            }
    
            println!();
        }
    }
    // If invalid choice
    else {
        println!("Invalid choice!");
    }
    
}

// :3