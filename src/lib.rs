use std::cmp;
use std::collections::HashMap;
use std::fmt::Alignment;
use std::ops::Deref;

use ab_glyph::{Font, GlyphId, ScaleFont};
use fonts::{NOTO_REST, NOTO_SANS, NOTO_SERIF};
use image::{GenericImage, GenericImageView, Pixel, Primitive, Rgba};
use twemoji_assets::svg::SvgTwemojiAsset;
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

type FontRef = &'static ab_glyph::FontRef<'static>;

#[rustfmt::skip]
pub mod fonts;

#[derive(Debug, Clone)]
pub enum Grapheme<'s> {
    Text(TextGrapheme<'s>),
    Emoji(EmojiGrapheme),
}

#[derive(Debug, Clone)]
pub struct TextGrapheme<'s> {
    text: &'s str,
}

impl<'s> TextGrapheme<'s> {
    fn new(s: &'s str) -> Self {
        Self { text: s }
    }
}

#[derive(Debug, Clone)]
pub struct EmojiGrapheme {
    twemoji_asset: &'static SvgTwemojiAsset,
}

impl<'s> Grapheme<'s> {
    fn from_emoji(s: &'s str) -> Self {
        match SvgTwemojiAsset::from_emoji(s) {
            Some(twemoji_asset) => Grapheme::Emoji(EmojiGrapheme { twemoji_asset }),
            None => Grapheme::Text(TextGrapheme { text: s }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Graphemes<'s> {
    inner: unicode_segmentation::Graphemes<'s>,
}

impl<'s> Iterator for Graphemes<'s> {
    type Item = Grapheme<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next()?;
        Some(Grapheme::from_emoji(next))
    }
}

impl<'s> DoubleEndedIterator for Graphemes<'s> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let next = self.inner.next_back()?;
        Some(Grapheme::from_emoji(next))
    }
}

impl<'s> Graphemes<'s> {
    pub fn new(s: &'s str) -> Self {
        Self {
            inner: s.graphemes(true),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FontOrder {
    SansFirst,
    SerifFirst,
}

pub struct GraphemesRenderer {
    pub color: Rgba<u8>,
    pub alignment: Alignment,
    pub scale: f32,
    pub baseline: (i32, i32),

    font_order: FontOrder,
    font_cache: HashMap<char, FontRef>,
}

impl GraphemesRenderer {
    pub fn new(font_order: FontOrder) -> Self {
        Self {
            color: Rgba([0, 0, 0, 255]),
            alignment: Alignment::Left,
            scale: 20.0,
            baseline: (0, 20),

            font_order,
            font_cache: HashMap::new(),
        }
    }

    fn find_font(&mut self, c: char) -> (FontRef, GlyphId) {
        if let Some(font_ref) = self.font_cache.get(&c) {
            let glyph_id = font_ref.glyph_id(c);
            return (font_ref, glyph_id);
        }

        let mut font_iter = match self.font_order {
            FontOrder::SansFirst => NOTO_SANS
                .iter()
                .chain(NOTO_SERIF.iter())
                .chain(NOTO_REST.iter()),
            FontOrder::SerifFirst => NOTO_SERIF
                .iter()
                .chain(NOTO_SANS.iter())
                .chain(NOTO_REST.iter()),
        };

        font_iter
            .find_map(|&font| {
                let glyph_id = font.glyph_id(c);
                match glyph_id.0 {
                    0 => None,
                    _ => Some((font.deref(), glyph_id)),
                }
            })
            .unwrap_or((NOTO_SANS[0].deref(), GlyphId(0)))
    }

    pub fn bounding_box(&mut self, graphemes: Graphemes) -> (f32, f32) {
        // TODO: figure out if kerning and ligatures should be considered here

        let (cap_font, _) = self.find_font('H');
        let cap_font = cap_font.as_scaled(self.scale);
        let emoji_edge = cap_font.ascent().ceil();

        let mut width = 0.0;
        let height = cap_font.height().ceil();

        for grapheme in graphemes {
            match grapheme {
                Grapheme::Emoji(_) => width += emoji_edge,
                Grapheme::Text(text_grapheme) => {
                    for c in text_grapheme.text.chars() {
                        let (font, glyph_id) = self.find_font(c);
                        let font = font.as_scaled(self.scale);
                        width += font.h_advance(glyph_id);
                    }
                }
            }
        }

        (width, height)
    }

    pub fn draw<I>(&mut self, graphemes: Graphemes, img: &mut I)
    where
        I: GenericImage<Pixel = Rgba<u8>>,
    {
        let y = self.baseline.1;
        let mut x = match self.alignment {
            Alignment::Left => self.baseline.0,
            Alignment::Right => {
                let width = self.bounding_box(graphemes.clone()).0;
                self.baseline.0 - width.ceil() as i32
            }
            Alignment::Center => {
                let width = self.bounding_box(graphemes.clone()).0;
                self.baseline.0 - (width / 2.0).ceil() as i32
            }
        };

        for grapheme in graphemes {
            match grapheme {
                Grapheme::Emoji(emoji_grapheme) => {
                    dbg!(emoji_grapheme);
                    todo!()
                },
                Grapheme::Text(text_grapheme) => {
                    self.draw_text_grapheme(text_grapheme, img, &mut x, y)
                }
            }
        }
    }

    fn draw_text_grapheme<I>(&mut self, grapheme: TextGrapheme, img: &mut I, x: &mut i32, y: i32)
    where
        I: GenericImage<Pixel = Rgba<u8>>,
    {
        let mut base_advance = None;
        for c in grapheme.text.chars().nfc() {
            let (font, glyph_id) = self.find_font(c);
            let font = font.as_scaled(self.scale);
            let mut glyph = font.scaled_glyph(c);
            glyph.position.x = base_advance.unwrap_or(0.0) / 2.0;
            if let Some(glyph) = font.outline_glyph(glyph) {
                let px_bounds = glyph.px_bounds();
                glyph.draw(|xg, yg, coverage| {
                    let xi = (*x as f32 + xg as f32 + px_bounds.min.x) as u32;
                    let yi = (y as f32 + yg as f32 + px_bounds.min.y) as u32;
                    let mut pixel = img.get_pixel(xi, yi);
                    let blend = self
                        .color
                        .map_with_alpha(|c| c, |a| (a as f32 * coverage) as u8);
                    pixel.blend(&blend);
                    img.put_pixel(xi, yi, pixel);
                });
            }

            if base_advance.is_none() { 
                base_advance = Some(font.h_advance(glyph_id)); 
            }
        }

        if let Some(base_advance) = base_advance {
            *x += base_advance as i32;
        }
    }
}
