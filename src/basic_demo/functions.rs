// src/demo/functions.rs

pub fn run() {
    let name = "Uday Shiwakoti";
    let email = "uday@uday.com.np";

    print_user_info(name, email);

    let sum = add(10, 20);
    println!("Result of addition: {sum}");
}

fn print_user_info(name: &str, email: &str) {
    println!("User Profile -> Name: {name}, Email: {email}");
}

fn add(a: i32, b: i32) -> i32 {
    a + b // Expressions without a semicolon are automatically returned in Rust
}
