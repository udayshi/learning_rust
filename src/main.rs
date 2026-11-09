mod basic_demo;
mod basic_utils;

/// Starts the currently selected learning example.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Running Demo ---");

    // 2. Call through the module path into the submodule function
    //demo::basic_dt::run();

    //calling module directly with private method
    // basic_demo::run();
    basic_utils::run().await
}
