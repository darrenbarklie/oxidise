# MOVES

For most types, operations like assigning a value to a variable, passing it
to a function, or returning it from a function don't copy the value: they
_move_ it. The source relinquishes ownership, the destination now controls
the value's lifetime.

Rust programs build up and tear down complex structures one value at a time,
one move at a time.

In Python, objects carry a reference count, tracking the number of values that
are currently referencing it. Python implements assignment simply by making
the destination point to the same object as the source, incrementing the
reference count.

In C++, assigning a `std::string` or `std::vector` produces a copy of the type.
This results in allocating three children and nine grandchildren for example.
C++ can consume unbounded amounts of memory and processor time. However it is
easy for the program to decide when to free all allocated memory.

Consider opposing trade offs:
- Python assignment is cheap, at the expense of reference counting and GC
- C++ keeps ownership of all memory clear, at the expense of deep copies

Comparing Rust with heap-allocated `Strings`, assignments of most types _move_
the value from the source to the destination, leaving the source uninitialised.
The elements stay where they are, but the ownership changes hands. References
to uninitialised will error at compile time.

The consequences are:
- Assignment is cheap (like Python)
- Ownership is always clear (like C++)
- Reference counts not required (like C++)
- Garbage collection not required (like C++)
- Must explicitly ask for copies when you want them (Rust)
- Must opt-in to reference counting with `Rc` and `Arc`

## More Move Operations

Beyond initialisations, moving variables drops the incumbant:

```rust
let mut s = "Incumbant".to_string();
s = "Replaced".to_string(); // value "Incumbant" dropped 

let mut t = "Incumbant".to_string();
let u = t;
t = "Replaced".to_string(); // nothing dropped
```

In the second example, `u` has taken ownership of the string from `t`, so
when assigned back to `t`, it was uninitialised – no string is dropped. 
