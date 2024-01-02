# REFERNCES TO VALUES

_Note: See code in 02-hash-table-no-ref_

Rust's standard library includes a hash table type:

```rust
use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;
```

When looping over data, avoid the inner loops taking ownership by using
references. A reference lets you access a value without affecting its
ownership. References come in two kinds:

- **shared reference**: Let's you read, but not modify its referent. You can
  have as many shared references to a particular value at a time as you like.
  The expression `&e` yields a shared reference to `e`'s value. If `e` has
  type `T`, then `&e` has type `&T`, pronounced "ref T". Shared refs are `Copy`
- **mutable reference**: May necessitate both read and write, howeer you may
  not have any other references of any sort to that value active. The
  expression `&mut e` yields a mutable reference to `e`'s value; you write its
  type as `&mut T`, which is pronounced "ref mute T". They are **not** `Copy`
