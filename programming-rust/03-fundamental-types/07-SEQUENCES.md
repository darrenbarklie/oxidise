# ARRAYS, VECTORS, SLICES

Rust has three types for representing a sequence of values in memory:

- The type `[T; N]` represents array of `N` values, each of type `T`. An
array's size is a constant computed at compile-time and is part of the type.
You cannot apprend new elements or shrink an array.
- The type `Vec<T>`, called a _vector of Ts_, is a dynamically allocated,
growable sequence of values of type `T`. A vector's elements live on the heap,
so can be resized at will.
- The types `&[T]` and `&mut [T]`, called a _shared slice of Ts_ and a 
_mutable shared slice of Ts_, are references to a series of elemetns that are
a part of some other value, like an array or vector. Think of a slice as a
pointer to its first element, together with a count of the number of elements
you can access starting at that point. A mutable slice lets you read and
modify elements, but can't be shared A shared slice let's you share access
across multiple readers, but doesn't let you modify elements.

Given a value of `v`:
- `v.len()`         number of elements in `v`
- `v[0]`            first element in `v` 
- `v[v.len() - 1]`  last element in `v`

Rust checks `i` always falls within range in `v[i]` or panics.

Length may be `0`, where accessing an index will panic.

`i` is measured as a `usize` value.

## Arrays

Several ways to write arrays. Simpliest:

```rust
let numbers: [u32; 5] = [1, 2, 3, 4, 5];
let animals = ["dog", "cat", "rabbit"];
```
