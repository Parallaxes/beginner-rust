/*
 * Program Name : palindrome
 * Description  : This program checks if a string is a palindrome.
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

    // Learned this earlier! 
    if input == input.chars().rev().collect::<String>() {
        println!("Palindrome!");
    } else {
        println!("Not a palindrome.");
    }
}

// :3