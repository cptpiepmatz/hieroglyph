// @generated

// __PLACEHOLDER_NOTO_SANS__

// __PLACEHOLDER_NOTO_SERIF__

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
