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

