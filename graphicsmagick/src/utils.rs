use graphicsmagick_sys::InitializeMagick;
use once_cell::sync::OnceCell;
use std::{
    ffi::CString,
    ptr::null,
    sync::{
        atomic::{AtomicBool, Ordering},
        Once,
    },
    thread,
};

static HAS_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn initialize() {
    static START: Once = Once::new();

    START.call_once(|| {
        assert_eq!(
            thread::current().name(),
            Some("main"),
            "You have to call `graphicsmagick::initialize` in main thread"
        );

        unsafe {
            InitializeMagick(null());
        }

        HAS_INITIALIZED.store(true, Ordering::SeqCst)
    });
}

#[inline]
pub fn has_initialized() -> bool {
    HAS_INITIALIZED.load(Ordering::SeqCst)
}

#[inline]
pub(crate) fn assert_initialized() {
    assert!(
        has_initialized(),
        "You have to call `graphicsmagick::initialize` first of all"
    )
}

pub trait MaxRGB {
    fn max_rgb() -> Self;
}

impl MaxRGB for u32 {
    fn max_rgb() -> Self {
        graphicsmagick_sys::MaxRGB
    }
}

impl MaxRGB for f64 {
    fn max_rgb() -> Self {
        graphicsmagick_sys::MaxRGBDouble
    }
}

pub fn max_rgb<T: MaxRGB>() -> T {
    <T>::max_rgb()
}

pub(crate) fn str_to_c_string(s: &str) -> CString {
    let buf = s.bytes().chain(0..1).collect::<Vec<_>>();
    unsafe { CString::from_vec_unchecked(buf) }
}
