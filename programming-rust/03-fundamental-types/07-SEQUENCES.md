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

To fill a long array with some value, you can write `[V; N]`, where `V` is
the value each element should have and `N` is the length. For example:
`[true; 10000]` is an array of 10,000 `bool`s initialised to `true`.

This syntax is used for fixed-size buffers: `[0_u8; 1024]` is a one-kilobyte
buffer filled with zeroes.

Rust has no notation for a non-initialised array. An array's length is part of
its type and fixed at compile time.

Iterating over elements, searching, sorting, filling, filtering, etc. are
provided as methods on slices, not arrays. Rust implicitly converts a reference
to an array to a slice when searching for methods.

You can call any slice method on an array directly:

```rust
let mut chaos = [3, 5, 1, 2, 4];
chaos.sort();
assert_eq!(chaos, [1, 2, 3, 4, 5]);
```

## Vectors

A vector `Vec<T>` is a resizable array of elements of type `T`, allocated to
the heap. The simpliest way to create a vector:

```rust
let mut primes = vec![2, 3, 5, 7];
assert_eq!(primes.iter().product::<i32>(), 210);

// Add elements dynamically
primes.push(11);
primes.push(13);
assert_eq!(primes.iter().product::<i32>(), 30030);
```

You can also build a vector by repeating a given value a certain number of
times, imitating array literals:

```rust
fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols]
}
```

The `vec!` macro is equivalent to calling `Vec::new` to create a new, empty
vector and then pushing the elements onto it:

```rust
let mut pets = Vec::new();
pets.push("Khoa");
pets.push("Chester");
pets.push("Popeye");
assert_eq!(pets, vec!["Khoa", "Chester", "Popeye"]);
```

Can also build a vector from the values produced by an iterator. You'll often
need to supply the type as `.collect()` can be used on different collections.

```rust
let v: Vec<i32> = (0...5).collect();
assert_eq!(v, [0, 1, 2, 3, 4, 5]);
```

As with arrays you can use slice methods on vectors. The call implicitly
borrows a `&mut [&str]` (for example) slice from the vector and invokes
the method on it.

`Vec` is an essential type to Rust – it's used almost anywhere one needs a
list of dynamic size.

`Vec<T>` consists of three values:
1. a pointer to the heap-allocated buffer for the elements created/owned
2. the number of elements that buffer has capacity to store
3. the number of elements currently contained now (length)

When the buffer reaches capacity, adding more entails allocating a larger
buffer, copying the contents across, updating vector's pointer and capacity
descriptions and freeing the old memory.

If you know the number of elements a vector needs in advance,
use `Vec::with_capacity` to hold all elements from the start,
without reallocation. Exceeding estimate enlarges as usual.

You can insert and remove elemetns wherever you like in a vector.

These operations shift all elements, so may be slow if the vector is long:

```rust
let mut v = vec![10, 20, 30, 40, 50];
v.insert(3, 35);
v.remove(1);
assert_eq!(v, [10, 30, 35, 40, 50]);
```

Use the `pop` method to remove the last element and return it:

```rust
let mut v = vec!["One", "Two"];
assert_eq!(v.pop(), Some("Two"));
assert_eq!(v.pop(), Some("One"));
assert_eq!(v.pop(), None);
```

Use a `for` loop to iterate over a vector:

```rust
let words = Vec<String> = std::env::args().skip(1).collect();
for w in words {
    println!("{}: {}", l,
        if l.len() % 2 == 0 {
            "even"
        } else {
            "odd"
        }
    )
}
# cargo run one two three four five six seven
```

## Slices

A slice written `[T]` without specifying the length, is a region of an array
or vector. Since a slice can be any length, slices cannot be stored directly
in variables or passed as function arguments.

Slices are always passed by reference.

A reference to a slice is a _fat pointer_: a two-word value comprising a
pointer to the slice's first element and the number of elements in the slice.

```rust
let v: Vec<f64> = vec![0.0,  0.707,  1.0,  0.707];
let a: [f64; 4] =     [0.0, -0.707, -1.0, -0.707];

let sv: &[f64] = &v;
let sa: &[f64] = &a;

fn print(n: &[f64]) 
    for elt in n {
        println!("{}", elt);
    }
}

print(sv);
print(sa);
```

For variables `sv` and `sa`, Rust automatically converts from `&Vec<f64>` and
`&[f64, 4]` types to `&[f64]` slice references to the data.

A reference to a slice is a non-owning pointer to a range of consecutive
values in memory. Slice references are a good choice to write a function that
operates on either an array or a vector. This allows for all methods available
on the slice type `[T]`.

To get a reference to a slice of an array or vector or existing slice:

```rust
print(&v[0..2]);
print(&a[2..]);
print(&sv[1..3]);
```

Rust checks all indices are valid, or panics.

Since slices are almost always behind references, we often just refer to types
like `&[T]` or `&str` as "slices".
