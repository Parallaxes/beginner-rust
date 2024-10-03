/*
 * Program Name : revnum
 * Description  : This program reverses the numerals of a number.
 * Author       : Parallaxis (Parallaxes)
 * Date         : 10-02-2024
 * Notes        : None
*/

use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    // What is this...?
    if input.chars().all(|c| c.is_digit(10)) {
        let reversed = input.chars().rev().collect::<String>();
        let reversed_number: i64 = reversed.parse().expect("Failed to parse number");
        println!("{}", reversed_number);
    } else {
        println!("Please enter a valid number.");
    }
}

// :3