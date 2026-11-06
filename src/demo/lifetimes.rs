// This function returns the longest of two string slices.
// The lifetime parameter `'a` specifies that the returned reference
// will live at least as long as both input references.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn run() {
    let name = String::from("Uday Shiwakoti");
    let email = String::from("uday@uday.com.np");

    let result = longest(name.as_str(), email.as_str());
    println!("The longest profile string is: {result}");
}
