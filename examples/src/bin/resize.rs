use anyhow::Context;
use graphicsmagick::{initialize, types::FilterTypes, wand::MagickWand};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    // This function should be invoked in the primary (original) thread
    // of the application's process, and before starting any OpenMP
    // threads, as part of program initialization.
    initialize();

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("meta")
        .join("GraphicsMagick-Logo.webp");
    let path = path.to_str().context("get image path failed")?;

    let mut mw = MagickWand::new();
    mw.read_image(path)?
        .resize_image(100, 100, FilterTypes::UndefinedFilter, 1.)?
        .write_image("/tmp/output.webp")?;

    Ok(())
}
