/*
 * Program Name : circle
 * Description  : This program calculates the radius, diameter, and area of a circle given the radius, diameter, or area.
 * Author       : Parallaxis (Parallaxes)
 * Date         : 10-02-2024
 * Notes        : Lots of warnings... Warnings I'm not going to fix.
*/

use std::io;

// Calculate the radius, diameter, and area

fn circle_from_radius(radius: f64) -> (f64, f64, f64) {
    let diameter = 2.0 * radius;
    let area = std::f64::consts::PI * radius * radius;

    (radius, diameter, area)
}

fn circle_from_diameter(diameter: f64) -> (f64, f64, f64) {
    let radius = diameter / 2.0;
    let area = std::f64::consts::PI * radius * radius;

    (radius, diameter, area)
}

fn circle_from_area(area: f64) -> (f64, f64, f64) {
    let radius = (area / std::f64::consts::PI).sqrt();
    let diameter = 2.0 * radius;

    (radius, diameter, area)
}

fn main() {
    println!("Enter the radius, diameter, or area of the circle: ");

    println!("1. Radius");
    println!("2. Diameter");
    println!("3. Area");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let option: f64 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input.");
            return;
        }
    };

    match option {
        1f64 => println!("Enter the radius: "),
        2f64 => println!("Enter the diameter: "),
        3f64 => println!("Enter the area: "),
        _ => {
            println!("Invalid input.");
            return;
        }
    }; // This annoying semicolon gave me a it of trouble...

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    // Oh god why does it look like this
    match num.trim().parse() {
        Ok(num) => {
            let circle = match option {
                1f64 => circle_from_radius(num),
                2f64 => circle_from_diameter(num),
                3f64 => circle_from_area(num),
                _ => panic!("Invalid input."),
            }; // And this one

            println!("{:?}", circle);
        }
        
        // Why not panic? I don't know. Maybe it was me panicing all along.
        Err(_) => println!("Invalid input."),
    }; // And this one
}

// :3