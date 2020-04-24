# libc_alloc

[![](http://meritbadge.herokuapp.com/libc_alloc)](https://crates.io/crates/libc_alloc)
[![](https://docs.rs/libc_alloc/badge.svg)](https://docs.rs/libc_alloc)

A simple global allocator for Rust which hooks into `libc`.
Useful when linking `no_std` + `alloc` code into existing embedded C code.

Uses `posix_memalign` for allocations, `realloc` for reallocations, and `free` for deallocations.

## Example

```rust
use libc_alloc::LibcAlloc;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;
```

## Project Status

Given how dead-simple this crate is, I doubt it will need to be updated very often (if at all).

Please file an issue and/or open a PR if you spot a bug / if something stops working.
