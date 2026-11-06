// 1. Defining a trait
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct User {
    pub username: String,
    pub email: String,
}

// 2. Implementing the trait for our User struct
impl Summary for User {
    fn summarize(&self) -> String {
        format!("User {} ({})", self.username, self.email)
    }
}

pub fn run() {
    let user = User {
        username: String::from("Uday Shiwakoti"),
        email: String::from("uday@uday.com.np"),
    };

    println!("Summary trait output: {}", user.summarize());
}
