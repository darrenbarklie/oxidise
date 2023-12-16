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

Rust applies move semantics to almost any use of value.

Passing arguments to functions moves ownership to the function's parameters;
returning a value from a function move ownership to the caller. Building a
tuple moves the values into the tuple.

Moves can occur:
- during initiatlisation
- returning values from a function
- constructing new values
- passing values to a function

Moves always apply to the value proper, not the heap storage they own (the 
three word header alone and not the buffer location). Rust's compiler code
generation is very effecient at processing moves, so there is minimal cost.

## Moves and Control Flow

For more complicated code, it is possible for a variable to have its value
moved away, not have a new value defined and be considered _uninitialised_.
If a varibale still has a value after evaluating an `if` expressions's
condition, then we can use it in both branches:

```rust
let x = vec![10, 20, 30];
if c {
    f(x);   // ok to move from x to here
} else {
    g(x);   // also ok to move from x to here
}
h(x);       // error: x uninitialised when used in path
```

For similar reasons, moving from a variable to a loop is forbidden:

```rust
let x = vec![10, 20, 30];
while f() {
    g(x);   // error: x would be moved in first pass, uninitialised in second
}
```
That is, unless we've assigned a new value before next iteration:

```rust
let mut x = vec![10, 20, 30];
while f() {
    g(x);       // move from x
    x = h();    // give x new value
}
e(x);
```

## Moves and Indexed Content

Not every kind of value owner allows uninitialised values when attempting to
move ownership away.

For example, you cannot pull values our of a vector:

```
let v = vec![101, 102, 103, 104, 105];
let third = v[2];   // error: cannot move out of index of Vec
```

Rust's compiler will recommend using a reference, which is probably what you
want to do. However if you mean to move an element out:

```rust
let mut v = Vec::new();
for i in 101 .. 106 {
    v.push(i.to_string());
}

// 1. Pop a value off the end of the vector
let fifth = v.pop().expect("vector empty!");
assert_eq!(fifth, "105");

// 2. Move a value out of a given index in the vector,
//    and move the last element into its spot
let second = v.swap_remove(1);
assert_eq(second, "102");

// 3. Swap in another value for the one we're taking out:
let third = std::mem::replace(&mut v[2], "substitute".to_string());
assert_eq(third, "103");

// Remaining vector
assert_eq!(v, vec!["101", "104", "substitute"]);
```

Each one of these methods moves an element out of the vector, but does so
in a way that leaves the vector in a state that is fully populated, if
perhaps smaller.

Collection types like `Vec` also generally offer methods to consume all
their elements in a loop:

```rust
let v = vec!["dead".to_string(),
            "on".to_string(),
            "arrival".to_string()];

for mut s in v {
    s.push('!');
    println!("{}", s);
}
```

When we pass the vector to the loop directly, as in `for ... in v`, this
_moves_ the vector out of `v`, leaving `v` uninitialised. The `for` loops
internal machinery takes ownership of the vector and dissects it into its
elements. At each iteration, the loop moves another element to the
variable `s`.

Since `s` now owns the string, we're able to modify it in the loop body before
printing it. And since the vector iteself is not longer visible to the code
nothing can observe it mid-loop in some partially emptied state.

If you need to move a value out of an owner that the compiler can't track,
consider changing the owner's type to something that can dynamically track
whether it has a value or not. For example:

```rust
struct Person { name: Option<String>, birth: i32 }

let mut composers = Ve{c::new();
composers.push(Person { name: Some("Palestrina".to_string()),
                        birth: 1525 });

// expect error: cannot move out of index of `Vec<Person>`
// let first_name = composers[0].name;

let first_name = std::mem::replace(&mut composers[0].name, None);
```

The `replace` call moves the original value out of the heap location and
leaves `None` in its place, with is now a value type.

This is such a common requirement, it can be abbreviated with the `take`
method, which as the same effect as the above:

```rust
let first_name = composers[0].name.take();
```
