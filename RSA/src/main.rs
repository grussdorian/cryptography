/*
RSA Algorithm, takes input from CLI, encrypts and decrypts the message using RSA algorithm
.env file contains the values of p and q. Edit it to see effects of different values of p and q
*/

use std::env;
use dotenv::dotenv;
fn mod_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent = exponent >> 1;
        base = (base * base) % modulus;
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Concatenate all arguments into a single string, skipping the first one
    dotenv().ok(); // This will load the .env file at the beginning of your program

    let args: Vec<String> = env::args().collect();
    let message = args[1..].join(" ");
    let program_name = args[0].clone();
    println!("Program name: {}", program_name);
    let p_str = env::var("p").expect("p not found in .env file");
    let q_str = env::var("q").expect("q not found in .env file");

    // Convert the string values to u64
    let p: u64 = p_str.parse().expect("Failed to parse p to u64");
    let q: u64 = q_str.parse().expect("Failed to parse q to u64");

    // let p: u64 = 53;
    // let q: u64 = 59;
    let n = p * q;
    let phi = (p - 1) * (q - 1);
    let e: u64 = 3;
    // Calculate d using Extended Euclidean Algorithm (simplified for e = 3)
    let mut d = 2; // Starting value for demonstration purposes
    while (d * e) % phi != 1 {
        d += 1;
    }
    let mut encrypted_message = String::new();
    let mut decrypted_message = String::new();
    for chr in message.chars() {
        let m = chr as u64;
        let c = mod_pow(m, e, n);
        let dcrp = mod_pow(c, d as u64, n);
        encrypted_message.push(std::char::from_u32(c as u32).unwrap_or_default());
        decrypted_message.push(std::char::from_u32(dcrp as u32).unwrap_or_default());
    }

    println!("{}", message);
    println!("Encrypted message: {}", encrypted_message);
    println!("Decrypted message: {}", decrypted_message);
    println!("Length of message: {}", message.len());
    println!("Length of encrypted message: {}", encrypted_message.len());
}