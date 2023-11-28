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

In real code, you usually won't need to write out the type suffixes, as context
will determine the type. However error messages can be surprising.

- `println!("{}", (-4).abs*());`
- => error: can't call method `abs` on ambiguous type `{interger}`

If signed integer types have `abs` method, what's the problem?
Rust wants to know the exactly which integer type a value has before it
will call the type's own methods. The default of `i32` only applies if
the type is till ambiguous after all method calls have been resolved.

## Checked, Wrapping, Saturating, and Overflowing Arithmetic

When an integer arithmetic operation overflows, Rust panics, in a debug
build. In a release build, then operation _wraps around_: it produces
the value equivalent to the mathmatically correct result modulo the range
of the value (maximum representable value for the integer type).

```rust
// In a release build
let x: u8 = 255;
let y = x + 1; // Wraps around to 0

let a: i8 = 127;
let b = a + 1; // Wraps around to -128
```

### Checked operations

Returns an `Option` of the result: `Some(v)` if the mathmatically correct
result can be represented as a value of that type, or `None` if it cannot.

```rust
// Can be represented as `u8`
assert_eq!(10_u8.checked_add(20), Some(30);

// Cannot be represented as `u8`
assert_eq!(100_u8.checked_add(200), None);

// Do the addition, panic if overflow
let sum = x.checke_add(y).unwrap();

// Oddly, signed division can overflow in one particular case
assert_eq!((-128_i8).checked_div(-1), None);
```

### Wrapping operations

This is how ordinary arithmetic operators behave in a release build. The
advantage being that they behave the same in all builds.

Returns the value equivalent to the mathematically correct result modulo
the range of the value:

```rust
// Represented as `u16`
assert_eq!(100_u16.wrapping_mul(200), 20000);

// Cannot be represented as `u16` ( 250,000 modulo 2^16)
assert_eq!(500_u16.wrapping_mul(500), 53392);

// Operations on signed types may wrap to negatives
assert_eq!(500_i16.wrapping_mul(500), -12144);

// In bitwise shift operations, shift distance is wrapped to
// fall within the size of the value. So shift of 17 bits in
// a 16-bit type is a shift of 1.
assert_eq!(5_i16.wrapping_shl(17), 10);
```


