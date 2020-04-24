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
use core::ptr;

mod libc;

/// Global Allocator which hooks into libc to allocate / free memory.
pub struct LibcAlloc;

unsafe impl GlobalAlloc for LibcAlloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut ptr = ptr::null_mut();
        // `posix_memalign` returns an error int, but it can be safely ignored it:
        // - EINVAL cannot occur, as layout.align() is guaranteed to be a power of two
        // - ENOMEM will leave `ptr` as a nullptr, which is returned from this method to
        //   indicate an allocation failure.
        libc::posix_memalign(
            &mut ptr,
            layout.align().max(core::mem::size_of::<usize>()),
            layout.size(),
        );

        ptr as *mut u8
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // Unfortuantely, calloc doesn't make any alignment guarantees, so the memory
        // has to be manually zeroed-out.
        let ptr = self.alloc(layout);
        if !ptr.is_null() {
            ptr::write_bytes(ptr, 0, layout.size());
        }
        ptr
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        libc::free(ptr as *mut c_void);
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        libc::realloc(ptr as *mut c_void, new_size) as *mut u8
    }
}
