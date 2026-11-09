use std::io::{self, BufRead, Write};

// Note: Ensure your local ollama instance is running (`ollama serve`)
// and you have pulled the model (`ollama pull gemma3:4b`).

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- CLI Chatbot with Ollama (gemma3:4b) ---");
    println!("Hello Uday Shiwakoti (uday@uday.com.np)! Type your message to chat.");
    println!("Type 'exit' to quit.\n >");

    let client = reqwest::blocking::Client::new();
    let url = "http://localhost:11434/api/generate";

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line_result in reader.lines() {
        let line = line_result?;
        let prompt = line.trim();

        if prompt.eq_ignore_ascii_case("exit") {
            println!("Closing chatbot session. Goodbye!");
            break;
        }

        if prompt.is_empty() {
            continue;
        }

        print!("Gemma3:4b is thinking...");
        io::stdout().flush()?;

        // Construct the JSON payload for Ollama
        let body = serde_json::json!({
            "model": "gemma3:4b",
            "prompt": prompt,
            "stream": false
        });

        match client.post(url).json(&body).send() {
            Ok(response) => {
                if response.status().is_success() {
                    let json_resp: serde_json::Value = response.json()?;
                    if let Some(reply) = json_resp["response"].as_str() {
                        print!("\r\x1b[K"); // Clear the "thinking" line
                        println!("AI: {}", reply.trim());
                    } else {
                        println!("\nAI: [Received empty response]");
                    }
                } else {
                    println!("\nError: Ollama returned status {}", response.status());
                }
            }
            Err(e) => {
                println!("\nFailed to connect to Ollama server: {}", e);
                println!("Make sure Ollama is running locally via `ollama serve`.");
            }
        }
        println!();
    }

    Ok(())
}

//cargo add serde_json reqwest --features reqwest/blocking,reqwest/json
