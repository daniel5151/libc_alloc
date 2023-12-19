//! A simple global allocator which hooks into `libc`.
//! Useful when linking `no_std` + `alloc` code into existing embedded C code.
//!
//! Uses `memalign` for allocations, and `free` for deallocations.
//! On Windows, uses `_aligned_*` functions.
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
use core::{cmp, ptr};

#[cfg(not(target_family = "windows"))]
mod libc;

#[cfg(target_family = "windows")]
mod win_crt;

/// Global Allocator which hooks into libc to allocate / free memory.
pub struct LibcAlloc;

#[cfg(feature = "global")]
#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

#[cfg(not(target_family = "windows"))]
unsafe impl GlobalAlloc for LibcAlloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        libc::memalign(
            layout.align().max(core::mem::size_of::<usize>()),
            layout.size(),
        ) as *mut u8
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // Unfortunately, calloc doesn't make any alignment guarantees, so the memory
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
    unsafe fn realloc(&self, old_ptr: *mut u8, old_layout: Layout, new_size: usize) -> *mut u8 {
        let new_layout = Layout::from_size_align_unchecked(new_size, old_layout.align());
        let new_ptr = self.alloc(new_layout);
        if !new_ptr.is_null() {
            let size = cmp::min(old_layout.size(), new_size);
            ptr::copy_nonoverlapping(old_ptr, new_ptr, size);
            self.dealloc(old_ptr, old_layout);
        }
        new_ptr
    }
}

#[cfg(target_family = "windows")]
unsafe impl GlobalAlloc for LibcAlloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        win_crt::_aligned_malloc(layout.size(), layout.align()) as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        win_crt::_aligned_free(ptr as *mut c_void)
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // Unfortunately, _aligned_calloc does not exist, so the memory
        // has to be manually zeroed-out.
        let ptr = self.alloc(layout);
        if !ptr.is_null() {
            ptr::write_bytes(ptr, 0, layout.size());
        }
        ptr
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        win_crt::_aligned_realloc(ptr as *mut c_void, new_size, layout.align()) as *mut u8
    }
}
