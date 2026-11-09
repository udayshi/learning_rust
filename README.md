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

## Run the active example

The active program is selected in [`src/main.rs`](src/main.rs). It currently runs the secure CLI store from
[`src/basic_utils/cli_secure_store.rs`](src/basic_utils/cli_secure_store.rs):

```bash
cargo run
```

To run another example, enable exactly one call in either [`src/basic_utils.rs`](src/basic_utils.rs) or
[`src/basic_demo.rs`](src/basic_demo.rs). To switch from a utility to a language lesson, change `main.rs` to call
`basic_demo::run()` instead of `basic_utils::run()`. Keep one active call at a time so the output stays focused.
Some examples require arguments, write local files, contact a local service, or deliberately stop the program; see
the notes below before enabling them.

### Secure CLI store

The active secure store accepts the following commands:

```text
add       # prompts for a key and a value, then encrypts and saves it
list      # lists saved records and tries to decrypt them
get:<key> # retrieves one saved record, for example: get:email
exit      # closes the program
```

It creates `rust_cli_secure.db` in the project directory. AES-256-GCM keys and nonces are generated with the
operating system's cryptographically secure random generator through the `aes-gcm` `getrandom` feature.

> **Learning-project limitation:** a new encryption key is generated for every program run and is not persisted.
> Records written during one run cannot be decrypted after restarting the application. Do not use this example to
> store real secrets.

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
| [`cli_secure_store.rs`](src/basic_utils/cli_secure_store.rs) | Interactive AES-256-GCM-encrypted key-value store backed by SQLite. The active example. |
| [`sqlite_crud.rs`](src/basic_utils/sqlite_crud.rs) | Create, read, update, and delete records in an in-memory SQLite database. |
| [`system_command.rs`](src/basic_utils/system_command.rs) | Runs `df -h /` and prints available disk-space information on Unix-like systems. |
| [`interactive_writer.rs`](src/basic_utils/interactive_writer.rs) | Appends typed lines to `uday_interactive_log.txt` until `exit` is entered. |
| [`file_read_write.rs`](src/basic_utils/file_read_write.rs) | Appends an audit entry to `uday_audit_log.txt`, then reads and prints it. |
| [`chatbot_ollama.rs`](src/basic_utils/chatbot_ollama.rs) | Sends prompts to a local Ollama `gemma3:4b` model over its HTTP API. |

### Utility-specific setup and effects

- `interactive_writer` and `file_read_write` create or append to local log files. These files are intentionally
  retained between runs.
- `system_command` requires the Unix `df` command and is not portable to Windows without changing the command.
- `chatbot_ollama` requires [Ollama](https://ollama.com/) to be installed, the server to be running, and the model
  to be available locally:

  ```bash
  ollama pull gemma3:4b
  ollama serve
  ```

  Enable `chatbot_ollama::run()` in `src/basic_utils.rs`, then start the application with `cargo run`. Type `exit`
  to close the chat session.

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
```

You can also run Clippy with:

```bash
cargo clippy -- -D warnings
```

Some intentionally simple learning examples may need to be modernized before the strict Clippy command passes.

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
