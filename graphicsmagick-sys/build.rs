use anyhow::{anyhow, bail, Context};
use std::{env, path::PathBuf, process::Command};

struct GraphicsMagickConfig {
    include_flags: Vec<String>,
    search_flags: Vec<String>,
    libs: Vec<String>,
}

fn new_graphicsmagick_config() -> anyhow::Result<GraphicsMagickConfig> {
    // TODO Take a environment variable
    let mut cmd = Command::new("GraphicsMagickWand-config");
    let output = cmd.args(&["--cppflags", "--ldflags", "--libs"]).output()?;
    if !output.status.success() {
        bail!("failed to run command `GraphicsMagick-config`");
    }

    let mut gmc = GraphicsMagickConfig {
        include_flags: Vec::new(),
        search_flags: Vec::new(),
        libs: Vec::new(),
    };

    let content = String::from_utf8(output.stdout)?;
    for line in content.lines() {
        if line.starts_with("-I") {
            gmc.include_flags.push(line.to_string());
        } else if line.starts_with("-L") {
            gmc.search_flags.push(line.to_string());
        } else if line.starts_with("-l") {
            gmc.libs
                .extend(line.split(' ').map(|item| String::from(&item[2..])));
        }
    }

    Ok(gmc)
}

fn main() -> anyhow::Result<()> {
    //    println!("cargo:rustc-link-lib=GraphicsMagick");
    println!("cargo:rerun-if-changed=wrapper.h");

    let gmc = new_graphicsmagick_config()?;

    for flag in gmc.search_flags {
        println!("cargo:rustc-flags={}", flag);
    }
    //    println!("cargo:rustc-flags=-L/usr/lib/x86_64-linux-gnu");

    for lib in gmc.libs {
        //        let kind = match &*lib {
        //            "lcms2" | "m" | "z" | "pthread" | "gomp" => "dylib",
        //            _ => "static",
        //        };
        println!("cargo:rustc-link-lib={}", lib);
    }

    //    let mut clang_args = Vec::new();
    //    let pkg = pkg_config::probe_library("GraphicsMagick")?;
    //    for include_path in pkg.include_paths {
    //        let include_path = include_path.to_str().context("invalid path")?;
    //        println!("cargo:include={}", include_path);
    //        clang_args.push(format!("-I{}", include_path));
    //    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(&gmc.include_flags)
        // blacklist for not FFI-safe type u128.
        .blacklist_type("_Float64x")
        .blacklist_function("strtold")
        .blacklist_function("qecvt")
        .blacklist_function("qfcvt")
        .blacklist_function("qgcvt")
        .blacklist_function("qecvt_r")
        .blacklist_function("qfcvt_r")
        .generate()
        .map_err(|_| anyhow!("Unable to generate bindings"))?;

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .context("Couldn't write bindings!")?;

    Ok(())
}
