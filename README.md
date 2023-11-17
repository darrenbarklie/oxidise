# Oxidise

## Overview

The purpose of this repository is to capture my Rust learning journey.

As I consume learning materials and complete practice projects, I'll maintain this resource as a singular location for code, resources and reflections.

## Resources

- [The Rust Programming Language](https://doc.rust-lang.org/book/)

## Commands

```zsh
# Check version
rustc --version

# Update to latest version
rustup update

# Open local Rust docs
rustup doc
rustup doc --std

# Compile a Rust program
rustc main.rs
# => ./main

# New Cargo project
cargo new hello_cargo

# Compile with Cargo
cargo build

# List all installed crates
cargo install --list

# Clean out cargo cache
cargo clean

# View help files for command
cargo help install

# Watch .src in quiet mode, clearing
cargo watch -q -c -w src/ -x run
```
