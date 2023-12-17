# RC AND ARC: SHARED OWNERSHIP

Although most values have owers in Rust, in some cases it's difficult to find
every value a single owner that has the lifetime you need; you'd like the
value to simply live until until everyone is done using it.

For this, Rust provides `Rc` and `Arc` - reference-counted pointer types.

As expected from Rust, these are entirely safe to use: you cannot forget to
adjust the reference count, create other pointers for the referent that Rust
doesn't notice, or stumble over any other problems common to C++.

Both types are similar, except `Arc` is safe to share between threads directly,
the name stands for _atomic reference count_. `Rc` uses faster non-thread-safe
code to update its reference count. If you don't need to share the pointers
between threads, don't pay the performance penalty of `Arc`.

Use `Rc` to create reference counts to manage values' lifetimes:

```rust
use std::rc::Rc;

fn main () {
    
    let s: Rc<String> = Rc::new("stalemate".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();
    
    println!("{}", s);
    println!("{}", t);
    println!("{}", u);
    
    println!("Done!");
}
```

An `Rc<T>` value is a pointer to a heap-allocated `T` that has had a reference
count affixed to it. Cloning an `Rc<T>` value does not copy the `T`; instead
it simply creates another pointer to it and increments the reference count.

Above, each of the three `Rc<String>` pointers is referring to the same block
of memory, which holds a reference count and space for the `String`. The usual
ownership rules apply to the `Rc` pointers themselves. When the last extant
`Rc` is dropped, Rust drops the `String` as well.

You can use any of `String`'s usual methods directly on `Rc<String>`.

A value owned by an `Rc` pointer is immutable.

Rust's memory and thread-safety guarantees depend on ensuring that no value is
ever simultaneously shared and mutable. Rust assumes the referent of an `Rc`
pointer might in general be shared, so it must not be mutable.

A well-known problem with using reference counts to manage memory is that, if
there are ever two reference-counted values that point to each other, each
will hold the other's reference count above zero, so the values will never 
be freed. It is possible to leak values in Rust this way, but it's rare.

You can sometimes avoid creating cycles of `Rc` pointers by using _weak 
pointers_, `std::rc::Weak`, for some of the links instead.

Moves and reference-counted pointers are two ways to relax the rigidity of the
ownership tree. The third is way borrowing references.
