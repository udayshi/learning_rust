use std::fs;

pub fn run(file_path: &str) {
    println!(
        "Reading file for Uday Shiwakoti (uday@uday.com.np): {}",
        file_path
    );

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("--- File Contents ---\n{}", contents);
}
