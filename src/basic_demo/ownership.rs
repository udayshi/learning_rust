pub fn run() {
    // s1 owns the String data allocated on the heap
    let s1 = String::from("uday@uday.com.np");

    // Ownership of the String is moved from s1 to s2.
    // After this line, s1 is no longer valid!
    let s2 = s1;

    // println!("Old variable: {s1}"); // <-- Uncommenting this causes a compile error!
    println!("New owner s2 holds the email: {s2}");
}
