mod chatbot_ollama;
mod cli_secure_store;
mod concurrent_fetch;
mod file_read_write;
mod http_server;
mod interactive_writer;
mod read_yaml;
mod sqlite_crud;
mod system_command;

/// Runs the currently selected utility example.
// pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
//     // let _ = sqlite_crud::run();
//     // let _ = system_command::run();
//     // let _ = interactive_writer::run();
//     //let _ = file_read_write::run();
//     //let _ = chatbot_ollama::run();

//     //let _ = cli_secure_store::run();
//     //let _ = read_yaml::run();
//     concurrent_fetch::run().await
// }
pub fn run() {
    let _ = http_server::run();
}
