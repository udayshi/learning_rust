use std::process::Command;

pub fn run() {
    let output = Command::new("df")
        .arg("-h")
        .arg("/")
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
