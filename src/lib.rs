//! A font-embedded text renderer for obscure characters, scripts, and emojis, built for maximum 
//! portability.
//!
//! This crate enables rendering of virtually any Unicode character, including rare scripts and 
//! emojis, without relying on system fonts or external resources. 
//! It's powered by [`cosmic-text`](cosmic_text) for shaping and layout, with bundled [`Noto`] fonts 
//! and emoji assets via [`twemoji-assets`](twemoji_assets).
//!
//! All assets are included in the binary, which increases its size (~35MB in release builds), but
//! ensures the renderer works in any environment, even minimal ones like Docker scratch images or 
//! embedded systems without font support.
//!
//! # Rendering
//! Rendering is decoupled from any specific imaging or graphics backend. 
//! Instead, [`DrawingContext`] offers a [`draw`](DrawingContext::draw) method that invokes a 
//! user-provided callback for each pixel, giving its `(x, y)` coordinate and color. 
//!
//! # Usage
//! A minimal setup requires:
//! - A [`DrawingContext`] (can be reused across frames or content with the same settings),
//! - A [`Segments`] object, which holds the segmented input text to render.
//!
//! # Example
//! The `examples/example.rs` showcasing how the [`image`] crate may be used to render very obscure 
//! characters onto a canvas.
//! ```rust
#![doc = include_str!("../examples/example.rs")]
//! ```
//!
//! # Note
//! Line breaks (`\n`) are currently not supported.
//!
//! [`Noto`]: https://www.google.com/get/noto/
//! [`image`]: https://crates.io/crates/image

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
