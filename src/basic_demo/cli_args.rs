use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    println!("Parsed arguments for Uday Shiwakoti (uday@uday.com.np):");
    for (index, arg) in args.iter().enumerate() {
        println!("Arg {}: {}", index, arg);
    }
}

//cargo run -- arg1 arg2 uday
