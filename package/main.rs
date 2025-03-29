use std::fs::File;
use std::io::{BufWriter, Cursor};
use std::path::Path;
use std::time::Duration;

use anyhow::{Result, anyhow};
use glob::glob;
use indicatif::{ProgressBar, ProgressStyle};

fn main() -> anyhow::Result<()> {
    compress_fonts()?;
    Ok(())
}

const INPUT_PATTERN: &str = "fonts/Noto/fonts/*/unhinted/otf/*-Regular.otf";
const COMPRESSION_LEVEL: i32 = 22;

pub fn compress_fonts() -> Result<()> {
    let mut archive = tar::Builder::new(Vec::<u8>::new());

    for entry in glob(INPUT_PATTERN)? {
        let entry = entry?;
        let file_name = entry
            .file_name()
            .ok_or_else(|| anyhow!("file name not found for {}", entry.display()))?;
        let as_name = Path::new(file_name);
        archive.append_path_with_name(&entry, as_name)?;
    }

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template("{spinner} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_message("Compressing archive...");

    let archive = archive.into_inner()?;
    let mut archive = Cursor::new(archive);
    let output_file = File::create("fonts.tar.zst")?;
    let mut output_file = BufWriter::new(output_file);
    zstd::stream::copy_encode(&mut archive, &mut output_file, COMPRESSION_LEVEL)?;

    spinner.finish_and_clear();
    println!("Archive compressed");
    Ok(())
}
