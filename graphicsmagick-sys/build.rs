#![allow(clippy::unreadable_literal)]

use anyhow::{Context, anyhow, bail};
use std::{
    env,
    path::PathBuf,
    process::{Command, Output},
};

#[derive(Debug)]
struct GraphicsMagickConfig {
    include_flags: Vec<String>,
    searches: Vec<String>,
    libs: Vec<String>,
}

fn new_graphicsmagick_config() -> anyhow::Result<GraphicsMagickConfig> {
    let gmw_config_env = "GRAPHICS_MAGICK_WAND_CONFIG";
    println!("cargo:rerun-if-env-changed={}", gmw_config_env);

    let cmd_path =
        env::var(gmw_config_env).unwrap_or_else(|_| "GraphicsMagickWand-config".to_string());
    let mut cmd = Command::new(&cmd_path);
    let output = cmd
        .args(["--cppflags", "--ldflags", "--libs"])
        .output()
        .context(format!(
            "Run command `{}` failed, please check the `GraphicsMagickWand-config` \
                is executable, or specify the environment variable `{}` correctly.",
            &cmd_path, gmw_config_env
        ))?;
    if !output.status.success() {
        bail!("failed to run command `{}`", &cmd_path);
    }

    let mut gmc = GraphicsMagickConfig {
        include_flags: Vec::new(),
        searches: Vec::new(),
        libs: Vec::new(),
    };

    let content = String::from_utf8(output.stdout)?;

    // split_ascii_whitespace would split '\n', '\t' and whitespace.
    // It will also filter out any empty str in the result.
    let mut it = content.split_ascii_whitespace();
    while let Some(token) = it.next() {
        let (flag, value) = if ["-I", "-L", "-l"].contains(&token) {
            // Since token only contains the flag, the next element of it
            // must contains the value.
            (token, it.next().unwrap())
        } else {
            (&token[..2], &token[2..])
        };

        match flag {
            "-I" => gmc.include_flags.push(format!("-I{}", value)),
            "-L" => gmc.searches.push(value.to_string()),
            "-l" => gmc.libs.push(value.to_string()),
            _ => (),
        }
    }

    Ok(gmc)
}

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=wrapper.h");

    let gmc = new_graphicsmagick_config()?;

    for flag in gmc.searches {
        println!("cargo:rustc-link-search={}", flag);
    }

    let link_kind = if cfg!(feature = "static") {
        "static"
    } else {
        "dylib"
    };

    for lib in gmc.libs {
        println!("cargo:rustc-link-lib={}={}", link_kind, lib);
    }

    let rustc = env::var_os("RUSTC").ok_or_else(|| anyhow!("env RUSTC not present"))?;

    let Output { stdout, status, .. } = Command::new(rustc)
        .arg("-V")
        .output()
        .map_err(|e| anyhow!("Fail to run 'rustc -V': {}", e))?;

    if !status.success() {
        bail!("'rustc -V' returns non-zero status");
    }

    let stdout = String::from_utf8_lossy(&stdout);

    let mut iter = stdout.split_whitespace();

    let rustc_parse_error = || anyhow!("Expected format 'rustc x.y.z (abcabcabc yyyy-mm-dd)'");
    iter.next().ok_or_else(rustc_parse_error)?;
    let rustc_version = iter
        .next()
        .ok_or_else(rustc_parse_error)?
        .trim_end_matches("-nightly");

    let rust_target: bindgen::RustTarget = rustc_version
        .parse()
        .map_err(|e| anyhow!("rustc version from 'rustc -V' is not a valid semver: {}", e))?;

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(&gmc.include_flags)
        .size_t_is_usize(false)
        // blacklist for not FFI-safe type u128.
        .blocklist_type("_Float64x")
        .blocklist_function("strtold")
        .blocklist_function("qecvt")
        .blocklist_function("qfcvt")
        .blocklist_function("qgcvt")
        .blocklist_function("qecvt_r")
        .blocklist_function("qfcvt_r")
        .rust_target(rust_target)
        .generate()
        .map_err(|e| anyhow!("Unable to generate bindings: {}", e))?;

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let binding_path = out_path.join("bindings.rs");
    bindings
        .write_to_file(binding_path)
        .context("Couldn't write bindings!")?;

    Ok(())
}
