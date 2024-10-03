/*
 * Program Name : swap
 * Description  : This program swaps three variables without using a temporary variable.
 * Author       : Parallaxis (Parallaxes)
 * Date         : 10-02-2024
 * Notes        : Cheese.
*/

fn main() {
    let mut a = 1;
    let mut b = 2;
    let mut c = 3;

    println!("Before: a = {}, b = {}, c = {}", a, b, c);

    // Cheese alert! This is a cheesy way to swap three variables without using a temporary variable.
    // Swap a -> b, b -> c, c -> a
    a = a + b + c;
    b = a - (b + c);
    c = a - (b + c);
    a = a - (b + c);

    println!("After: a = {}, b = {}, c = {}", a, b, c);
}

// :3