# COPY TYPES

So far, we've shown values being moved involve vectors, strings, and other
types that could potentially use a lot of memory and be expensive to copy.
Move keeps ownership of such types clear and assignment cheap. But for
simplier types like integers or characters, this sort of hanlding isn't
necessary.

For example, assigning a `String` moves the value, whereas assigning an `i32`
copies it.

Assignment moves `String`s, so that we don't end up with two strings responsible
for freeing the same buffer. However an `i32` is simply a pattern of bits in
memory, so with no complex logic to maintain, we make an independent copy.

This means that values don't become uninitialised - there is no harm in
retaining the original values, the advantages of a move aren't applicable.

The exception to _most_ types being moved is the _Copy types_.

Assigning a value of a `Copy` type copies the value, rather than moving it.
The source of the assignment remains initialised and usable, with the same
value it had before. Passing `Copy` types to functions and constructors
behaves similarily.

The standard `Copy` type includes all the machine integer and floating-point
numeric types, the `char` and `bool` types, and a few others. A tuple or
fixed-size array of `Copy` types is itself a `Copy` type.

Only types for which a simple bit-for-bit copy suffices can be `Copy`. `String`
is not a `Copy` type because it owns a heap-allocated buffer. `Box<T>` is not
`Copy`; it owns the heap-allocated referent. `File` is not `Copy` as
duplicating such a value would entail asking the OS for another file handle.
`MutexGuard` isn't `Copy` as only one thread may hold a mutex at a time.

Any type that needs to do something special when a value is dropped cannot
be `Copy`.

For self-defined `struct` and `enum` types, they are also not `Copy`.

However, where sensible to do so (i.e. the `struct` holds only `i32` fields)
you can make the type `Copy` using attribute `#[derive(Copy, Clone)]`. The
compiler will check for correct usage.

The compiler doesn't apply auto-`Copy` behaviour on `struct` types, as it has
a big impact on how code is able to use it. Making a stype `Copy` represents
a serious commitment on the part of the implementer.

In Rust, a _move_ is a byte-for-byte, shallow copy that leaves the source
uninitialised, where a _copy_ is the same except it leaves the source
initialised. This reinforces that basic operations must remain simple, and 
that costs remain apparent to the programmer.
