mod basic_demo;
mod basic_utils;

fn main() {
    println!("--- Running Demo ---");

    // 2. Call through the module path into the submodule function
    //demo::basic_dt::run();

    //calling module directly with private method
    // basic_demo::run();
    basic_utils::run();
}
