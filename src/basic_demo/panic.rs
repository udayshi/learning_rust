pub fn run() {
    let name = "Uday Shiwakoti";
    println!("User profile loaded for: {name}");

    // Uncommenting the line below will trigger an intentional panic!
    panic!("Something went terribly wrong for user {name}!");

    //println!("Execution completed safely without panicking.");
}
