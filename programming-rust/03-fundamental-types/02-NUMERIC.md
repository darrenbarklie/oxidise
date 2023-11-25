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

