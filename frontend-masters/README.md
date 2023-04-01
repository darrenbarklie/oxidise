# THE RUST PROGRAMMING LANGUAGE

## [Frontend Masters](https://frontendmasters.com/courses/rust/) / [Richarrd Feldman](https://frontendmasters.com/teachers/richard-feldman/)

### Macros

At compile time, macros are _expanded_ into a series of functions for processing. At run time there is no cost.
Macros allow for many different type of arguments, where functions require a fix number of arguments defined.

Macros are more powerful than functions, except for the ability to pass around first-class functions in Rust.

## 02. Primitives

### Strings

- `println!()`
- `format!()`

```rust
    // Print line
    println!("Hello, world!");

    // String interpolation
    let x = "Hello";
    let y = "world";
    println!("{}, {}!", x, y);

    // Return string
    format!("{}, {}!", x, y);

    // Return panic
    let crash_reason = "Server fault";
    panic!("App crashed: {}", crash_reason);
    // Program ends
    println!("This code is unreachable");
```

### Floats & Mutability

```rust
    let x = 1.1;
    let y = 2.2;

    println!("The result of x * y is {}", x * y);
    // => The result of x * y is 2.4200000000000004
```

It's good programming practice to leverage immutability, explicitly defining mutable values.

```rust
    let x = 1.1;
    x = 3.3;
    // => error: cannot assign twice to immutable variable `x`
    // Can't reassign and also can't mutate the value held
    // Mutable
    let mut z = 1.1;
    z = 2.2;
```

### Intergers

todo...

### Booleans

todo...

## Exercise

...
