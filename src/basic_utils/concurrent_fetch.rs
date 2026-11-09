use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Post {
    id: u32,
    title: String,
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let author = "Uday Shiwakoti";
    println!(
        "--- Concurrent API Fetcher for {} (uday@uday.com.np) ---",
        author
    );
    println!("Fetching posts 1 through 10 concurrently...");

    let client = Client::new();
    let mut tasks = vec![];

    // Spawn an asynchronous task for each ID from 1 to 5
    for id in 1..=5 {
        let client_clone = client.clone();
        let task = tokio::spawn(async move {
            let url = format!("https://jsonplaceholder.typicode.com/posts/{}", id);
            let res = client_clone.get(&url).send().await?.json::<Post>().await?;
            Ok::<Post, reqwest::Error>(res)
        });
        tasks.push(task);
    }

    // Wait for all concurrent tasks to complete
    let mut results = vec![];
    for task in tasks {
        let post = task.await??;
        results.push(post);
    }

    results.sort_by_key(|post| post.id);

    println!("\n--- All 10 requests completed successfully! ---");
    for post in results {
        println!("{}", format_post(&post));
    }

    Ok(())
}

/// Formats one fetched post for terminal output.
fn format_post(post: &Post) -> String {
    format!("- Post ID {}: {}", post.id, post.title)
}

#[cfg(test)]
mod tests {
    use super::{Post, format_post};

    #[test]
    fn formats_post_id_and_title() {
        let post = Post {
            id: 7,
            title: String::from("A clear title"),
        };

        assert_eq!(format_post(&post), "- Post ID 7: A clear title");
    }

    #[test]
    fn preserves_an_empty_title_in_output() {
        let post = Post {
            id: 10,
            title: String::new(),
        };

        assert_eq!(format_post(&post), "- Post ID 10: ");
    }
}

/*
cargo add reqwest --features json
cargo add tokio --features full
cargo add serde --features derive
cargo add serde_json
*/
