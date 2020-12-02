#![allow(non_camel_case_types)]

use core::ffi::c_void;

/// unsigned __int64 or unsigned integer, depending on the target platform
///
/// see: https://docs.microsoft.com/en-us/cpp/c-runtime-library/standard-types
pub type size_t = usize;

extern "C" {
    pub fn _aligned_malloc(size: size_t, align: size_t) -> *mut c_void;
    pub fn _aligned_realloc(p: *mut c_void, size: size_t, align: size_t) -> *mut c_void;
    pub fn _aligned_free(p: *mut c_void);
}
