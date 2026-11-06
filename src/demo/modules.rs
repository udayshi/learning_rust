mod profile {
    pub struct User {
        pub name: String,
        pub email: String,
    }

    impl User {
        pub fn new(name: &str, email: &str) -> Self {
            Self {
                name: String::from(name),
                email: String::from(email),
            }
        }
    }
}

pub fn run() {
    let user = profile::User::new("Uday Shiwakoti", "uday@uday.com.np");
    println!(
        "Nested Module User -> Name: {}, Email: {}",
        user.name, user.email
    );
}
