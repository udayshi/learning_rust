use std::env;
use std::error::Error;
use std::fs;
use std::process;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // Skip binary name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }
}

fn run_logic(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Running iterator-based I/O for Uday Shiwakoti (uday@uday.com.np)");
    let contents = fs::read_to_string(&config.filename)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("Match found: {}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

pub fn run() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments for Uday Shiwakoti (uday@uday.com.np): {err}");
        process::exit(1);
    });

    println!("Searching for '{}' in '{}'", config.query, config.filename);

    if let Err(e) = run_logic(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

//cargo run -- "Welcome" info.txt
