//! A tiny subset of the `libc` crate.

#![allow(non_camel_case_types)]

use core::ffi::c_void;

// While `size_t` doesn't _have_ to be usize, every platform supported
// by the `libc` crate defines `type size_t = usize`.
pub type size_t = usize;

extern "C" {
    pub fn free(p: *mut c_void);

    #[cfg(not(target_os = "macos"))]
    pub fn memalign(align: size_t, size: size_t) -> *mut c_void;

    #[cfg(target_os = "macos")]
    pub fn posix_memalign(ptr: *mut *mut c_void, align: size_t, size: size_t) -> core::ffi::c_int;
}
