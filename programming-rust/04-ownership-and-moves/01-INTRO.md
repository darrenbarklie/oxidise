# INTRO

When it comes to managing memory, there are two desirable characteristics:
- Memory is freed promptly
- Never want to use a pointer to an object after it's been freed

However traditionally these are mutually exclusive: freeing a value while
pointers exist to it necessarily leaves those points dangling.

- The "safety first" camp uses garbage collection to manage memory,
  automatically freeing objects when all reachable pointers to them are gone.
  This eliminates dangling points by simply keeping the objects around until
  there are no pointers to them left to dangle. Almost all modern languages
  fall in this camp: Python, JavaScript, Ruby, Java, C#, Haskell.

- The "Control First" camp leaves you in charge of freeing memory. The
  program's memory consumption is entirely in your hands, but avoiding
  dangling pointers also becomes entirely the developers concern. C and C++
  are the only mainstream languages in this camp. Pointer misuse has been a
  common culprit in reported security problems since computing began!

Rust aims to be safe _and_ perfomant.

Rust innovates by restricting how your programs can use pointers.

This allows Rust's compile-time checks to verify that your program is free
of memory safety errors: dangling pointers, double frees, using uninitialised
memory, etc. The same rules are the basis for safe concurrent programming, by
using Rust's threading primitives, ensuring a lack of data races. A bug in
one thread cannot corrupt another, thanks to non-deterministic features;
mutexes, message channels, atomic values, etc.

## Ownership

In programming, the "owning" object get to decide when to free the owned
object and when the owner is destroyed, it destroys its possessions with it.

The actual object is always exactly _three words long_; a pointer to a
heap-allocated buffer, a capacity and a length. The fields are private to the
class and not accessible to the object's users.

As the object owns the buffer, when the program destroys the object, the
object's destructor frees the buffer.

Although it's fine for other code to create temporary points to the owned
memory, it's that code's responsibility to ensure it's pointers are gone
before the owner decides to destroy the owned object.

The owner determines the lifetime of the owned.

However in Rust, the concept of ownership is built in to the language iteself
and is enforced by compile-time checks. Every value has a single owner that
determines its lifetime. When the owner is free – _dropped_ - the owned value
is dropped too.
