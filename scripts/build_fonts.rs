//! ```cargo
//! [dependencies]
//! ab_glyph = "0.2"
//! heck = "0.5"
//! ```

use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

use ab_glyph::{Font, FontVec};
use heck::ToShoutySnakeCase;

const TEMPLATE: &str = include_str!("../templates/fonts.template.rs");
const PLACEHOLDER_NOTO_SANS: &str = "// __PLACEHOLDER_NOTO_SANS__";
const PLACEHOLDER_NOTO_SERIF: &str = "// __PLACEHOLDER_NOTO_SERIF__";
const PLACEHOLDER_NOTO_REST: &str = "// __PLACEHOLDER_NOTO_REST__";

struct FontMeta {
    name: String,
    _identifier: String,
    _font: FontVec,
    _max_codepoint: u32,
    path: PathBuf,
}

impl FontMeta {
    fn from_incomplete_path(path: &Path) -> FontMeta {
        let name = path
            .components()
            .last()
            .unwrap()
            .as_os_str()
            .to_string_lossy()
            .into_owned();
        let identifier = name.to_shouty_snake_case() + "_FONT_REF";
        let path = path.join(format!("unhinted/otf/{name}-Regular.otf"));
        let font = fs::read(&path).unwrap();
        let font = FontVec::try_from_vec(font).unwrap();
        let max_codepoint = font
            .codepoint_ids()
            .map(|(_, c)| c as u32)
            .max()
            .unwrap_or_default();
        FontMeta {
            name,
            _identifier: identifier,
            _font: font,
            _max_codepoint: max_codepoint,
            path,
        }
    }

    fn make_macro(name: &str, list: &[FontMeta]) -> String {
        let mut out = String::new();
        write!(out, "pub static {}: [&[u8]; {}] = [", name, list.len()).unwrap();
        for item in list {
            write!(
                out,
                "\n    include_bytes!(\"../{}\"),",
                item.path.to_string_lossy().replace("\\", "/")
            )
            .unwrap();
        }
        write!(out, "\n];").unwrap();
        out
    }
}

fn main() {
    let mut font_metas: Vec<_> = fs::read_dir("fonts/Noto/fonts")
        .unwrap()
        .map(Result::unwrap)
        .filter(|entry| entry.metadata().unwrap().is_dir())
        .map(|entry| entry.path())
        .map(|path| FontMeta::from_incomplete_path(&path))
        .collect();
    font_metas.sort_by_key(|FontMeta { name, .. }| name.len());

    let (mut noto_sans, mut noto_serif, mut noto_rest) = (vec![], vec![], vec![]);
    for font_meta in font_metas {
        let name = font_meta.name.as_str();
        match name {
            _ if name.contains("Test") => (),
            _ if name.contains("NotoSans") => noto_sans.push(font_meta),
            _ if name.contains("NotoSerif") => noto_serif.push(font_meta),
            _ => noto_rest.push(font_meta),
        }
    }

    #[rustfmt::skip]
    let file = TEMPLATE
        .replace(PLACEHOLDER_NOTO_SANS, &FontMeta::make_macro("NOTO_SANS", &noto_sans))
        .replace(PLACEHOLDER_NOTO_SERIF, &FontMeta::make_macro("NOTO_SERIF", &noto_serif))
        .replace( PLACEHOLDER_NOTO_REST, &FontMeta::make_macro("NOTO_REST", &noto_rest));

    fs::write("src/fonts.rs", file).unwrap();
}
