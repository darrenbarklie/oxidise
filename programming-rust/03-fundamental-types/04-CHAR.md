# CHARACTERS

Rust's character type `char`represents a single Unicode character at a 32-bit
value. Rust uses the `char` type for single characters in isolation, but
uses UTF-8 encoding for strings and streams of text. A `String` represents its
text as a sequence of UTF-8 bytes, not an array of `char`s.

Character literals are characters enclosed in single quotes: `'a'`.

Backslash escapes are required for a few characters, as with byte literals.

You can write character's unicode in hexadecimal:
- If within ASCII character set, write as `'\xHH'`
- If beyond, use hexadecimal up to six digits as `'\u{HHHHHH}'`
- Range always between 0x0000 to 0xD7FF, or 0xE000 to 0x10FFFF

Rust uses the type system and dynamic checks to ensure `char` stays in range.

```rust
assert_eq!('*' as i32, 42);
assert_eq!('ಠ' as u16, 0xca0);
assert_eq!('ಠ' as i8, -0x60);
```
