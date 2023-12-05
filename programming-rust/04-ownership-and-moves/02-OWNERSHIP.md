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
