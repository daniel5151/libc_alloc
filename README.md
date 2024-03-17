# libc_alloc

[![](http://meritbadge.herokuapp.com/libc_alloc)](https://crates.io/crates/libc_alloc)
[![](https://docs.rs/libc_alloc/badge.svg)](https://docs.rs/libc_alloc)

A simple global allocator for Rust which hooks into `libc` functions.
Useful when linking `no_std` + `alloc` code into existing C codebases.

On Unix-like OSs, use `memalign` for allocations, and `free` for deallocations.

On macOS, use `posix_memalign` for allocations, and `free` for deallocations.

On Windows, use native [`_aligned_malloc`](https://docs.microsoft.com/en-us/cpp/c-runtime-library/reference/aligned-malloc) for allocations, [`_aligned_realloc`](https://docs.microsoft.com/en-us/cpp/c-runtime-library/reference/aligned-realloc) for reallocations, and [`_aligned_free`](https://docs.microsoft.com/en-us/cpp/c-runtime-library/reference/aligned-free) for deallocations.

## Example

```rust
use libc_alloc::LibcAlloc;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;
```

Alternatively, with the `global` Cargo feature, the crate only needs to be pulled in:

```rust
extern crate libc_alloc;
```

## Project Status

Given how dead-simple this crate is, I doubt it will need to be updated very often.

Please file an issue and/or open a PR if you spot a bug / if something stops working.
