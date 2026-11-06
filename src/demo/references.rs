pub fn run() {
    let email = String::from("uday@uday.com.np");

    // 1. Immutable reference (&) - passes access without transferring ownership
    let len = calculate_length(&email);
    println!("The email '{}' is {} characters long.", email, len);

    // 2. Mutable reference (&mut) - allows modifying borrowed data
    let mut name = String::from("Uday");
    append_lastname(&mut name);
    println!("Updated Full Name: {name}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn append_lastname(s: &mut String) {
    s.push_str(" Shiwakoti");
}
