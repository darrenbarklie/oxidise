# STRING TYPES

Rust has string literals and two string types.

## String Literals

String literals are enclosed in double quotes. They use backslash escapes,
as per `char` literals.

```rust
let dictated = "\"This time\", he assured himself."
```
Single quotes don't need a backslash.

A string may span multiple lines.

If one line of a string end with a backslash, the newline character and 
following whitespace are dropped:

```rust
println!("You know in all of the times that I've shared \
    I've never been so scared.");
```

In some cases, doubling backslashes becomes a chore, so _raw strings_ can be
used to escape:

```rust
let default_win_install_path = r"C:\Program Files\Dir";
let pattern = Regex::new(r"\d+(\.\d+)*");
```

As no escape sequences are recognised, `r##" ... "##` format can create unique
escape sequence start and end points.

## Byte Strings

A string literal with `b" "` prefix is a slice of `u8` values. Bytes rather
than Unicode text. Escape with `br#" ... "#`.

```rust
let method = b"GET";
assert_eq!(method, &[b'G', b'E', b'T'];
```

The type of `method` is `&[u8; 3]` - a reference to an array of three bytes.

Byte strings can't contain arbitary Unicode characters, only ASCII and `\xHH`
escape sequences.

## Strings in Memory

Rust strings are sequences of Unicode characters, but they are not stored as
arrays of characters. They are stored using UTF-8, a variable-width encoding.
Each ASCII character in a string is one byte in size. Other charcters may
take multiple bytes.

```rust
let string1: String = "hello".to_string();
let string2 = &string1[1..];
let string3 = "Some string in memory";
let string4 = "ಠ_ಠ";
    
println!("{}", string2.len());
println!("{}", string4.len());
println!("{:?}", string4.chars());
println!("{:?}", string4.chars().count());
```

A `String` has a resizable buffer holding a UTF-8 text. The buffer is
allocated on the heap, so it can resize its buffer as needed or requested.

Think of a `String` as a `Vec<u8>` that holds UTF-8.

A `&str` ("stir" or "string slice") is a reference to a run of `UTF-8` text
owned by someone else: it "borrows" the text. `&str` is a fat pointer, like
other string references, containing both the address of the actual data
and its length.

Think of `&str` as a `&[u8]` that holds UTF-8.

A string literal is a `&str` that refers to preallocated text, typically
stored in read-only memory along with the program's machine code. It is
impossible to modify a `&str`.

For creating new strings at run time, use `String`.

## String

`&str` is very much like `&[T]`: a fat pointer to some data.

`String` is analogous to `Vec<T>`.

Like a `Vec`, each `String` has its own heap-allocated buffer that isn't
shared with any other `String`. When the `String` falls out of scope, the
memory is automatically freed.

There are several ways to create `Strings`:
- `.to_string()` converts a `&str` to a `String`
- `.to_owned()` converts as above, but also works on other types
- `format!()` macro returns a new `String` without new line finish
- Arrays, slices and vectors of strings create `Strings` via `.concat()`
  and `.join(sep)`

`&str` can refer to any sliceof any string, whether a string literal (stored
in the executable) or a `String` (allocated and freed at run time). This means
that `&str` is more appropriate for function arguments when the caller
should be allowed to pass either kind of string.

## Using Strings

Strings support operators: `==`, `!=`, `<`, `<=`, `=>`, `>`.

Beware that simple `char`-by-`char` comparison does _not_ always give the
expected answers. Ordering operators should also be used with care.

## Other String-Like Types

Rust guarentees that strings are valid UTF-8. Where you need to break out,
use _string-like types_:
- Stick to `String` and `&str` for Unicode text
- When working with filenames, use `std::path::PathBuf` and `&Path`
- When working with binary data, use `Vec<u8>` and `&[u8]`
- When working with env vars, use `OsString` and `&OsStr`
- When interoperating with C (null-terminated strings) use `std::ffi::CString`
  and `&CStr`

## Type Aliases

The `type` keyword can declare a new name for an existing type:

```rust
type Bytes = Vec<u8>;
```
