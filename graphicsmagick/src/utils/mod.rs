mod init;
pub(crate) use init::assert_initialized;
pub use init::{has_initialized, initialize};

mod rgb;
pub use rgb::{max_rgb, MaxRGB};

mod magick_alloc;
pub(crate) use magick_alloc::{CStrExt, MagickAutoRelinquish};
pub use magick_alloc::{MagickBoxSlice, MagickCString};
