use std::fmt::Write;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::{env, fs, io};

use tar::Archive;
use zstd::stream::Decoder;

fn main() -> anyhow::Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    let fonts_dir = out_dir.join("fonts");
    let module_path = out_dir.join("fonts.rs");

    unpack_fonts("fonts.tar.zst", &fonts_dir)?;
    println!("cargo:rerun-if-changed=fonts.tar.zst");

    build_fonts_module(&fonts_dir, &module_path)?;
    println!("cargo:rerun-if-changed=templates/fonts.template.rs");

    Ok(())
}

fn unpack_fonts(archive_path: impl AsRef<Path>, output_dir: impl AsRef<Path>) -> io::Result<()> {
    let file = File::open(archive_path)?;
    let reader = BufReader::new(file);
    let decoder = Decoder::new(reader)?;
    let mut archive = Archive::new(decoder);
    archive.unpack(output_dir)?;
    Ok(())
}

fn build_fonts_module(
    fonts_dir: impl AsRef<Path>,
    module_path: impl AsRef<Path>,
) -> anyhow::Result<()> {
    const TEMPLATE: &str = include_str!("./templates/fonts.template.rs");
    const PLACEHOLDER_NOTO_SANS: &str = "// __PLACEHOLDER_NOTO_SANS__";
    const PLACEHOLDER_NOTO_SERIF: &str = "// __PLACEHOLDER_NOTO_SERIF__";
    const PLACEHOLDER_NOTO_REST: &str = "// __PLACEHOLDER_NOTO_REST__";

    let mut noto_sans = vec![];
    let mut noto_serif = vec![];
    let mut noto_rest = vec![];

    let fonts_dir = fs::read_dir(fonts_dir)?;
    for font_file in fonts_dir {
        let font_file = font_file?;
        let file_name = font_file.file_name().to_string_lossy().to_string();
        match file_name.as_str() {
            _ if file_name.contains("Test") => (),
            _ if file_name.contains("NotoSans") => noto_sans.push(file_name),
            _ if file_name.contains("NotoSerif") => noto_serif.push(file_name),
            _ => noto_rest.push(file_name),
        }
    }

    fn make_macro(name: &str, list: &[String]) -> anyhow::Result<String> {
        let mut out = String::new();
        write!(out, "pub static {}: [&[u8]; {}] = [", name, list.len())?;
        for item in list {
            write!(out, "\n    include_bytes!(\"./fonts/{}\"),", item)?;
        }
        write!(out, "\n];")?;
        Ok(out)
    }

    #[rustfmt::skip]
    let file = TEMPLATE
        .replace(PLACEHOLDER_NOTO_SANS, &make_macro("NOTO_SANS", &noto_sans)?)
        .replace(PLACEHOLDER_NOTO_SERIF, &make_macro("NOTO_SERIF", &noto_serif)?)
        .replace( PLACEHOLDER_NOTO_REST, &make_macro("NOTO_REST", &noto_rest)?);

    fs::write(module_path, file)?;
    Ok(())
}
