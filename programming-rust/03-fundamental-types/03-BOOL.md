# BOOL TYPE

Two values: `true` and `false`

Comparison operators like `==` and `<` produce `bool` results: `2 < 5` is
`true`.

Rust is strict with control structures, requiring conditions to be `bool`
expressions, as do short-circuiting operators. You must write
`if x != 0 { ... }` not simply `if x { ... }`.

Rust's `as` operator can convert `bool` values to integer types:

```rust
assert_eq!(false as i32, 0);
assert_eq!(true  as i32, 1);
```
