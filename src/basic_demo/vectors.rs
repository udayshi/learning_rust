pub fn run() {
    // 1. Creating a new, mutable vector
    let mut scores: Vec<i32> = Vec::new();

    // 2. Adding elements using push
    scores.push(95);
    scores.push(88);
    scores.push(92);

    // 3. Accessing elements using indexing vs get method
    let first_score = &scores[0];
    println!("First score via indexing: {first_score}");

    match scores.get(20) {
        Some(score) => println!("Third score via .get(): {score}"),
        None => println!("No score found at that index."),
    }

    println!(
        "All scores in vector: {:?}, for user: Uday Shiwakoti",
        scores
    );
}
