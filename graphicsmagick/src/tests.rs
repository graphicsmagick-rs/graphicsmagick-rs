pub(crate) fn logo_path() -> String {
    let mut path = std::env::var("PWD").unwrap();
    path.extend("/meta/GraphicsMagick-Logo.webp".chars());
    path
}

pub(crate) fn logo_unicode_path() -> String {
    let mut path = std::env::var("PWD").unwrap();
    path.extend("/meta/GraphicsMagick-图标.webp".chars());
    path
}
