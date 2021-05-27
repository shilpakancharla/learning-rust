# Learning Rust

## Installation

To install Rust on macOS, run the command `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` in the terminal.

## Running programs (macOS)

To compile and run a Rust file, use `rustc main.rs` and then `./main`.

## Cargo

* Manages dependencies for repeatable builds
* Downloads and builds external libraries
* Calls `rustc` with correct parameters
* Use `cargo new <name>` to create a new project
* TOML: Tom's Obvious Minimal Language
* Change into working directory, use `cargo run` to compile and build all parts of the program. Creates `Cargo.lock` file that specifies dependencies. `target` folder has files generated from building the program.
* `cargo build --release`: builds program with optimization, result goes in `target-release` directory
