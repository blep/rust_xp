# rust_xp
Rust language learning and experimentation

Install using `rustup-init.exe`.

Then `rustup doc --book` allow you to view the rust book offline.

- rustup show: rust versions and target configuration
- rustup man rustc: supposed to show man page but panick.
- set RUST_BACKTRACE=1 environment variable to get stacktrace on assertion failure
- `rustc main.rs` to compile manually, but typically use `cargo` build tool instead

# CARGO

- Sources must be in src/ directory
- `cargo build` to build the project based on `Cargo.toml` package definition
- `cargo run` to build then run the project with debug configuration
- `cargo run --release` to build then run the project with release configuration
- Created `Cargo.lock` file is used to track version of the dependencies that where used. Reuse those version when file exist.
- `cargo new guessing_game --bin` creates a new project named guessing_game building an executable (project name cannot starts with number)
- In dependencies, version `0.3.0` indicates anything compatible with 0.3.0. Use `=0.3.0` to freeze exact version of the dependencies.
- `cargo update` check if update are available for dependencies and update `Cargo.lock` file with new version found.


# Cheet sheet
- default import: see std::prelude documentation
- `&var` pass var by immutable reference
- `&mut var` pass var by mutable reference
- `let var = ...` declares immutable variable `var`
- `let mut var = ...` declares mutable variable `var`
- `io::Result` is annotated to generate compiler error if .expect() method is not called. This is a specialization of std::Result
- `extern crate rand;` to use external library `rand` and implictly do a `use rand;`.
- can shadow variable declaration, typically done after parsing it.
- String::parse() infer parsing value constraint (sign allowed) from return type constraint.

## Printing

```
use std::io;


```

