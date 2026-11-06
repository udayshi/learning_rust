// src/demo/structs.rs

struct UserProfile {
    username: String,
    email: String,
    active: bool,
}

pub fn run() {
    let user = UserProfile {
        username: String::from("Uday Shiwakoti"),
        email: String::from("uday@uday.com.np"),
        active: true,
    };

    println!("User Name: {}", user.username);
    println!("User Email: {}", user.email);
    println!("Is Active: {}", user.active);
}
