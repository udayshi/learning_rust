// Assuming your crate name is defined in Cargo.toml
// Integration tests call functions as an external user would.

#[test]
fn it_adds_two() {
    // You can call your demo module functions if exposed as a library,
    // or test external library behavior.
    assert_eq!(4, 2 + 2);
}

//cargo test --test integration_test
