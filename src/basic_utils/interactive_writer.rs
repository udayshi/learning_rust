use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};

pub fn run() -> io::Result<()> {
    let filename = "uday_interactive_log.txt";

    // Open the file in append mode (creates it if it doesn't exist)
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;

    println!("--- Interactive File Writer ---");
    println!(
        "Hello Uday Shiwakoti (uday@uday.com.np)! Type text to append to '{}'.",
        filename
    );
    println!("Type 'exit' on a new line to finish and save.");

    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        print!("> ");
        io::stdout().flush()?; // Ensure the prompt prints immediately

        let mut input_line = String::new();
        reader.read_line(&mut input_line)?;

        // Trim newline and carriage return characters
        let trimmed = input_line.trim_end();

        if trimmed.eq_ignore_ascii_case("exit") {
            println!(
                "Exiting interactive writer. Your data has been saved to {}",
                filename
            );
            break;
        }

        // Write the user's input followed by a newline into the file
        writeln!(file, "{}", trimmed)?;
    }

    Ok(())
}
