/*
 * Program Name : reverse
 * Description  : This program reverses a string using recursion
 * Author       : Parallaxis (Parallaxes)
 * Date         : 10-02-2024
 * Notes        : None
*/

use std::io;

// Reverse, reverse, reverse
fn reverse(input: &str) -> String {
    if input.is_empty() {
        String::new()
    } else {
        let (first, rest) = input.split_at(1);
        format!("{}{}", reverse(rest), first)
    }
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    let reversed = reverse(input);

    println!("{}", reversed);
}

// :3