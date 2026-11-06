use std::fs::File;
use std::io::ErrorKind;

pub fn run() {
    let name = "Uday Shiwakoti";
    let greeting_file_result = File::open("hello.txt");

    let _file = match greeting_file_result {
        Ok(file) => {
            println!("{name}, file successfully opened!");
            file
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("{name}, file not found, but we can handle this gracefully.");
                // You could create the file here if desired
                panic!("File not found error handled for user: {name}");
            }
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
