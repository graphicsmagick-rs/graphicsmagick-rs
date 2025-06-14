#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(rust_2018_idioms)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]
#![allow(clippy::too_many_arguments)]
#![doc = include_str!("../README.md")]

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
