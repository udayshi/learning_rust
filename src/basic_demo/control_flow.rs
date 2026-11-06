pub fn run() {
    let name = "Uday Shiwakoti";
    let score = 85;

    // 1. If/Else conditional expression
    if score >= 90 {
        println!("{name}, you earned an A!");
    } else if score >= 75 {
        println!("{name}, you passed with a good score: {score}!");
    } else {
        println!("{name}, keep trying!");
    }

    // 2. Using if in a let statement
    let status = if score >= 75 { "Passed" } else { "Retry" };
    println!("Status: {status}");
}
