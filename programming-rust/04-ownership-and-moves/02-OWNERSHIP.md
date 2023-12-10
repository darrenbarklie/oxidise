# OWNERSHIP

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

A variable owns its value. When control leaves the block in which the variable
is declared, the variable is dropped, so its value is dropped along with it.

```rust
fn print() {
    let mut vector = vec![1,2,3];
    println!("{:?}", vector);
}                               // dropped here
```

The type of the variable `padovan` is `Vec<i32>`, a vector of 32-bit integers.
In memory, the variable sits on the stack and the buffer lives on the heap. The
words holding `padovan`'s pointer, capacity and length live directly in the
stack frame.

The vector owns the buffer holding its elements. Where the variable goes out
of scope at the end of the function, the program drops the vector. And since
the vector owns the buffer, the buffer goes with it.

## Box

A `Box<T>` is a pointer to a value of type `T` stored on the heap. Calling
`Box::new(v)` allocates some heap space, moves the value `v` into it and
returns a `Box` pointing to the heap space. Since the `Box` owns the space
it points to, when the `Box` is dropped, it frees the space too.

```rust
{
    let point = Box::new((0.625, 0.5));
    let label = format!("{:?}", point);
    assert_eq!(label, "(0.625, 0.5)");
}
```

When the program calls `Box::new`, it allocates space for a tuple of two `f64`
values on the heap, moves on its argument into that space and returns a pointer
to it.

The stack frame itself holds the variables `point` and `label`, each of which
refers to a heap allocation that it owns. When they are dropped, the
allocations they own are freed along with them.

Just as variables own their values, structs own their fields and
tuples, arraysm and vectors own their elements:

```rust
struct Person { name: String, birth: i32 }

let mut composers = Vec::new();
composers.push(Person { name: "Palestrina".to_string, birth: 1525 });
composers.push(Person { name: "Dowland".to_string, birth: 1563 });
composers.push(Person { name: "Lully".to_string, birth: 1632 });

for composer in &composers {
    println!("{}, born {}", composer.name, composer.birth);
}
```

There are many ownership relationships here:
- `composers` owns its elements, each a `Person` struct
- Each `Person` owns its fields
- The `String` field owns its text

When control leaves the scope in which `composers` is declared, the program
drops its value and takes the entire arrangement with it.

The owners and their owned values form _trees_: your owner is your parent,
and the values you own are your children. At the ultimate root of each tree
is a variable; when that variable goes out of scope, the entire tree goes
with it.

We have a tree built from a mixture of types, with Rust's single-owner rule
forbidding any rejoining of structure that could make th arrangement more
complexl than a tree. Every value in a Rust program is a member of tree,
rooted in some variable.

Rust programs don't usually explictly drop values at all. In Rust, to drop a 
value is to remove it from the ownership tree somehow; leaving the scope of a
variable, deleting an element from a vector, etc. Rust ensures the value is
properly dropped, along with everything it owns.

In a certain sense, Rust is less powerful than other languages. This allows
the analyses that can be performed be more powerful. Relationships it may
encounter in code are more tractable.

Rust extends ownership with some flexiblity:
- You can move values from one owner to another
- Simple types (integers, floats, characters) are excused as `Copy` types
- Stdlib has reference-counted `Rc` and `Arc` for multiple ownership logic
- "Borrow a reference" with non-owning pointers, with limited lifetimes

Each contributes flexibility while upholding Rust's core promises.
