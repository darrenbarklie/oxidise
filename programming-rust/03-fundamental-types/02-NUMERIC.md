# FIXED-WIDTH NUMERIC TYPES

Fixed-width numeric types in Rust were chosen to match the types that almost all
modern processors implemnt directly in hardware.

Fixed-width numeric types can overflow or lose precision, but are adequate for
most applications and are thousands of times faster than representations such as
arbitrary-precision integers and exact rationals (available in `num` crate).

## Numeric types

| Size    | Unsigned | Signed   | Floating |
|---------|----------|----------|----------|
| 8       | u8       | i8       |          |
| 16      | u16      | i16      |          |
| 32      | u23      | i32      | f32      |
| 64      | u64      | i64      | f64      |
| 128     | u128     | i128     |          |
| Machine | usize    | isize    |          |

Rust treats characters as distinct from the numeric types: a `char` is not a `u8`,
nor is it a `u32`.

Rust requires array indices (and other such representations) to be `usize` types.

Integer literals can take a type as a suffix, for example `42u64` or `1729_isize`.
If no type is defined, it will try to infer from usage, otherwise default to `i32`.
Rust will error where this creates a problem.

The prefixes `0x`, `0o` and `0b` designate hexidecimal, octal and binary literals.

## Byte Literals

Character-like literals for `u8` values: `b'X'` represents the ASCII code for the
character X, as a `u8` value. `b'A'` and `65u8` are exactly equivalent. Only
ASCII characters my appear in byte literals.

Some characters require _stand-in notation_:

- Single quote      `b'\''`
- Backslash         `b'\\'`
- Newline           `b'\n'`
- Carriage return   `b'\r'`
- Tab               `b'\t'`

For characters that are hard to read or write, you can write their code in
hexidecimal instead, in the form of `b'\x1b'` for 'escape'.

## Type casts

Convert one integer type to another with the `as` keyword, for example:

`assert_eq!(    10_u8 as u16,    10_u16);   // in range`

## Operations

The standard library provides some operation as methods on integers. For example:

- `2_u16.pow(4)`        exponentiation
- `(-4_i32).abs()`      absolute value
- `u8_count_ones()`     population count
