# graphicsmagick-rs

[![Rustc Version](https://img.shields.io/badge/rustc-1.42+-lightgray.svg)](https://blog.rust-lang.org/2020/03/12/Rust-1.42.html)
[![Actions](https://github.com/jmjoy/graphicsmagick-rs/workflows/Rust/badge.svg)](https://github.com/jmjoy/graphicsmagick-rs/actions)
[![Crate](https://img.shields.io/crates/v/graphicsmagick.svg)](https://crates.io/crates/graphicsmagick)
[![API](https://docs.rs/graphicsmagick/badge.svg)](https://docs.rs/graphicsmagick)

<img src="/meta/GraphicsMagick-Logo.webp" alt="GraphicsMagick-Logo" align="right" />

GraphicsMagick binding for Rust.

**Under development.**

## Support

Support and tested GraphicsMagick version: `1.3.20 ~ 1.3.35`.

## Requirement

Require `graphicsmagick`, `libgraphicsmagick`, `clang` and `libclang`.

In Deepin/Ubuntu/Debian, you can install these by:

```bash
sudo apt install graphicsmagick libgraphicsmagick1-dev
sudo apt install llvm-dev libclang-dev clang
```

Before build, please check the `GraphicsMagickWand-config` is executable,
or specify the environment variable `GRAPHICS_MAGICK_WAND_CONFIG` correctly.

## License

[MIT](/LICENSE).
