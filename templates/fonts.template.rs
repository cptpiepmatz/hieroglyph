// @generated

/// `NOTO_SANS` is a static array of unhinted OTF files for sans serif fonts (Google Noto fonts).
///
/// These fonts are pulled from the
/// [Noto Fonts GitHub repository](https://github.com/notofonts/notofonts.github.io.git)
/// and embedded directly in the binary using [`include_bytes!`].
/// The array's length and file order depend on the number of included files and are an 
/// implementation detail; do not rely on the exact length or order.
// __PLACEHOLDER_NOTO_SANS__

/// `NOTO_SERIF` is a static array of unhinted OTF files for serif fonts (Google Noto fonts).
///
/// These fonts are pulled from the
/// [Noto Fonts GitHub repository](https://github.com/notofonts/notofonts.github.io.git)
/// and embedded directly in the binary using [`include_bytes!`].
/// The array's size and file order reflect the included files and are considered an 
/// implementation detail; do not depend on the exact length or order.
// __PLACEHOLDER_NOTO_SERIF__

/// `NOTO_REST` is a static array of unhinted OTF files for fonts that are neither sans serif nor 
/// serif (Google Noto fonts).
///
/// These fonts are pulled from the
/// [Noto Fonts GitHub repository](https://github.com/notofonts/notofonts.github.io.git)
/// and embedded directly in the binary using [`include_bytes!`].
/// The array's length and the order of its elements depend on the available files and are an 
/// implementation detail; do not rely on the exact length or order.
// __PLACEHOLDER_NOTO_REST__

#[cfg(test)]
#[test]
#[rustfmt::skip]
fn fonts_are_valid() {
    let parse = cosmic_text::ttf_parser::Face::parse;

    NOTO_SANS.iter().map(|this| parse(this, 0)).collect::<Result<Vec<_>, _>>().unwrap();
    NOTO_SERIF.iter().map(|this| parse(this, 0)).collect::<Result<Vec<_>, _>>().unwrap();
    NOTO_REST.iter().map(|this| parse(this, 0)).collect::<Result<Vec<_>, _>>().unwrap();
}
