# rust_xp
Rust language learning and experimentation

Install using `rustup-init.exe`.

Then `rustup doc --book` allow you to view the rust book offline.

- rustup show: rust versions and target configuration
- rustup man rustc: supposed to show man page but panick.
- set RUST_BACKTRACE=1 environment variable to get stacktrace on assertion failure. `RUST_BACKTRACE=full` for more details.
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


# Cheat sheet
- default import: see std::prelude documentation
- `io::Result` is annotated to generate compiler error if .expect() method is not called. This is a specialization of std::Result
- `extern crate rand;` to use external library `rand` and implictly do a `use rand;`.
- String::parse() infer parsing value constraint (sign allowed) from return type constraint.

## Variable
- `&var` pass var by immutable reference
- `&mut var` pass var by mutable reference
- `let var = ...` declares immutable variable `var`
- `let mut var = ...` declares mutable variable `var`
- can shadow variable declaration, typically done after parsing it.
- `let (x, y) = (1, 2);`
- `let x: i32 = 5;`
- `let f: fn(i32) -> i32 = plus_one;`
- `let y: bool = false;`
- `let x = 'x';` declare a char (32bits unicode code point)
- `let mut m = [1, 2, 3]; // m: [i32; 3]`
- `let a = [0; 20]; // a: [i32; 20]` // initialize all 20 elements to 0
- `let complete = &a[..];` // A slice containing all of the elements in `a`.
- `let middle = &a[1..4];` // A slice of `a`: only the elements `1`, `2`, and `3`.
- `let x: (i32, &str) = (1, "hello");`
- `let y = if x == 5 { 10 } else { 15 };` // y: i32

## for loop

- for x in 0..10 { .. } // 10 is not included (like python range)
- for x in 0..10 { .. } // like python

```
'outer: for x in 0..10 {
    'inner: for y in 0..10 {
        if x % 2 == 0 { continue 'outer; } // Continues the loop over `x`.
        if y % 2 == 0 { continue 'inner; } // Continues the loop over `y`.
        println!("x: {}, y: {}", x, y);
    }
}
```

## collections

- `let v = vec![1, 2, 3, 4, 5];` // v: Vec<i32>
- `let v = vec![0; 10];` // A vector of ten zeroes.
- `println!("The third element of v is {}", v[2]);`

# Types
- i8, i16, i32, i64, u8, u16, u32, u64
- f32, f64
- isize, usize // pointer sized

## Printing

```
use std::io;


```

