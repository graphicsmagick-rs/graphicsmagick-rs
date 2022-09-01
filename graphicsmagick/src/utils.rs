use graphicsmagick_sys::InitializeMagick;
use std::{
    borrow::Cow,
    ffi::{CStr, CString},
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut},
    os::raw::{c_char, c_double, c_uint, c_void},
    ptr::null,
    slice::{from_raw_parts, from_raw_parts_mut},
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
#[repr(transparent)]
struct MagickAlloc(*mut c_void);

impl MagickAlloc {
    unsafe fn new(ptr: *mut c_void) -> Self {
        Self(ptr)
    }
}
impl Drop for MagickAlloc {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                graphicsmagick_sys::MagickFree(self.0);
            }
        }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct MagickCString(MagickAlloc);

impl MagickCString {
    pub(crate) unsafe fn new(c: *const c_char) -> Self {
        Self(MagickAlloc(c as *mut c_void))
    }

    /// Return pointer to the underlying data.
    pub fn as_ptr(&self) -> *const c_char {
        self.0 .0 as *const c_char
    }

    /// Convert [`MagickCString`] to [`CStr`].
    pub fn as_c_str(&self) -> &CStr {
        let ptr = self.as_ptr();
        if ptr.is_null() {
            unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") }
        } else {
            unsafe { CStr::from_ptr(ptr) }
        }
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

#[derive(Debug)]
pub struct MagickBoxSlice<T> {
    alloc: MagickAlloc,
    len: usize,
    phantom: PhantomData<T>,
}

impl<T> MagickBoxSlice<T> {
    /// T must be either the same type as U, or be transparent newtype
    /// of U.
    /// T must be able to deal with all valid representation of U.
    pub(crate) unsafe fn new<U>(a: *mut U, len: usize) -> Option<Self> {
        assert_eq!(mem::size_of::<U>(), mem::size_of::<T>());

        (!a.is_null()).then(|| Self {
            alloc: MagickAlloc::new(a as *mut c_void),
            len,
            phantom: PhantomData,
        })
    }
}

impl<T> Deref for MagickBoxSlice<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { from_raw_parts(self.alloc.0 as *const T, self.len) }
    }
}

impl<T> DerefMut for MagickBoxSlice<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { from_raw_parts_mut(self.alloc.0 as *mut T, self.len) }
    }
}
