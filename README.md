# graphicsmagick-rs

[![Rustc Version](https://img.shields.io/badge/rustc-1.42+-lightgray.svg)](https://blog.rust-lang.org/2020/03/12/Rust-1.42.html)
[![Actions](https://github.com/jmjoy/graphicsmagick-rs/workflows/Rust/badge.svg)](https://github.com/jmjoy/graphicsmagick-rs/actions)
[![Crate](https://img.shields.io/crates/v/graphicsmagick.svg)](https://crates.io/crates/graphicsmagick)
[![API](https://docs.rs/graphicsmagick/badge.svg)](https://docs.rs/graphicsmagick)

<img src="https://github.com/jmjoy/graphicsmagick-rs/blob/master/meta/GraphicsMagick-Logo.webp?raw=true" alt="GraphicsMagick-Logo" align="right" />

[GraphicsMagick](http://www.graphicsmagick.org/index.html) binding for Rust.

## Requirement

Require `graphicsmagick`, `libgraphicsmagick`, `clang` and `libclang`.

In Deepin/Ubuntu/Debian, you can install these by:

```bash
sudo apt install graphicsmagick libgraphicsmagick1-dev
sudo apt install llvm-dev libclang-dev clang
```

Before build, please check the `GraphicsMagickWand-config` is executable,
or specify the environment variable `GRAPHICS_MAGICK_WAND_CONFIG` correctly.

## Support

1. Support and tested GraphicsMagick version: `>= 1.3.20`.

1. If you want to use higher version functions, you should specify features in `Cargo.toml` like:

   ```toml
   features = ["v1_3_36"]
   ```

1. `GraphicsMagick` supports OpenMP if you are compiling with OpenMP-enabled `cc`, you can set the environment variable
   `OMP_NUM_THREADS` to limit the number of threads or set `OMP_DISPLAY_ENV=TRUE` to display the OpenMP info when
   running the application.

   Read <http://www.graphicsmagick.org/OpenMP.html> for details.

## Example

Simple resize example:

```rust
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
```

## License

[MIT](https://github.com/jmjoy/graphicsmagick-rs/blob/master/LICENSE).
