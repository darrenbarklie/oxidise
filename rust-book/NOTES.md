# Chapter 01

## Overview

Rust programming language is primarily about _empowerment_: to help developers program with confidence in a wider variety of domains than previously.

It makes systems-level requirements for memory management, data representation, concurrency and parallelism accessible with code that is reliable and efficient in terms of speed and memory usage. However it is expressive and ergonomic enough to make CLI apps and web servers pleasant to write.

Where high-level ergonomics and low-level control often conflict in programming language design, Rust offers optional low-level access, where code is prone to a variety of subtle bugs.

The Rust compiler plays a gatekeeper role by refusing to compile code with such elusive bugs. Rust is an _ahead-of-time compiled_ language, meaning the executable can be run without Rust installed.

## Tooling

Rust brings contemporary developer tools to systems programming:

- **Cargo** : dependency manager and build tool for painless adding, compiling and managing of dependencies
- **Rustfmt** : consistent coding style across developers
- **Rust Language Server** : powering IDE integration for code completion and inline linting

## Audience

Rust is ideal for many people for various reasons:

- **dev teams** : the compiler keeps developer collectives productive while producing systems-level code
- **students** : a welcoming community supports those interested in learning systems concepts
- **companies** : build CLIs, web services, DevOps tooling, embedded devices, audio and video analysis/transcoding, search engines, IoT applications, machine learning and even browsers

Rust is for people who crave _speed_ and _stability_ in a language. The Rust compiler ensures stability through feature additions and refactoring, minimising future maintenance of brittle legacy code that no dev dare touch.

By striving for zero-cost abstractions, higher-level features that compile to lower-level code as fast as code written manually, Rust endeavours to make _safe code_ be _fast code_.

Rust's greatest ambitions is to eliminate trade-offs programmers have accepted to decades, by providing:

- safety _and_ productivity
- speed _and_ ergonomics

By striving

## Getting Started

On Linux or macOS:

```bash
# Install rustup
$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
# => Rust is installed now. Great!

# Install C compiler
xcode-select --install

# Check version
rustc --version
# => rustc x.y.z (abcabcabc yyyy-mm-dd)

# Update
rustup update

# Docs
rustup doc
```

## Writing and Running a Rust Program

Create a new file called _main.rs_. Use snake case for files with multiple words: _hello_world.rs_. Rust style is to indent with four spaces. The `main()` function is always the first to run.

```rust
fn main() {
    println!("Hello, world!");
}
```

To compile and run the code:

```bash
rustc main.rs
./main
# => Hello, world!
```

`println!()` is called a macro, defined by the use of `!`. Macros don't always follow the rules of functions. Lines end with a semicolon, indicating the expression is over.

### Cargo

Cargo is Rust's build system and package manager, used to manage projects, build code, download and build dependencies. Use `cargo --version` to check installation and version number. Create a new project with _Cargo.toml_, _/src_, _/.git_ and _.gitignore_ by running:

```bash
# New Cargo project
cargo new hello_cargo
```

The _Cargo.toml_ file is authored in TOML: Tom's Obvious, Minimal Language format. `[package]` configures packages, where `[dependencies]` defines project dependencies; packages of code known as _crates_.

## Cargo Commands

These are the most commonly used commands:

- **`cargo --version`** : check installation/version
- **`cargo new project_name`** : create new project
- **`cargo doc`** : build dependencies docs
- **`cargo doc --open`** : open dependencies docs
- **`cargo build`** : build test binary in _target/debug_ directory
- **`cargo run`** : build and run in single command
- **`cargo check`** : faster compiler check without compilation step
- **`cargo build --release`** : build production binary in _target/release_ directory

---

# Chapter 02

## Input/Output

To ask for user input, process that input and validate the received input is in the required form is a common requirement for the majority of software.

To facilitate this, the Rust standard library (`std`) offers the `io` library:

```rust
use std::io;
```

## The Prelude

By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the [_prelude_](https://doc.rust-lang.org/std/prelude/index.html) which is focused on things, particularity traits, which are used in almost every single Rust program.

Preludes can be seen as a pattern to make using multiple types more convenient. Various libraries may define their own preludes. The difference between ‘the prelude’ and other preludes is that they are not automatically `use`d and must be manually imported, which is still more convenient than importing the constituent components.

## Creating Variables

Creates a new variable named `apples` and bind it to the value of 5:

```rust
let apples = 5;
```

In Rust, variables are immutable by default, meaning once a value is allocated it won't change.

```rust
let apples = 5; // immutable
let mut oranges = 5; // mutable
```

Here is a more realistic example:

```rust
let mut guess = String::new();
```

The equals sign tells Rust we want to bind something to the variable now. On the right of the equals sign is the value that the variable is bound to, which is the result of calling `String::new()`, a function that returns a new instance of a String. `String` is a growable, UTF-8 encoded string type, provided by the standard library.

The `::` syntax in the `::new` line indicates that `new` is an associated function of the `String` type. An _associated function_ is a function that is implemented on a type. You will find `new` on many types.

In full, `let mut guess = String::new();` creates a mutable variable bound to a new, empty instance of a `String` type.

---

# Chapter 03

## Variables and Mutability
