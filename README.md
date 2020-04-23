# libc_alloc

A simple global allocator for Rust which hooks into `libc`. Useful in `no_std` contexts.

It uses `posix_memalign` to allocate (i.e: malloc, but with the ability to specify alignment), and `free` to deallocate.

## Example

```rust
use libc_alloc::LibcAlloc;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;
```

## Project Status

Given how dead-simple this crate is, I doubt it will need to be updated very often (if at all).

Please file an issue and/or open a PR if you spot a bug / if something stops working.
