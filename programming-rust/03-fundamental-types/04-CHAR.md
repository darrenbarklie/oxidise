# CHARACTERS

Rust's character type `char`represents a single Unicode character at a 32-bit
value. Rust uses the `char` type for single characters in isolation, but
uses UTF-8 encoding for strings and streams of text. A `String` represents its
text as a sequence of UTF-8 bytes, not an array of `char`s.

Character literals are characters enclosed in single quotes: `'a'`.

Backslash escapes are required for a few characters, as with byte literals.
