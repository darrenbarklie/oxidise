# TYPES

Rust is designed around its types.

Developers can choose the best data representation to balance simiplicity and cost.

Source level types have concrete machine-level counterparts with predictable costs and performance.

Type system = memory and thread safety.
Generics and traits = flexibility.

## Type Inference

Rust's type inference occurs when the type can be deduced from usage or return type. In such
cases, the type is elided. Rust will produce the same machine code regardless, or error.

Functions can be _generic_: a single function can work on values of many different types. Generic types
functions give the language a degree of the same flexibility, while catching errors at complile time.

Despite their flexibility, generic functions are just as effecient as their nongeneric counterparts.

## Types in Rust

- `i8`, `i16`, `i32`, `i64`, `i128`     Signed integer
- `u8`, `u16`, `u32`, `u64`, `u128`     Unsigned integer
- `isize`, `usize`                      Same size as machine address (32 or 64 bits)
- `f32`, `f64`                          IEEE floating point number (single, double precision)
- `bool`                                Boolean
- `char`                                Unicode character (32 bits)
- `(char, u8, i32)`                     Tuple (mixed types permitted)
- `()`                                  Unit (empty tuple)
- `struct S { x: f32, y: f32}`          Named-field struct
- `struct T (i32, char)`                Tuple-like struct
- `struct E`                            Unit-like struct (no fields)
- `enum Attend { OnTime, Late(u32) }`   Enumeration
- `Box<Attend>`                         Box (owning point to value in heap)
- `&i32, &mut i32`                      Shared references (non-owning pointers mustn't outline referent)
- `String`                              UFT-8 string (dynamic size)
- `&str`                                Reference to `str` (non-owning pointer)
- `[f64; 4], [u8; 256]`                 Array, fixed length, elements of same type
- `Vec<f64>`                            Vector, varying length, elements of same type
- `Option<&str>`                        Optional value, either `None` or `Some(v)` with value `v`
- `Result<u64, Error>`                  Result of operation, success `Ok(v)` or `Err(e)`
- `&dyn Any, &mut dyn Read`             Trait object, reference that implements given set of methods
- `fn(&str) -> bool`                    Pointer to function
- `-`                                   Closure `|a, b| { a*a, + b*b }`

- `&[u8], &mut [u8]`                    Reference to slice, portion of array/vec; pointer, length

