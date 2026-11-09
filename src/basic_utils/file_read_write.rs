use std::fs::{self, OpenOptions};
use std::io::{self, Write};

pub fn run() -> io::Result<()> {
    let filename = "uday_audit_log.txt";
    let new_log_entry = "Log entry for Uday Shiwakoti (uday@uday.com.np): Action performed.\n";

    // Use OpenOptions to open the file for appending, creating it if it doesn't exist
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;

    // Write the new content to the end of the file
    file.write_all(new_log_entry.as_bytes())?;
    println!("Successfully appended data to {}", filename);

    // Read and print the current contents of the file to verify
    let contents = fs::read_to_string(filename)?;
    println!("--- Current File Contents ---");
    println!("{}", contents);

    // Optional: Clean up the log file afterwards
    //  fs::remove_file(filename)?;

    Ok(())
}
