# REFERENCES

All the pointer types seen so far – `Box<T>` heap pointer and pointers
internal to `String` and `Vec` are owning pointers: when the owner is dropped,
the referent goes with it.

Rust also has non-owning pointer types called _references_, which have no
effect on their referents' lifetimes.

In fact, it is rather the opposite: references must never outlive their
referents. You must make it apparent in your code that no reference can
possibly outlive the value that it points to.

For empahsis, Rust refers to creating a reference to some value as _borrowing_
the value: what you have borrowed you must return to its owner.

Under the hood, references are just addresses, however the rules around them,
although difficult to master, prevent classic, everyday bugs and liberate
multithreaded programming practices.
