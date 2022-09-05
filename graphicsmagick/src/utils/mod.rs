use std::ffi::CString;

mod init;
pub(crate) use init::assert_initialized;
pub use init::{has_initialized, initialize};

mod rgb;
pub use rgb::{max_rgb, MaxRGB};

mod magick_alloc;
pub(crate) use magick_alloc::{CStrExt, MagickAutoRelinquish};
pub use magick_alloc::{MagickBoxSlice, MagickCString};

pub(crate) fn str_to_c_string(s: &str) -> CString {
    let buf = s.bytes().collect::<Vec<_>>();
    // from_vec_unchecked appends the trailing '\0'
    //
    // Safety:
    // s is a utf-8 str, so it cannot contains '\0'.
    unsafe { CString::from_vec_unchecked(buf) }
}
