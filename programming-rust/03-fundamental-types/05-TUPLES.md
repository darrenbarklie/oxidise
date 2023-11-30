# TUPLES

A tuple is a collection of assorted types: `("Belfast", 1985)`

Access via a const only: `x.4` and not `x.i` or `x[i]`

A tuple is not an array, as in other languages.

Tuple's name is dervived from "pair", "triple", "quadruple", "quintuple", etc.
Hence _n-tuple_ or _tuple_.

Rust code often uses tuple types to return multiple values from a function:

```rust
fn split_at(&self, mid: usize) -> (&str, &str);
```

You can use pattern-matching syntax to assign each element to a variable:

```rust
let text = "All my life I've been searching for something"
let (head, tail) = text.split_at(12);
```

A zero-tuple `()` (unit type) is used when there is no meaningful value to
carry, but requires some sort of type none-the-less.

Trialing commas are valid and encouraged in tupels, as well as function
arguments, arrays, struct definitions and enum definitions. They must be
used to distinguish a singleton tuple from a simple parenthetic expression.
