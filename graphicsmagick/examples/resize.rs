use graphicsmagick::{initialize, types::FilterType, wand::MagickWand};
use graphicsmagick_sys::{
    CloneImageInfo, ExceptionInfo, GetExceptionInfo, Image, InitializeMagick, ReadImage, WriteImage,
};
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufReader, Read, Write},
    mem::MaybeUninit,
    os::raw::c_char,
    process::exit,
    ptr::{null, null_mut},
};

fn main() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let args = env::args().into_iter().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("Usage: COMMAND <INPUT> <OUTPUT>");
        return Ok(());
    }

    process(&args[1], &args[2])?;

    Ok(())
}

fn process(input: &str, output: &str) -> anyhow::Result<()> {
    initialize();

    let mut file = File::open(input)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let mut mw = MagickWand::new();
    let content = mw
        .read_image_blob(&buf)?
        .resize_image(500, 300, FilterType::UndefinedFilter, 1.)?
        .set_image_format("WEBP")?
        .write_image_blob()?;

    let mut file = File::create(output)?;
    file.write_all(&content)?;

    Ok(())
}
