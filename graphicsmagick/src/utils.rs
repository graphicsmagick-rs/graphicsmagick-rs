use graphicsmagick_sys::InitializeMagick;
use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_double, c_uint, c_void},
    ptr::null,
    str::Utf8Error,
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

struct MagickCString(*const c_char);
impl Drop for MagickCString {
    fn drop(&mut self) {
        let c = self.0;

        if !c.is_null() {
            unsafe {
                graphicsmagick_sys::MagickFree(c as *mut c_void);
            }
        }
    }
}

pub(crate) fn c_str_to_string(c: *const c_char) -> Result<String, Utf8Error> {
    // Use MagickCString to ensure c is free on unwinding.
    let _magick_cstring = MagickCString(c);

    c_str_to_string_no_free(c)
}

pub(crate) fn c_str_to_string_no_free(c: *const c_char) -> Result<String, Utf8Error> {
    if c.is_null() {
        return Ok("".to_string());
    }
    let s = unsafe { CStr::from_ptr(c) }.to_str()?.to_string();
    Ok(s)
}

pub(crate) fn c_arr_to_vec<T, U, F>(a: *const T, len: usize, f: F) -> Option<Vec<U>>
where
    F: Fn(*const T) -> U,
{
    if a.is_null() {
        return None;
    }
    let mut v = Vec::with_capacity(len);
    for i in 0..len {
        let p = unsafe { a.add(i) };
        v.push(f(p));
    }
    unsafe {
        graphicsmagick_sys::MagickFree(a as *mut c_void);
    }
    Some(v)
}
