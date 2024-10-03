// @generated

use std::sync::LazyLock;

use ab_glyph::FontRef;

macro_rules! lazy_font {
    ($path:literal) => {
        LazyLock::new(|| {
            // this is an internal macro and will only use predefined fonts
            FontRef::try_from_slice(include_bytes!($path)).unwrap()
        })
    };
}

// __PLACEHOLDER_NOTO_SANS__

// __PLACEHOLDER_NOTO_SERIF__

// __PLACEHOLDER_NOTO_REST__

#[cfg(test)]
#[test]
#[rustfmt::skip]
fn fonts_are_valid() {
    let _ = NOTO_SANS.iter().map(|this| LazyLock::force(this)).collect::<Vec<_>>();
    let _ = NOTO_SERIF.iter().map(|this| LazyLock::force(this)).collect::<Vec<_>>();
    let _ = NOTO_REST.iter().map(|this| LazyLock::force(this)).collect::<Vec<_>>();
}
