# POINTERS

Rust has several types that represent memory addresses.

The lanaguage is designed to help keep allocations to a minimum. Values nest
by default. The value `((0,0,), (1440, 900))` is stored as four adjacent
integers. If you store in a local variable, you've got a local variable four
integers wide. Nothing is allocated in the heap.

This is great for memory effeciency, but means Rust must use pointer types
explicitly. However, the benefit is they are constrained to eliminate
undefined behaviour.

## References

A value of type `&String` is pronounced "ref String".
A value of type `i32` is a "reference to an `i32`"

Get started by thinking of references as Rust's basic pointer type.

At run time, a reference to an `i32` is is a single machine word holding
the address of the `i32`, which may be on the stack or the heap.

The expression `&x` produces a reference to `x`.

In Rust we say that is "borrows a reference to `x`".

Given a reference `r`, the expression `*r` refers to the value `r` points to.

A reference does not automatically free any resources when it goes
out of scope.

Rust references are never null, there is no way to produce a null reference in
safe Rust. Rust tracks the ownership and lifetimes of values, so mistakes like
dangling pointers, double frees, and pointer invalidation are ruled out at
compile time.

Rust references comes in two flavours:

- `&T`      immutable shared reference
- `&mut T`  mutable exclusive reference

Rust uses this dichotomy between shared and mutable references to enforce a
"single writer _or_ multiple readers" rule.

You can read and write the value, or it can be shared by any number of readers,
but never both at the same time. This separation, enforced by compile-time
checks, is central to Rust's safety guarantees.

## Boxes

The simpliest way to allocate a value in the heap is to use `Box::new:`

```rust
let t = (12, eggs);
let b = Box::new(t);
```

The type of `t` is `(i32, &str)` so the type of `b` is `Box<(i32, &str)>`.

The call to `Box::new` allocates enough memory to contain the tuple on the
heap. When `b` goes out of scope, the memory is freed immediately, unless `b`
has been _moved_.

## Raw Pointers (Unsafe)

Rust also has the raw pointer types `*mut T` and `*const T`.

Using a raw pointer is unsafe, as Rust makes no effort to track it points to.
