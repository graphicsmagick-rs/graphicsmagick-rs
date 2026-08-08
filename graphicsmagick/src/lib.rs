#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(rust_2018_idioms)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]
#![allow(clippy::too_many_arguments)]
#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/113815004?s=200&v=4")]

pub mod error;
#[cfg(test)]
pub(crate) mod tests;
pub mod types;
pub(crate) mod utils;
pub mod wand;

pub use crate::{
    error::{Error, Result},
    utils::{MagickBoxSlice, MagickCString, MaxRGB, has_initialized, initialize, max_rgb},
};

pub use null_terminated_str;
