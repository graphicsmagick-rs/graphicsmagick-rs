#[macro_use]
mod macros;
pub mod error;
#[cfg(test)]
pub(crate) mod tests;
pub mod types;
pub(crate) mod utils;
pub mod wand;

pub use crate::{
    error::{Error, Result},
    utils::{has_initialized, initialize, max_rgb, MaxRGB},
};
