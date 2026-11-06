mod sqlite_crud;
mod system_command;

pub fn run() {
    let _ = sqlite_crud::run();
    // let _ = system_command::run();
}
