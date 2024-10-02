/*
 * Program Name : currency
 * Description  : This program converts currency from USD to EUR and vice versa.
 * Author       : Parallaxis (Parallaxes)
 * Date         : 10-01-2024
 * Notes        : None
*/

use std::io;

fn main() {
    println!("Currency Converter");
    println!("1. USD to EUR");
    println!("2. EUR to USD");

    // More failproofing!!
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    // USD to EUR
    if choice.trim() == "1" {
        println!("Enter the amount in USD: ");
        let mut usd = String::new();
        io::stdin()
            .read_line(&mut usd)
            .expect("Failed to read line");

        let usd: f64 = usd.trim().parse().expect("Please type a number!");

        let eur = usd * 0.90;

        // Add padding to the output
        println!("{:.2} USD is {:.2} EUR", usd, eur);
    } 
    // EUR to USD
    else if choice.trim() == "2" {
        println!("Enter the amount in EUR: ");
        let mut eur = String::new();
        io::stdin()
            .read_line(&mut eur)
            .expect("Failed to read line");

        let eur: f64 = eur.trim().parse().expect("Please type a number!");

        let usd = eur * 1.11;
        println!("{:.2} EUR is {:.2} USD", eur, usd);
    }
    // Nonexistent choice
    else {
        println!("Invalid choice");
    }
}

// :3