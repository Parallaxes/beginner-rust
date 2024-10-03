/*
 * Program Name : random
 * Description  : This program generates a random number and matches a case to it.
 * Author       : Parallaxis (Parallaxes)
 * Date         : 10-02-2024
 * Notes        : None
*/

use rand::Rng;

fn main() {
    // Generate a random number between 1 and 3 (inclusive)
    let num = rand::thread_rng().gen_range(1..=3);

    // Match the number to a case
    match num {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Something else"),
        _ => unreachable!(),
    }
}

// :3