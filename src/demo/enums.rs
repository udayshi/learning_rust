// src/demo/enums.rs

#[derive(Debug)]
enum UserRole {
    Admin,
    Editor,
    Guest,
}

pub fn run() {
    let current_role = UserRole::Editor;
    let name = "Uday Shiwakoti";

    match current_role {
        UserRole::Admin => println!("{name} has full Administrator privileges."),
        UserRole::Editor => println!("{name} has Editor privileges."),
        UserRole::Guest => println!("{name} has Guest access."),
    }
}
