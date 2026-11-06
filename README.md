# Learn Rust

A small hands-on Rust project for building a solid foundation in the language. Rust is receiving a lot of
attention in the AI era, but understanding its core concepts comes first. The examples are intentionally small so
each idea can be read, run, and changed without getting lost in application complexity.

The examples in this repository cover:

### [Basic data types](src/demo/basic_dt.rs)

Learn about integer and floating point scalars, strings, booleans, and characters. The example gives each value an
appropriate Rust type and prints a small user profile. It is a starting point for seeing how static typing works.
You will use these types whenever a program needs to represent numbers, text, flags, or individual characters.

### [Variables and mutability](src/demo/variables.rs)

See how Rust bindings are immutable by default and how `mut` allows a value to change. The example updates a score
after it is created and prints both values. This shows when mutability must be declared explicitly.
Use this distinction to protect values that should not change and to make intentional updates clear in application code.

### [Functions](src/demo/functions.rs)

Practice passing parameters to functions and returning a value from a helper function. One helper prints user
information, while another adds two integers. The addition function shows Rust's expression based return style.
Functions are useful for giving a repeated task a name and keeping programs divided into small, readable pieces.

### [Control flow](src/demo/control_flow.rs)

Explore conditional expressions with `if`, `else if`, and `else`. The example assigns a grade message based on a
score and then uses another conditional expression to set a status. It shows that an `if` expression can produce a
value.
Use conditionals when a program must choose an action from data such as a score, user role, or configuration.

### [Ownership](src/demo/ownership.rs)

Understand heap allocation and move semantics with a `String` value. The example moves an email from one variable
to another, leaving the original variable unavailable. This is the central rule that prevents invalid memory access.
Ownership matters whenever code stores data on the heap or passes values between functions and parts of an application.

### [Borrowing and references](src/demo/references.rs)

Learn safe immutable and mutable access without transferring ownership. An immutable reference is used to calculate
an email length, and a mutable reference adds a last name. Both examples keep the original owner responsible for the
data.
Use references when a function needs to inspect or update data without taking it away from the caller.

### [Structs](src/demo/structs.rs)

Use a custom `UserProfile` type to group a username, email address, and active status. The example creates a struct
value and reads each of its fields. Structs make related data easier to model and pass around together.
They are useful for real world concepts such as users, orders, settings, or API responses with named fields.

### [Methods](src/demo/methods.rs)

Create an `Account` with an associated function and display it through an instance method. `Account::new` constructs
the value, while `display` reads it through an immutable `self` reference. This shows how behavior can live beside a
struct definition.
Methods are useful when an operation belongs to a value, such as validating an account or updating an order.

### [Enums](src/demo/enums.rs)

Represent user roles with an enum and select behavior using `match`. The example defines administrator, editor, and
guest roles, then prints the access level for the selected role. The match expression makes every enum variant
explicit.
Use enums when a value can be one of a fixed set of choices, such as a status, event, or message type.

### [Modules](src/demo/modules.rs)

Define a nested `profile` module and use its public `User` type. The module exposes a constructor while keeping the
module boundary clear. This introduces how `pub` controls access to items in another module.
Modules are useful for grouping related code and keeping a growing project easy to navigate.

### [The `use` keyword](src/demo/use_keyword.rs)

Import the `connect` function from a nested `network` module with `use network::connect`. The example then calls
`connect` directly instead of writing the full module path. This shows how `use` can make code that uses external
items easier to read.
Use `use` when a function, type, or trait from another module is needed often enough that its full path becomes noisy.

### [External modules](src/demo/external_mod.rs)

Run code that is stored in its own Rust source file and included by the parent `demo` module. The example prints a
message and a small user profile from `external_mod.rs`. This shows how a larger program can organize behavior across
separate module files.
Separate files are useful when one source file becomes too large to understand comfortably.

### [Vectors](src/demo/vectors.rs)

Create a mutable vector of scores and add values with `push`. The example reads the first value with indexing and
uses `get` to safely handle an index that does not exist. This introduces `Vec<T>` and the `Option` returned by safe
lookups.
Vectors are useful for ordered collections that can grow or shrink, such as scores, tasks, or search results.

### [Strings](src/demo/strings.rs)

Build an owned `String` by appending text and a character. The `format!` macro then combines the name and email into
a separate value without taking ownership of either input. This shows common ways to create and combine text in Rust.
Strings are used anywhere an application accepts, stores, formats, or displays text from users and external systems.

### [Hash maps](src/demo/hash_maps.rs)

Create a `HashMap` that associates user details with scores. The example inserts key value pairs and retrieves one
value with `get` inside a `match` expression. This demonstrates how hash map lookups safely return an `Option`.
Hash maps are useful for fast lookup by a key, such as finding a user by email or a product by identifier.

### [Panics](src/demo/panic.rs)

See how `panic!` immediately stops a program when it reaches an unrecoverable problem. This example intentionally
panics after printing a user profile message. Do not select this demo if you want `cargo run` to finish normally.
Reserve panics for situations a program cannot safely continue from, rather than expected input or file errors.

### [Results and file errors](src/demo/result.rs)

Try to open `hello.txt` and handle the returned `Result` with `match`. The example distinguishes a missing file from
other I O errors before it panics with an explanatory message. It introduces the structure of recoverable error
handling, although the current missing-file branch still ends the program deliberately.
Use `Result` when an operation can fail in an expected way, such as opening a file, calling a service, or parsing input.

### [Generics](src/demo/generics.rs)

Define a `Profile<T>` struct whose identifier can use different types. The example creates one profile with an
integer identifier and another with a string identifier. This shows how a generic type parameter allows one data
model to work with multiple kinds of values.
Generics are useful when the same behavior or structure should work with several types without copying code.

### [Traits](src/demo/traits.rs)

Define a `Summary` trait that requires a `summarize` method. The `User` struct implements that trait by formatting
its username and email address into a description. This shows how traits describe shared behavior that types can
choose to provide.
Traits are useful for writing code against a shared capability, even when different types provide it in their own way.

### [Lifetimes](src/demo/lifetimes.rs)

Compare two string slices and return the longest one with a named lifetime parameter. The `'a` annotation tells Rust
that the returned reference cannot live longer than either input reference. This introduces how lifetimes protect
references from outliving the data they point to.
You will encounter lifetimes when functions return references or when a type stores references to data owned elsewhere.

## Prerequisites

Install Rust and Cargo through [rustup](https://rustup.rs/). Cargo builds, runs, and checks this project, so both
tools need to be available in your terminal. Confirm the installation with:

Rustup keeps the Rust toolchain current, which is useful when following examples that need a recent compiler.

```bash
cargo --version
```

## Run the project

```bash
cargo run
```

The [demo selector](src/demo.rs) controls which example runs. Uncomment one `run()` call and comment out the
current call before using `cargo run`. Only one call should remain active so the output stays focused on one topic.
This approach is useful for learning because it lets you experiment with one concept at a time.

## Check the code

```bash
cargo check
cargo test
```

`cargo check` verifies that the project compiles without building a runnable binary. `cargo test` runs the test suite
when tests are added to the project.
Run these commands while making changes to catch mistakes before sharing or committing your work.
