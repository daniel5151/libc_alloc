//! A simple global allocator for Rust which hooks into `libc`.
//! Useful when linking `no_std` + `alloc` code into existing embedded C code.
//!
//! Uses `posix_memalign` for allocations, `realloc` for reallocations, and
//! `free` for deallocations.
//!
//! ## Example
//!
//! ```
//! use libc_alloc::LibcAlloc;
//!
//! #[global_allocator]
//! static ALLOCATOR: LibcAlloc = LibcAlloc;
//! ```

#![no_std]

use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;

mod libc;

/// Global Allocator which hooks into libc to allocate / free memory.
pub struct LibcAlloc;

unsafe impl GlobalAlloc for LibcAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut ptr = core::ptr::null_mut();
        // `posix_memalign` returns an error int, but we can safely ignore it:
        // - EINVAL cannot occur, as layout.align() is guaranteed to be a power of two
        // - ENOMEM will leave `ptr` as a nullptr, which we return from this method to
        //   indicate an allocation failure.
        libc::posix_memalign(
            &mut ptr,
            core::mem::size_of::<usize>() * layout.align(),
            layout.size(),
        );

        ptr as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        libc::free(ptr as *mut c_void);
    }
}
