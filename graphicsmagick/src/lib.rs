#[macro_use]
mod macros;
pub mod error;
pub mod types;
pub(crate) mod utils;
pub mod wand;

pub use crate::{
    error::{Error, Result},
    utils::{has_initialized, initialize, max_rgb},
};
