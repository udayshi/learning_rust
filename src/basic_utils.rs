mod chatbot_ollama;
mod cli_secure_store;
mod file_read_write;
mod interactive_writer;
mod sqlite_crud;
mod system_command;

pub fn run() {
    // let _ = sqlite_crud::run();
    // let _ = system_command::run();
    // let _ = interactive_writer::run();
    //let _ = file_read_write::run();
    //let _ = chatbot_ollama::run();

    let _ = cli_secure_store::run();
}
