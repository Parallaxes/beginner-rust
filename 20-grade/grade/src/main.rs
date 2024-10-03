/*
 * Program Name : grade
 * Description  : This program takes a weight and grades from the user and calculates the weighted average.
 * Author       : Parallaxis (Parallaxes)
 * Date         : 10-02-2024
 * Notes        : Decided to just take in a singular total grade and weight and calculate the average. WHAT IS THIS ABOMINATION??
 */

use std::io;

fn main() {
    // Welcome to input and data processing hell. Enjoy your stay would you?
    println!("Enter weight for summative:");
    let mut weight1 = String::new();
    io::stdin().read_line(&mut weight1).expect("Failed to read line");

    println!("Enter weight for formative:");
    let mut weight2 = String::new();
    io::stdin().read_line(&mut weight2).expect("Failed to read line");

    let weight1: f32 = weight1.trim().parse().expect("Please type a number!");
    let weight2: f32 = weight2.trim().parse().expect("Please type a number!");

    println!("Enter grade for summative:");
    let mut grade1 = String::new();
    io::stdin().read_line(&mut grade1).expect("Failed to read line");

    println!("Enter grade for formative:");
    let mut grade2 = String::new();
    io::stdin().read_line(&mut grade2).expect("Failed to read line");

    let grade1: f32 = grade1.trim().parse().expect("Please type a number!");
    let grade2: f32 = grade2.trim().parse().expect("Please type a number!");

    let total_weight = weight1 + weight2;

    let weighted_average = (weight1 * grade1 + weight2 * grade2) / total_weight;

    println!("The weighted average is: {}", weighted_average);

    // Crying
}

// :3