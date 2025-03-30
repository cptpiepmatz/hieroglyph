pub mod fonts {
    //! This module contains all the fonts included in `hieroglyph`.
    //!
    //! It provides static arrays of unhinted OTF files for Google Noto fonts, divided into three 
    //! categories:
    //! - **Sans serif fonts** ([`NOTO_SANS`])
    //! - **Serif fonts** ([`NOTO_SERIF`])
    //! - **Other fonts** ([`NOTO_REST`])
    //!
    //! The fonts are pulled from the
    //! [Noto Fonts GitHub repository](https://github.com/notofonts/notofonts.github.io.git)
    //! and are embedded directly in the binary using [`include_bytes!`]. 
    //! The arrays' lengths and the order of files are determined by the available files and should 
    //! be treated as an implementation detail.
    //! 
    //! This module is generated at build time and is found in the `OUT_DIR`.
    include!(concat!(env!("OUT_DIR"), "/fonts.rs"));
}

mod draw;
mod segments;

pub use draw::*;
pub use segments::*;
