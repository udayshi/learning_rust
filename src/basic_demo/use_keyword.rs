mod network {
    pub fn connect(email: &str) {
        println!("Connecting securely to network for user: {email}");
    }
}

// Bring the `connect` function into scope using `use`
use network::connect;

pub fn run() {
    let email = "uday@uday.com.np";

    // Now we can call connect directly without writing network::connect
    connect(email);
}
