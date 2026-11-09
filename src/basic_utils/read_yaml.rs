use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug)]
struct Settings {
    theme: String,
    timeout_seconds: u32,
    features_enabled: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Config {
    name: String,
    email: String,
    settings: Settings,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("--- Reading YAML Configuration for Uday Shiwakoti ---");

    // 1. Open the YAML file
    let mut file = File::open("demo.yaml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // 2. Parse the YAML string into our strongly-typed Config struct
    let config: Config = serde_yaml::from_str(&contents)?;

    // 3. Print the parsed values
    println!("Parsed Name: {}", config.name);
    println!("Parsed Email: {}", config.email);
    println!("Theme: {}", config.settings.theme);
    println!("Timeout: {} seconds", config.settings.timeout_seconds);
    println!("Enabled Features: {:?}", config.settings.features_enabled);

    Ok(())
}

/*
cargo add serde --features derive
cargo add serde_yaml@0.9
*/
