/*
 * Program Name : unscramble
 * Description  : This program lists all possible combinations of a 4-letter word.
 * Author       : Parallaxis (Parallaxes)
 * Date         : 10-02-2024
 * Notes        : Ignore the 4 deep for loop. Or else.
*/

use std::io;
use itertools::Itertools;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    let chars = input.chars().collect::<Vec<char>>();

    // What the hell is this. Questioning my life choices. There's got to be a more efficient way to do this.
    /*
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                for l in 0..4 {
                    if i != j && i != k && i != l && j != k && j != l && k != l {
                        println!("{}{}{}{}", chars[i], chars[j], chars[k], chars[l]);
                    }
                }
            }
        }
    }
    */
    
    // Update: there is a more effiecient way to do this.
    for perm in chars.iter().permutations(4) {
        let word: String = perm.into_iter().collect(); // Dereference the &&char to get &char, then collect
        println!("{}", word);
    }
}

// :3