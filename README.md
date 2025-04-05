<h1 align="center">hieroglyph</h1>
<p align="center">
  <b>
    Render obscure scripts and emojis locally with zero dependencies.
  </b>
</p>

<br>

<div align="center">

  [![Crates.io](https://img.shields.io/crates/v/hieroglyph?style=for-the-badge)](https://crates.io/crates/hieroglyph)
  [![Docs.rs](https://img.shields.io/docsrs/hieroglyph?style=for-the-badge)](https://docs.rs/hieroglyph)
  [![License](https://img.shields.io/crates/l/hieroglyph?style=for-the-badge)](LICENSE)

</div>


`hieroglyph` is a text shaping and layout engine for rendering Unicode and emoji 
content without any system font dependencies. 
It uses statically embedded [Noto](https://github.com/notofonts) and 
[Twemoji](https://github.com/cptpiepmatz/twemoji-assets) fonts to produce 
pixel-precise output directly into RGBA buffers, suitable for GUIs, image 
generation, or text rendering in headless environments.

## Installation

```toml
[dependencies]
hieroglyph = "0.1"
```

## Usage

```rust
use hieroglyph::{DrawingContext, Segments};

let ctx = DrawingContext::new();
let segments = Segments::new("Hello 🌍 𓂀");

ctx.draw(&segments, |(x, y), color| {
    your_pixel_buffer.set_pixel(x, y, color.to_rgba());
});
```

## Fonts

All required fonts are bundled into the binary:

- Complete [Noto Sans/Serif](https://www.google.com/get/noto/) set for Unicode text
- Complete [Twemoji](https://github.com/jdecked/twemoji) set for emoji rendering

At runtime, the font system maps each character to the correct asset in memory, 
with no external dependencies. 
Additional fonts can be manually registered if needed.

## License

This project is licensed under the [MIT License](LICENSE).
