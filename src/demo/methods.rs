// src/demo/methods.rs

#[derive(Debug)]
struct Account {
    username: String,
    email: String,
}

impl Account {
    // Associated function (constructor)
    fn new(username: &str, email: &str) -> Self {
        Self {
            username: String::from(username),
            email: String::from(email),
        }
    }

    // Method taking an immutable reference to self
    fn display(&self) {
        println!("Account -> User: {}, Email: {}", self.username, self.email);
    }
}

pub fn run() {
    let my_account = Account::new("Uday Shiwakoti", "uday@uday.com.np");
    my_account.display();
}
