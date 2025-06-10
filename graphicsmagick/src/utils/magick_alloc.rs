use std::{
    borrow::Cow,
    ffi::CStr,
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut},
    os::raw::{c_char, c_void},
    ptr::NonNull,
    slice::{from_raw_parts, from_raw_parts_mut},
    str::Utf8Error,
};

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
pub(crate) struct MagickAutoRelinquish(NonNull<c_void>);

impl MagickAutoRelinquish {
    pub(crate) unsafe fn new(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    pub(crate) unsafe fn as_c_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.as_ptr() as *const c_char) }
    }
}
impl Drop for MagickAutoRelinquish {
    fn drop(&mut self) {
        unsafe {
            graphicsmagick_sys::MagickFree(self.0.as_ptr());
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
        self.0.0 as *const c_char
    }

    /// Convert [`MagickCString`] to [`CStr`].
    pub fn as_c_str(&self) -> &CStr {
        let ptr = self.as_ptr();
        if ptr.is_null() {
            c""
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
        unsafe {
            debug_assert!(!ptr.is_null());

            CStr::from_ptr(ptr)
        }
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
        unsafe {
            assert_eq!(mem::size_of::<U>(), mem::size_of::<T>());

            (!a.is_null()).then(|| Self {
                alloc: MagickAlloc::new(a as *mut c_void),
                len,
                phantom: PhantomData,
            })
        }
    }

    /// Return pointer to the underlying data
    pub fn as_ptr(&self) -> *const T {
        self.alloc.0 as *const T
    }

    /// Return pointer to the underlying data
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.alloc.0 as *mut T
    }
}

impl<T> Deref for MagickBoxSlice<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { from_raw_parts(self.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for MagickBoxSlice<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { from_raw_parts_mut(self.as_mut_ptr(), self.len) }
    }
}
