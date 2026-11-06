// A generic struct that can hold any type T for its value
#[derive(Debug)]
struct Profile<T> {
    username: String,
    identifier: T,
}

pub fn run() {
    // Using an integer for the generic identifier type
    let user_id = Profile {
        username: String::from("Uday Shiwakoti"),
        identifier: 42,
    };

    // Using a string slice for the generic identifier type
    let user_email = Profile {
        username: String::from("Uday Shiwakoti"),
        identifier: String::from("uday@uday.com.np"),
    };

    println!("User with integer ID: {:?}", user_id);
    println!("User with email identifier: {:?}", user_email);
}
