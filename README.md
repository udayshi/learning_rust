# Learn Rust

A small hands-on Rust project for building a solid foundation in the language. Rust is receiving a lot of
attention in the AI era, but understanding its core concepts comes first. Without the basics, it is easy to get lost.

The examples in this repository cover:

### [Basic data types](src/demo/basic_dt.rs)

Learn about scalars, strings, booleans, characters, and static typing.

### [Variables and mutability](src/demo/variables.rs)

See how Rust bindings are immutable by default and how `mut` allows values to change.

### [Functions](src/demo/functions.rs)

Practice parameters, return values, and expression-based returns.

### [Control flow](src/demo/control_flow.rs)

Explore conditional expressions with `if`, `else if`, and `else`.

### [Ownership](src/demo/ownership.rs)

Understand heap allocation and move semantics.

### [Borrowing and references](src/demo/references.rs)

Learn safe immutable and mutable access without transferring ownership.

### [Structs](src/demo/structs.rs)

Use custom data types to group related fields.

### [Methods](src/demo/methods.rs)

Create an `Account` with an associated function and display it through an instance method.

### [Enums](src/demo/enums.rs)

Represent user roles with an enum and select behavior using `match`.

### [Modules](src/demo/modules.rs)

Define a nested `profile` module and use its public `User` type.

## Prerequisites

Install Rust and Cargo through [rustup](https://rustup.rs/), then confirm the installation:

```bash
cargo --version
```

## Run the project

```bash
cargo run
```

The [demo selector](src/demo.rs) controls which example runs. Uncomment one `run()` call and comment out the
current call before using `cargo run`.

## Check the code

```bash
cargo check
cargo test
```
