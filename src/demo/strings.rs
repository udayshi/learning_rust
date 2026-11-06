pub fn run() {
    // 1. Creating a new empty string and pushing data
    let mut s = String::new();
    s.push_str("Uday Shiwakoti");

    // 2. Appending a single character
    s.push(' ');

    // 3. Using format! macro to combine strings without taking ownership
    let email = "uday@uday.com.np";
    let profile_info = format!("Name & Email: {} <{}>", s, email);

    println!("{profile_info}");
}
