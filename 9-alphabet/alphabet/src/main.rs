/*
 * Program Name : alphabet
 * Description  : This program counts the number of vowels and consonants in a string; terminates when there is a non-alphabetic character.
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

    let input = input.trim().to_lowercase();

    let mut vowels = 0;
    let mut consonants = 0;

    for c in input.chars() {
        if !c.is_alphabetic() {
            break;
        }

        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
            _ => consonants += 1,
        }
    }

    println!("Vowels: {}\nConsonants: {}", vowels, consonants);
}

// :3