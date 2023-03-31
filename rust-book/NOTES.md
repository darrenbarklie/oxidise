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

Default variables are immutable, however, you still have the option to make your variables mutable. This promotes safety and easy concurrency. When a variable is immutable, once a value is bound to a name, you can’t change that value.

Attempting to reassign an inmutable variable will cause a compiler error.

### Contstants

Constants are values that are bound to a name and are not allowed to change (like immutable variables). Declared with the `const` keyword, they must be type annotated and may be set only to a constant expression, not the result of a value computed at runtime.

Constants can be declared in any scope, including the global scope, so are suited for values that many parts of the code need to know about. They are valid for the entire time a program runs, within the scope they were declared in.

Consts use a naming convention of all uppercase with underscores between words. The compiler is able to evaluate a limited set of operations at compile time, which allows for some expanded expression.

### Shadowing

You can declare a new variable with the same name as a previous variable. The first variable is _shadowed_ by the second — the second is what the compiler will see when you use the variable.

Shadowing differs from marking a variable as `mut`, because we’ll get a compile-time error if we reassign without using the `let` keyword.

By using `let`, we can perform transformations on a value but have the variable be immutable after those transformations have been completed.

As we’re effectively creating a new variable with shadowing, we can reuse the name but change the type.

## Data Types

Every value in Rust is of a certain data type.

Rust is a statically typed language — it must know the types of all variables at compile time.

There are two data type subsets:

- scalar
- compound

The compiler will usually infer the type based on usage. Where many types might be possible, we must declaure the type:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

Not offer a type will cause a compiler error.

### Scalar Type

A scalar type represents a single value:

- integers
- floating-point numbers
- Booleans
- characters

#### Integer Types

An integer is a number without a fractional component.

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8 bit   | i8     | u8       |
| 16 bit  | i16    | u16      |
| 32 bit  | i32    | u32      |
| 64 bit  | i64    | u64      |
| 128 bit | i128   | u128     |
| arch    | isize  | usize    |

Each variant can be either signed (has negatives) or unsigned (positives only) and has an explicit size.

Signed numbers are stored using [two’s complement](https://en.wikipedia.org/wiki/Two%27s_complement?useskin=vector) representation (the greatest place binary digit is used to indicate the sign).

Each signed variant can store numbers from -(2^n - 1) to (2^n - 1) - 1 inclusive, where n is the number of bits that variant uses.

The `isize` and `usize` types depend on the architecture of the computer your program is running on.

Integer literals can be written in any of the form:

| Number literals  | Example       |
| ---------------- | ------------- |
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |

If uncertain of what to use, Rust's default `u32` is a good start. `isize` or `usize` would be useful when indexing a collection.

[Integer overflow](https://rust-book.cs.brown.edu/ch03-02-data-types.html#integer-overflow) will error in debug mode and perform _two’s complement wrapping_ when compiled for release.

#### Floating-Point Types

Floating point numbers are numbers with decimal points: `f32` (single precision) and `f64` (double precision), with the default `f64` on modern systems, offering similar speeds but double precision.

#### Numeric Operations

Addition, subtraction, multiplication, division, and remainder are supported in Rust. Integer division rounds down to the nearest integer.

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}
```

Each operate evaluates to a single value which is then bound to the variable.

#### The Boolean Type

A Boolean type in Rust has two possible values: `true` and `false`. Booleans are one byte in size.

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

The main way to use Boolean values is through conditionals, such as an `if` expression for determining control flow.

#### The Character Type

Rust’s `char` type is the language’s most primitive alphabetic type.

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

We specify `char` literals with single quotes, as opposed to string literals, which use double quotes.

Rust’s `char` type is four bytes in size and represents a Unicode Scalar Value, capable of represeting ASCII, Chinese, Japanese and Korean characters, emjoi and zero-width spaces.

### Compound Types

Rust has two compound types for grouping multiple values into one type:

- tuples
- arrays

#### Tuple Type

General way of grouping togehter a number of values with a variety of types into one compound type. Tuples have fixed length: once declared they cannot grown or shrink in size.

Create tuples in parentheses with comma seperation. Each position in the tuple has a type, which can differ.

```rust
fn main() {
    let tup: (i64, f64, u8) = (500, 6.4, 1);
}
```

That variable `tup` binds to the entire tuple, because a typle is considered a single compound element.

We can use pattern matching to destructure a tuple value:

```rust
fn main () {
    let tup = (500, 6.4, 1)

    let (x, y, z) = tup;

    println!("The vlaue of y is: {y}");
}
```

We can also access a tuple element directly by using a period ( `.` ) followed by the index of the value we want to access (zero-based):

```rust
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
```

A type without any values is called a _unit_. This value and its corresponding type are both written `()` and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don't return any other value.
