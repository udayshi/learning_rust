# Learn Rust

A small, hands-on Rust project containing focused examples of core language concepts. Each lesson lives in its own
module so it can be read, run, and modified independently.

## Prerequisites

Install Rust and Cargo with [rustup](https://rustup.rs/), then confirm Cargo is available:

```bash
cargo --version
```

## Project layout

| Path | Purpose |
| --- | --- |
| [`src/main.rs`](src/main.rs) | Application entry point and active program selector. |
| [`src/basic_demo.rs`](src/basic_demo.rs) | Learning-example module declarations and lesson selector. |
| [`src/basic_demo/`](src/basic_demo/) | Individual Rust language learning examples. |
| [`src/basic_utils.rs`](src/basic_utils.rs) | Utility-example module declarations and selector. |
| [`src/basic_utils/`](src/basic_utils/) | Practical utility examples, including SQLite and system commands. |
| [`tests/integration_test.rs`](tests/integration_test.rs) | Integration-test example. |
| [`info.txt`](info.txt) | Sample text input for file-reading and search examples. |

## Run the current lesson

The active program is selected in [`src/main.rs`](src/main.rs). It currently runs the in-memory SQLite CRUD example
from [`src/basic_utils/sqlite_crud.rs`](src/basic_utils/sqlite_crud.rs):

```bash
cargo run
```

To run a learning lesson instead, change `main.rs` to call `basic_demo::run()`. Then uncomment its `run()` call in
`basic_demo::run()` and comment out the current one. Keep one active call at a time so the output stays focused.
Some lessons require arguments or deliberately stop the program; see the notes below before enabling them.

For example, after enabling the iterator-based search lesson, pass a search query and file name after `--`:

```bash
cargo run -- Welcome info.txt
```

Set `IGNORE_CASE` to make that search case-insensitive:

```bash
IGNORE_CASE=1 cargo run -- welcome info.txt
```

## Learning examples

Select a lesson by enabling its call in [`src/basic_demo.rs`](src/basic_demo.rs). Each file below is a direct link to
the example source.

| Example | Description |
| --- | --- |
| [`basic_dt.rs`](src/basic_demo/basic_dt.rs) | Basic scalar data types, strings, booleans, and characters. |
| [`variables.rs`](src/basic_demo/variables.rs) | Immutable bindings and intentional mutation with `mut`. |
| [`functions.rs`](src/basic_demo/functions.rs) | Function parameters, return values, and expression-based returns. |
| [`control_flow.rs`](src/basic_demo/control_flow.rs) | Conditional branches with `if`, `else if`, and `else`. |
| [`ownership.rs`](src/basic_demo/ownership.rs) | Ownership transfer and move semantics. |
| [`references.rs`](src/basic_demo/references.rs) | Immutable and mutable borrowing without transferring ownership. |
| [`structs.rs`](src/basic_demo/structs.rs) | Struct definitions and field access. |
| [`methods.rs`](src/basic_demo/methods.rs) | Associated functions and instance methods. |
| [`enums.rs`](src/basic_demo/enums.rs) | Enum variants and exhaustive `match` expressions. |
| [`modules.rs`](src/basic_demo/modules.rs) | Nested modules and visibility with `pub`. |
| [`use_keyword.rs`](src/basic_demo/use_keyword.rs) | Importing module members with `use`. |
| [`external_mod.rs`](src/basic_demo/external_mod.rs) | Organizing code into a separate module file. |
| [`vectors.rs`](src/basic_demo/vectors.rs) | Growable vectors, indexing, and safe lookup with `get`. |
| [`strings.rs`](src/basic_demo/strings.rs) | Constructing, modifying, and formatting owned strings. |
| [`hash_maps.rs`](src/basic_demo/hash_maps.rs) | Key-value storage and optional hash-map lookups. |
| [`panic.rs`](src/basic_demo/panic.rs) | An intentional unrecoverable error with `panic!`. |
| [`result.rs`](src/basic_demo/result.rs) | Recoverable file errors with `Result` and `match`. |
| [`generics.rs`](src/basic_demo/generics.rs) | Generic structs that work with multiple data types. |
| [`traits.rs`](src/basic_demo/traits.rs) | Defining and implementing shared behavior with traits. |
| [`lifetimes.rs`](src/basic_demo/lifetimes.rs) | Lifetime annotations for returned references. |
| [`cli_args.rs`](src/basic_demo/cli_args.rs) | Reading command-line arguments through `std::env`. |
| [`read_file.rs`](src/basic_demo/read_file.rs) | Reading a text file from disk. |
| [`refactor_io.rs`](src/basic_demo/refactor_io.rs) | Validating CLI input and searching file contents. |
| [`iterators_io_run.rs`](src/basic_demo/iterators_io_run.rs) | Iterator-based argument parsing and optional case-insensitive search. |
| [`tests.rs`](src/basic_demo/tests.rs) | Basic unit-test assertions. |
| [`control_tests.rs`](src/basic_demo/control_tests.rs) | Passing and ignored unit tests. |

## Utility examples

Select a utility in [`src/basic_utils.rs`](src/basic_utils.rs).

| Example | Description |
| --- | --- |
| [`sqlite_crud.rs`](src/basic_utils/sqlite_crud.rs) | Create, read, update, and delete records in an in-memory SQLite database. |
| [`system_command.rs`](src/basic_utils/system_command.rs) | Runs `df -h /` and prints the available disk-space information. |

### Lesson-specific commands

After selecting the appropriate lesson in `src/basic_demo.rs`, use these commands where required:

```bash
# Show command-line arguments
cargo run -- first second

# Run the refactored I/O search lesson
cargo run -- Welcome info.txt

# Run only ignored tests
cargo test -- --ignored

# Run the integration-test example
cargo test --test integration_test
```

The `panic` lesson intentionally terminates the process. The `result` lesson attempts to open `hello.txt`; create
that file first if you want it to complete normally. The `read_file` lesson is preconfigured to read `info.txt` when
enabled.

## Check and test

Run these commands before sharing changes:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy -- -D warnings
```

Apply Rust formatting with:

```bash
cargo fmt
```

## Build for Linux

The normal release build for the current machine is:

```bash
cargo build --release
```

To build a Linux x86_64 binary from another operating system, Cargo needs both the Rust target standard library and
a compatible Linux C cross-compiler. The SQLite dependency is configured with the `bundled` feature, so it compiles
SQLite's C source during the build.

```bash
cargo build --target x86_64-unknown-linux-gnu --release
```

On macOS, this command fails until the `x86_64-unknown-linux-gnu` Rust target and an
`x86_64-linux-gnu-gcc`-compatible C compiler are installed. If Rust was installed with rustup, add the target with:

```bash
rustup target add x86_64-unknown-linux-gnu
```

Building inside an x86_64 Linux Docker environment is often the simplest option because it provides the Linux build
environment directly. A successful cross-build writes the executable to:

```text
target/x86_64-unknown-linux-gnu/release/rust_practise
```

## Adding a lesson

1. Add a small, clearly named file under `src/basic_demo/`.
2. Declare the module in `src/basic_demo.rs`.
3. Add or enable its call in `basic_demo::run()`.
4. Put unit tests beside the lesson in a `#[cfg(test)]` module.
5. Update this README with any required inputs or special run commands.
