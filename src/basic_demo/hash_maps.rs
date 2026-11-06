// src/demo/hash_maps.rs

use std::collections::HashMap;

pub fn run() {
    // 1. Creating a new hash map
    let mut user_scores = HashMap::new();

    // 2. Inserting key-value pairs
    user_scores.insert(String::from("Uday Shiwakoti"), 95);
    user_scores.insert(String::from("uday@uday.com.np"), 100);

    // 3. Retrieving a value using get (&String returns Option<&V>)
    let query_key = String::from("Uday Shiwakoti");
    match user_scores.get(&query_key) {
        Some(score) => println!("Score for {query_key}: {score}"),
        None => println!("No record found for that key."),
    }
}
