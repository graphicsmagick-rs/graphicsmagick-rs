pub(crate) fn logo_path() -> String {
    let mut path = std::env::var("PWD").unwrap();
    path.push_str("/meta/GraphicsMagick-Logo.webp");
    path
}

pub(crate) fn logo_unicode_path() -> String {
    let mut path = std::env::var("PWD").unwrap();
    path.push_str("/meta/GraphicsMagick-图标.webp");
    path
}
