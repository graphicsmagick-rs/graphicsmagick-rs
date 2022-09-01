use graphicsmagick_sys::InitializeMagick;
use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_double, c_uint, c_void},
    ptr::null,
    string::FromUtf8Error,
    sync::Once,
    thread,
};

static HAS_INITIALIZED: Once = Once::new();

/// Wrapper of `graphicsmagick_sys::InitializeMagick`, call it before any `graphicsmagick` action.
/// Must be call in the main thread.
pub fn initialize() {
    HAS_INITIALIZED.call_once(|| {
        assert_eq!(
            thread::current().name(),
            Some("main"),
            "You have to call `graphicsmagick::initialize` in main thread"
        );

        unsafe {
            InitializeMagick(null());
        }
    });
}

/// Check if [`initialize`] has called.
#[inline]
pub fn has_initialized() -> bool {
    HAS_INITIALIZED.is_completed()
}

#[inline]
pub(crate) fn assert_initialized() {
    assert!(
        has_initialized(),
        "You have to call `graphicsmagick::initialize` first of all"
    )
}

/// For [`max_rgb`] to return max RGB value.
pub trait MaxRGB {
    fn max_rgb() -> Self;
}

impl MaxRGB for c_uint {
    fn max_rgb() -> Self {
        graphicsmagick_sys::MaxRGB
    }
}

impl MaxRGB for c_double {
    fn max_rgb() -> Self {
        graphicsmagick_sys::MaxRGBDouble
    }
}

/// Wrapper of `graphicsmagick_sys::MaxRGB` and `graphicsmagick_sys::MaxRGBDouble`.
pub fn max_rgb<T: MaxRGB>() -> T {
    <T>::max_rgb()
}

pub(crate) fn str_to_c_string(s: &str) -> CString {
    let buf = s.bytes().collect::<Vec<_>>();
    // from_vec_unchecked appends the trailing '\0'
    //
    // Safety:
    // s is a utf-8 str, so it cannot contains '\0'.
    unsafe { CString::from_vec_unchecked(buf) }
}

struct MagickAlloc(*const c_void);
impl Drop for MagickAlloc {
    fn drop(&mut self) {
        let c = self.0;

        if !c.is_null() {
            unsafe {
                graphicsmagick_sys::MagickFree(c as *mut c_void);
            }
        }
    }
}

pub(crate) fn c_str_to_string(c: *const c_char) -> Result<String, FromUtf8Error> {
    // Use MagickAlloc to ensure c is free on unwinding.
    let _magick_cstring = MagickAlloc(c as *const c_void);

    c_str_to_string_no_free(c)
}

pub(crate) fn c_str_to_string_no_free(c: *const c_char) -> Result<String, FromUtf8Error> {
    if c.is_null() {
        Ok("".to_string())
    } else {
        let cstr = unsafe { CStr::from_ptr(c) };
        let bytes = cstr.to_bytes().to_vec();

        String::from_utf8(bytes)
    }
}

pub(crate) fn c_arr_to_vec<T, U, F>(a: *const T, len: usize, f: F) -> Option<Vec<U>>
where
    F: Fn(*const T) -> U,
{
    if a.is_null() {
        return None;
    }

    // Use MagickAlloc to ensure a is free on unwinding.
    let _magick_vec = MagickAlloc(a as *const c_void);

    let mut v = Vec::with_capacity(len);
    for i in 0..len {
        let p = unsafe { a.add(i) };
        v.push(f(p));
    }

    Some(v)
}
