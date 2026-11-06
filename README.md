# Learn Rust

A small, hands-on Rust project containing focused examples of core language concepts. Each lesson lives in its own
module so it can be read, run, and modified independently.

## Prerequisites

Install Rust and Cargo with [rustup](https://rustup.rs/), then confirm Cargo is available:

```bash
cargo --version
```

## Project layout

```text
src/main.rs                # Application entry point
src/basic_demo.rs          # Lesson selector and module declarations
src/basic_demo/*.rs        # Individual learning examples
src/basic_utils.rs         # Shared utility examples
tests/               # Integration-test examples
info.txt             # Sample input for file and search lessons
```

## Run the current lesson

The active lesson is selected in [`src/basic_demo.rs`](src/basic_demo.rs). It currently runs the basic data types
demo:

```bash
cargo run
```

To run another lesson, uncomment its `run()` call in `basic_demo::run()` and comment out the current one. Keep one
active call at a time so the output stays focused. Some lessons require arguments or deliberately stop the program;
see the notes below before enabling them.

For example, after enabling the iterator-based search lesson, pass a search query and file name after `--`:

```bash
cargo run -- Welcome info.txt
```

Set `IGNORE_CASE` to make that search case-insensitive:

```bash
IGNORE_CASE=1 cargo run -- welcome info.txt
```

## Lessons

| Topic | Module | What it demonstrates |
| --- | --- | --- |
| Basic data types | `basic_dt` | Scalar values and strings |
| Variables | `variables` | Immutability and `mut` |
| Functions | `functions` | Parameters and return values |
| Control flow | `control_flow` | `if`, `else`, and expression values |
| Ownership and borrowing | `ownership`, `references` | Moves, immutable references, and mutable references |
| Structs and methods | `structs`, `methods` | Custom types, constructors, and methods |
| Enums | `enums` | Variants and `match` |
| Modules | `modules`, `use_keyword`, `external_mod` | Visibility, imports, and separate module files |
| Collections | `vectors`, `strings`, `hash_maps` | Common standard-library collections |
| Errors | `panic`, `result` | Unrecoverable and recoverable error handling |
| Generics, traits, lifetimes | `generics`, `traits`, `lifetimes` | Reusable types, shared behavior, and reference lifetimes |
| Command-line arguments | `cli_args` | Reading arguments with `std::env` |
| File reading | `read_file` | Reading a text file from disk |
| Refactored I/O | `refactor_io` | Argument validation and file search |
| Iterator-based I/O | `iterators_io_run` | Iterator parsing and case-sensitive or insensitive search |
| Tests | `tests`, `control_tests` | Unit tests and ignored tests |

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

## Adding a lesson

1. Add a small, clearly named file under `src/basic_demo/`.
2. Declare the module in `src/basic_demo.rs`.
3. Add or enable its call in `basic_demo::run()`.
4. Put unit tests beside the lesson in a `#[cfg(test)]` module.
5. Update this README with any required inputs or special run commands.
