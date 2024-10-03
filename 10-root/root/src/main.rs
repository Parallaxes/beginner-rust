/*
 * Program Name : root
 * Description  : This program finds the fifth root of the sum of the squares of the first 100 ODD numbers.
 * Author       : Parallaxis (Parallaxes)
 * Date         : 10-02-2024
 * Notes        : Not even completely sure that it's correct, but it compiles and runs :)
*/

fn main() {
    let mut sum = 0;

    // Weird loop?
    for i in 0..200 {
        if i & 1 == 1 {
            sum += i * i;
        }
    }

    // Yeah, had to google this one.
    let root = (sum as f64).sqrt().powf(1.0 / 5.0);

    println!("{}", root);
}

// :3