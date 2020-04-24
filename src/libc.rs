//! A tiny subset of the `libc` crate.

#![allow(non_camel_case_types)]

use core::ffi::c_void;

// While `c_int` doesn't _have_ to be i32, every platform supported by the
// `libc` crate defines `type c_int = i32`.
pub type c_int = i32;

// Similarly, while `size_t` doesn't _have_ to be usize, every platform
// supported by the `libc` crate defines `type size_t = usize`.
pub type size_t = usize;

extern "C" {
    pub fn free(p: *mut c_void);
    pub fn posix_memalign(memptr: *mut *mut c_void, align: size_t, size: size_t) -> c_int;
    pub fn realloc(p: *mut c_void, size: size_t) -> *mut c_void;
}
