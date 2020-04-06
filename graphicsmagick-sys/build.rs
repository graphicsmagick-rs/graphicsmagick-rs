use anyhow::{anyhow, bail, Context};
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    process::Command,
};

struct GraphicsMagickConfig {
    include_flags: Vec<String>,
    search_flags: Vec<String>,
    libs: Vec<String>,
}

fn new_graphicsmagick_config() -> anyhow::Result<GraphicsMagickConfig> {
    let cmd_path =
        env::var("GRAPHICS_MAGICK_WAND_CONFIG").unwrap_or("GraphicsMagickWand-config".to_string());
    let mut cmd = Command::new(&cmd_path);
    let output = cmd.args(&["--cppflags", "--ldflags", "--libs"]).output()?;
    if !output.status.success() {
        bail!("failed to run command `{}`", &cmd_path);
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
    let binding_path = out_path.join("bindings.rs");
    bindings
        .write_to_file(&binding_path)
        .context("Couldn't write bindings!")?;

    let lib_version_prefix = "pub const MagickLibVersion: u32 = ";
    let file = File::open(&binding_path)?;
    let reader = BufReader::new(file);

    let _lib_version = reader
        .lines()
        .find(|line| {
            line.as_ref()
                .ok()
                .map(|line| line.starts_with(lib_version_prefix))
                .unwrap_or_default()
        })
        .and_then(|line| {
            line.ok()
                .map(|line| {
                    line.chars()
                        .skip(lib_version_prefix.len())
                        .take_while(|c| *c != ';')
                        .collect::<String>()
                })
                .and_then(|version| version.parse::<u32>().ok())
        })
        .context("Unable to know lib version")?;

    Ok(())
}
