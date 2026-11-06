# Learn Rust

A small hands-on Rust project for building a solid foundation in the language. Rust is receiving a lot of
attention in the AI era, but understanding its core concepts comes first. The examples are intentionally small so
each idea can be read, run, and changed without getting lost in application complexity.

The examples in this repository cover:

### [Basic data types](src/demo/basic_dt.rs)

Learn about integer and floating point scalars, strings, booleans, and characters. The example gives each value an
appropriate Rust type and prints a small user profile. It is a starting point for seeing how static typing works.

### [Variables and mutability](src/demo/variables.rs)

See how Rust bindings are immutable by default and how `mut` allows a value to change. The example updates a score
after it is created and prints both values. This shows when mutability must be declared explicitly.

### [Functions](src/demo/functions.rs)

Practice passing parameters to functions and returning a value from a helper function. One helper prints user
information, while another adds two integers. The addition function shows Rust's expression based return style.

### [Control flow](src/demo/control_flow.rs)

Explore conditional expressions with `if`, `else if`, and `else`. The example assigns a grade message based on a
score and then uses another conditional expression to set a status. It shows that an `if` expression can produce a
value.

### [Ownership](src/demo/ownership.rs)

Understand heap allocation and move semantics with a `String` value. The example moves an email from one variable
to another, leaving the original variable unavailable. This is the central rule that prevents invalid memory access.

### [Borrowing and references](src/demo/references.rs)

Learn safe immutable and mutable access without transferring ownership. An immutable reference is used to calculate
an email length, and a mutable reference adds a last name. Both examples keep the original owner responsible for the
data.

### [Structs](src/demo/structs.rs)

Use a custom `UserProfile` type to group a username, email address, and active status. The example creates a struct
value and reads each of its fields. Structs make related data easier to model and pass around together.

### [Methods](src/demo/methods.rs)

Create an `Account` with an associated function and display it through an instance method. `Account::new` constructs
the value, while `display` reads it through an immutable `self` reference. This shows how behavior can live beside a
struct definition.

### [Enums](src/demo/enums.rs)

Represent user roles with an enum and select behavior using `match`. The example defines administrator, editor, and
guest roles, then prints the access level for the selected role. The match expression makes every enum variant
explicit.

### [Modules](src/demo/modules.rs)

Define a nested `profile` module and use its public `User` type. The module exposes a constructor while keeping the
module boundary clear. This introduces how `pub` controls access to items in another module.

### [The `use` keyword](src/demo/use_keyword.rs)

Import the `connect` function from a nested `network` module with `use network::connect`. The example then calls
`connect` directly instead of writing the full module path. This shows how `use` can make code that uses external
items easier to read.

### [External modules](src/demo/external_mod.rs)

Run code that is stored in its own Rust source file and included by the parent `demo` module. The example prints a
message and a small user profile from `external_mod.rs`. This shows how a larger program can organize behavior across
separate module files.

### [Vectors](src/demo/vectors.rs)

Create a mutable vector of scores and add values with `push`. The example reads the first value with indexing and
uses `get` to safely handle an index that does not exist. This introduces `Vec<T>` and the `Option` returned by safe
lookups.

### [Strings](src/demo/strings.rs)

Build an owned `String` by appending text and a character. The `format!` macro then combines the name and email into
a separate value without taking ownership of either input. This shows common ways to create and combine text in Rust.

### [Hash maps](src/demo/hash_maps.rs)

Create a `HashMap` that associates user details with scores. The example inserts key value pairs and retrieves one
value with `get` inside a `match` expression. This demonstrates how hash map lookups safely return an `Option`.

### [Panics](src/demo/panic.rs)

See how `panic!` immediately stops a program when it reaches an unrecoverable problem. This example intentionally
panics after printing a user profile message. Do not select this demo if you want `cargo run` to finish normally.

### [Results and file errors](src/demo/result.rs)

Try to open `hello.txt` and handle the returned `Result` with `match`. The example distinguishes a missing file from
other I O errors before it panics with an explanatory message. It introduces the structure of recoverable error
handling, although the current missing-file branch still ends the program deliberately.

### [Generics](src/demo/generics.rs)

Define a `Profile<T>` struct whose identifier can use different types. The example creates one profile with an
integer identifier and another with a string identifier. This shows how a generic type parameter allows one data
model to work with multiple kinds of values.

### [Traits](src/demo/traits.rs)

Define a `Summary` trait that requires a `summarize` method. The `User` struct implements that trait by formatting
its username and email address into a description. This shows how traits describe shared behavior that types can
choose to provide.

### [Lifetimes](src/demo/lifetimes.rs)

Compare two string slices and return the longest one with a named lifetime parameter. The `'a` annotation tells Rust
that the returned reference cannot live longer than either input reference. This introduces how lifetimes protect
references from outliving the data they point to.

## Prerequisites

Install Rust and Cargo through [rustup](https://rustup.rs/). Cargo builds, runs, and checks this project, so both
tools need to be available in your terminal. Confirm the installation with:

```bash
cargo --version
```

## Run the project

```bash
cargo run
```

The [demo selector](src/demo.rs) controls which example runs. Uncomment one `run()` call and comment out the
current call before using `cargo run`. Only one call should remain active so the output stays focused on one topic.

## Check the code

```bash
cargo check
cargo test
```

`cargo check` verifies that the project compiles without building a runnable binary. `cargo test` runs the test suite
when tests are added to the project.
