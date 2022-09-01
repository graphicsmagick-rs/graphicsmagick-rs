use graphicsmagick_sys::InitializeMagick;
use std::{
    borrow::Cow,
    ffi::{CStr, CString},
    os::raw::{c_char, c_double, c_uint, c_void},
    ptr::{null, NonNull},
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

#[derive(Debug)]
struct MagickAlloc(NonNull<c_void>);

impl MagickAlloc {
    fn new(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr() as *const c_void
    }
}
impl Drop for MagickAlloc {
    fn drop(&mut self) {
        unsafe {
            graphicsmagick_sys::MagickFree(self.0.as_ptr());
        }
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
    let _magick_vec = MagickAlloc::new(a as *mut c_void);

    let mut v = Vec::with_capacity(len);
    for i in 0..len {
        let p = unsafe { a.add(i) };
        v.push(f(p));
    }

    Some(v)
}

#[derive(Debug)]
pub struct MagickCString(Option<MagickAlloc>);

impl MagickCString {
    pub(crate) unsafe fn new(c: *const c_char) -> Self {
        Self(MagickAlloc::new(c as *mut c_void))
    }

    /// Convert [`MagickCString`] to [`CStr`].
    pub fn as_c_str(&self) -> &CStr {
        self.0
            .as_ref()
            .map(|alloc| unsafe { CStr::from_ptr(alloc.as_ptr() as *const c_char) })
            .unwrap_or_else(|| unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") })
    }

    /// Convert [`MagickCString`] to [`str`].
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        self.as_c_str().to_str()
    }

    /// Convert [`MagickCString`] to utf-8 string, including
    /// non utf-8 characters.
    ///
    /// If all characters are valid utf-8 character, then
    /// `Cow::Borrowed` is returned.
    ///
    /// Otherwise, `Cow::Owned` is returned where any invalid UTF-8 sequences
    /// are replaced with [`std::char::REPLACEMENT_CHARACTER`],
    /// which looks like this: "�"
    pub fn to_str_lossy(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(self.as_c_str().to_bytes())
    }
}

pub(crate) trait CStrExt {
    unsafe fn from_ptr_checked_on_debug<'a>(ptr: *const c_char) -> &'a CStr {
        debug_assert!(!ptr.is_null());

        CStr::from_ptr(ptr)
    }
}

impl CStrExt for CStr {}
