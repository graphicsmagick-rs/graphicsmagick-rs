mod init;
pub(crate) use init::assert_initialized;
pub use init::{has_initialized, initialize};

mod rgb;
pub use rgb::{MaxRGB, max_rgb};

mod magick_alloc;
pub(crate) use magick_alloc::{CStrExt, MagickAutoRelinquish};
pub use magick_alloc::{MagickBoxSlice, MagickCString};
