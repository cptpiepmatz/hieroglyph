use std::cmp;
use std::{collections::HashMap, ops::Deref};
use std::fmt::Alignment;

use ab_glyph::{Font, GlyphId, ScaleFont};
use fonts::{NOTO_REST, NOTO_SANS, NOTO_SERIF};
use image::{GenericImage, GenericImageView, Pixel, Primitive};
use twemoji_assets::svg::SvgTwemojiAsset;
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
        Self {
            text: s,
        }
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

pub struct GraphemesRenderer<C: Pixel> {
    pub color: C,
    pub alignment: Alignment,
    pub scale: f32,
    pub baseline: (u32, u32),

    font_order: FontOrder,
    font_cache: HashMap<char, FontRef>,
}

impl<C: Pixel> GraphemesRenderer<C> {
    pub fn new(font_order: FontOrder) -> Self {
        Self {
            color: *Pixel::from_slice(&[
                // construct black generically
                C::Subpixel::DEFAULT_MIN_VALUE,
                C::Subpixel::DEFAULT_MIN_VALUE,
                C::Subpixel::DEFAULT_MIN_VALUE,
                C::Subpixel::DEFAULT_MAX_VALUE,
            ]),
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
            return (font_ref, glyph_id)
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

        font_iter.find_map(|&font| {
            let glyph_id = font.glyph_id(c);
            match glyph_id.0 {
                0 => None,
                _ => Some((font.deref(), glyph_id))
            }
        }).unwrap_or((NOTO_SANS[0].deref(), GlyphId(0)))
    }

    pub fn pixel_bb(&mut self, graphemes: Graphemes) -> (u32, u32) {
        // TODO: figure out if kerning and ligatures should be considered here

        let (cap_font, _) = self.find_font('H');
        let cap_font = cap_font.as_scaled(self.scale);
        let emoji_edge = cap_font.ascent().ceil() as u32;
        
        let mut width = 0;
        let height = cap_font.height().ceil() as u32;
        
        for grapheme in graphemes {
            match grapheme {
                Grapheme::Emoji(_) => width += emoji_edge,
                Grapheme::Text(text_grapheme) => {
                    for c in text_grapheme.text.chars() {
                        let (font, glyph_id) = self.find_font(c);
                        let font = font.as_scaled(self.scale);
                        width += font.h_advance(glyph_id).ceil() as u32;
                    }
                },
            }
        }

        (width, height)
    }

    pub fn draw<I>(&mut self, graphemes: Graphemes, img: I)
    where
        I: GenericImage<Pixel = C>
    {
        todo!()
    }
}
