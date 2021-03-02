#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(rust_2018_idioms)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]
#![allow(clippy::too_many_arguments)]
#![allow(unused_doc_comments)]
//! <img src="https://github.com/jmjoy/graphicsmagick-rs/blob/master/meta/GraphicsMagick-Logo.webp?raw=true" alt="GraphicsMagick-Logo" align="right" />
//!
//! GraphicsMagick binding for Rust.
//!
//! **Under development.**
//!
//! ## Requirement
//!
//! Require `graphicsmagick`, `libgraphicsmagick`, `clang` and `libclang`.
//!
//! In Deepin/Ubuntu/Debian, you can install these by:
//!
//! ```bash
//! sudo apt install graphicsmagick libgraphicsmagick1-dev
//! sudo apt install llvm-dev libclang-dev clang
//! ```
//!
//! Before build, please check the `GraphicsMagickWand-config` is executable,
//! or specify the environment variable `GRAPHICS_MAGICK_WAND_CONFIG` correctly.
//!
//! ## Support
//!
//! 1. Support and tested GraphicsMagick version: `1.3.20 ~ 1.3.35`.
//!
//! 1. There are some version flag in the documentation, like `gm_v_1_3_26`, meaning that your `GraphicsMagick` version must
//!    be greater than or equal to `1.3.26` to used this method.
//!
//!    ![version-demo](https://github.com/jmjoy/graphicsmagick-rs/blob/master/meta/version-demo.webp?raw=true)
//!    
//! 1. `GraphicsMagick` supports OpenMP if you are compiling with OpenMP-enabled `cc`, you can set the environment variable
//!    `OMP_NUM_THREADS` to limit the number of threads or set `OMP_DISPLAY_ENV=TRUE` to display the OpenMP info when
//!    running the application.
//!
//!    Read <http://www.graphicsmagick.org/OpenMP.html> for details.
//!
//! ## Example
//!
//! Simple resize example:
//!
//! ```rust
//! use anyhow::Context;
//! use graphicsmagick::{initialize, types::FilterTypes, wand::MagickWand};
//! use std::path::PathBuf;
//!
//! fn main() -> anyhow::Result<()> {
//!     // This function should be invoked in the primary (original) thread
//!     // of the application's process, and before starting any OpenMP
//!     // threads, as part of program initialization.
//!     initialize();
//!
//!     let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//!         .join("..")
//!         .join("meta")
//!         .join("GraphicsMagick-Logo.webp");
//!     let path = path.to_str().context("get image path failed")?;
//!
//!     let mut mw = MagickWand::new();
//!     mw.read_image(path)?
//!         .resize_image(100, 100, FilterTypes::UndefinedFilter, 1.)?
//!         .write_image("/tmp/output.webp")?;
//!
//!     Ok(())
//! }
//! ```

#[macro_use]
mod macros;
pub mod error;
#[cfg(test)]
pub(crate) mod tests;
pub mod types;
pub(crate) mod utils;
pub mod wand;

#[cfg(doctest)]
doc_comment!(include_str!("../README.md"));

pub use crate::{
    error::{Error, Result},
    utils::{has_initialized, initialize, max_rgb, MaxRGB},
};
