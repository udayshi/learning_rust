// src/demo/variables.rs

pub fn run() {
    // Immutable by default
    let name = "Uday Shiwakoti";
    let email = "uday@uday.com.np";

    // Mutable variable using `mut`
    let mut score = 90;
    println!("User: {name} ({email})");
    println!("Initial Score: {score}");

    score = 95; // Allowed because score is mutable
    println!("Updated Score: {score}");
}
