use std::error::Error;
use std::fs;

use std::env;
use std::process;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
fn run_refactor(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Running refactored I/O for Uday Shiwakoti (uday@uday.com.np)");
    let contents = fs::read_to_string(&config.filename)?;

    println!(
        "Searching for query: '{}' in file: '{}'",
        config.query, config.filename
    );
    for line in contents.lines() {
        if line.contains(&config.query) {
            println!("Found match: {}", line);
        }
    }

    Ok(())
}
pub fn run() {
    let args: Vec<String> = env::args().collect();

    // Use Config::build to parse our arguments: cargo run <query> <filename>
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments for Uday Shiwakoti (uday@uday.com.np): {err}");
        process::exit(1);
    });

    println!("Searching for '{}' in '{}'", config.query, config.filename);

    if let Err(e) = run_refactor(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

//cargo run -- "Welcome" info.txt
