use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;

use cosmic_text::fontdb::Database;
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};
use resvg::tiny_skia::{self, Pixmap, PixmapMut};
use resvg::usvg::{Options, Transform, Tree};

use crate::{EmojiSegment, Segment, Segments, TextSegment, fonts};

// use the emoji string as key
type TreeCache = HashMap<&'static str, Tree>;

#[derive(Debug)]
pub struct DrawingContext {
    font_system: FontSystem,
    swash_cache: SwashCache,
    tree_cache: TreeCache,
    font_size: f32,
    color: [u8; 4], // TODO: check if want a better color type
    line_height: f32,
    /// Cache capital info using `font_size` and `line_height` as keys.
    ///
    /// Since both `font_size` and `line_height` are `f32`, we cannot use them
    /// directly as keys. By converting them via [`f32::to_bits`], we can
    /// key properly. Normally this isn't a good idea but considering that
    /// users will only set the font size to specific values, this should be
    /// fine.
    capital_info: HashMap<(u32, u32), (u32, f32)>,
}

trait FontIterator: Iterator<Item = Vec<u8>> + Debug {}
impl<I> FontIterator for I where I: Iterator<Item = Vec<u8>> + Debug {}

#[derive(Debug)]
pub struct DrawingContextBuilder {
    font_order: FontOrder,
    pre_fonts: Option<Box<dyn FontIterator>>,
    locale: Cow<'static, str>,
}

impl Default for DrawingContextBuilder {
    fn default() -> Self {
        Self {
            font_order: Default::default(),
            pre_fonts: Default::default(),
            locale: "en".into(),
        }
    }
}

impl DrawingContextBuilder {
    pub fn font_order(mut self, font_order: FontOrder) -> Self {
        self.font_order = font_order;
        self
    }

    pub fn pre_fonts(
        self,
        pre_fonts: impl Iterator<Item = impl Into<Vec<u8>> + 'static> + 'static + Debug,
    ) -> DrawingContextBuilder {
        DrawingContextBuilder {
            pre_fonts: Some(Box::new(pre_fonts.map(Into::into))),
            font_order: self.font_order,
            locale: self.locale,
        }
    }

    pub fn locale(mut self, locale: impl Into<Cow<'static, str>>) -> Self {
        self.locale = locale.into();
        self
    }

    pub fn build(self) -> DrawingContext {
        DrawingContext::new_from_builder(self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum FontOrder {
    #[default]
    SansFirst,
    SerifFirst,
}

impl DrawingContext {
    pub fn new() -> Self {
        Self::configure().build()
    }

    pub fn configure() -> DrawingContextBuilder {
        DrawingContextBuilder::default()
    }

    pub fn new_from_builder(builder: DrawingContextBuilder) -> Self {
        use fonts::{NOTO_REST, NOTO_SANS, NOTO_SERIF};

        let mut font_db = Database::new();
        let default_fonts = match builder.font_order {
            FontOrder::SansFirst => NOTO_SANS.iter().chain(NOTO_SERIF.iter()),
            FontOrder::SerifFirst => NOTO_SERIF.iter().chain(NOTO_SANS.iter()),
        }
        .chain(NOTO_REST.iter())
        .map(|&bytes| Vec::from(bytes));
        match builder.pre_fonts {
            None => default_fonts.for_each(|bytes| font_db.load_font_data(bytes)),
            Some(pre_fonts) => pre_fonts
                .map(Into::into)
                .chain(default_fonts)
                .for_each(|bytes| font_db.load_font_data(bytes)),
        }
        let font_system = FontSystem::new_with_locale_and_db(builder.locale.to_string(), font_db);

        Self {
            font_size: 12.0,
            color: [0, 0, 0, 255],

            font_system,
            swash_cache: SwashCache::new(),
            tree_cache: TreeCache::new(),
            line_height: 50.0, // irrelevant for users right now as we do not handle newlines
            capital_info: HashMap::new(),
        }
    }

    pub fn font_size(&mut self, font_size: f32) {
        self.font_size = font_size;
        self.line_height = font_size;
    }

    pub fn rgba(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.color = [r, g, b, a];
    }

    pub fn rgb(&mut self, r: u8, g: u8, b: u8) {
        self.rgba(r, g, b, 255);
    }

    pub fn width(&mut self, segments: &Segments) -> u32 {
        let metrics = Metrics::new(self.font_size, self.line_height);
        let attrs = Attrs::new();
        let mut buffer = Buffer::new_empty(metrics);

        let (capital_height, _) = self.capital_info();

        let mut width = 0.0;
        for segment in segments.as_slice() {
            match segment {
                Segment::Emoji(_) => {
                    let x_spacer = capital_height as f32 * 0.1;
                    width += capital_height as f32 + x_spacer;
                }
                Segment::Text(text_segment) => {
                    buffer.set_text(
                        &mut self.font_system,
                        text_segment.as_str(),
                        attrs,
                        Shaping::Advanced,
                    );
                    width += buffer
                        .layout_runs()
                        .map(|run| run.glyphs.iter().map(|glyph| glyph.w).sum::<f32>())
                        .sum::<f32>()
                        .ceil();
                }
            }
        }

        width as u32
    }

    pub fn draw(&mut self, segments: &Segments, mut f: impl FnMut((i32, i32), [u8; 4])) {
        let metrics = Metrics::new(self.font_size, self.line_height);
        let attrs = Attrs::new();
        let mut buffer = Buffer::new_empty(metrics);

        let (capital_height, capital_line_y) = self.capital_info();
        let capital_info = self.capital_info();
        // TODO: expect or handle error
        let mut emoji_buffer = Pixmap::new(capital_height, capital_height).unwrap();
        let mut emoji_buffer = emoji_buffer.as_mut();

        let x_offset = (|| {
            let segment = segments.as_slice().first()?;
            let segment = segment.as_text()?;
            buffer.set_text(
                &mut self.font_system,
                segment.as_str(),
                attrs,
                Shaping::Advanced,
            );
            let run = buffer.layout_runs().next()?;
            let glyph = run.glyphs.iter().next()?;
            let glyph = glyph.physical((0., 0.), 1.0);
            let swash_image = self
                .swash_cache
                .get_image(&mut self.font_system, glyph.cache_key)
                .as_ref()?;
            Some(swash_image.placement.left)
        })()
        .unwrap_or(0);

        let mut x_advance = 0;
        for segment in segments.as_slice() {
            match segment {
                Segment::Emoji(emoji_segment) => {
                    self.draw_emoji_segment(
                        *emoji_segment,
                        &mut f,
                        &mut x_advance,
                        x_offset,
                        &mut emoji_buffer,
                        capital_info,
                    );
                }
                Segment::Text(text_segment) => {
                    self.draw_text_segment(
                        *text_segment,
                        &mut f,
                        &mut x_advance,
                        x_offset,
                        &mut buffer,
                        attrs,
                    );
                }
            }
        }
    }

    fn draw_text_segment(
        &mut self,
        segment: TextSegment,
        mut f: impl FnMut((i32, i32), [u8; 4]),
        x_advance: &mut i32,
        x_offset: i32,
        buffer: &mut Buffer,
        attrs: Attrs<'_>,
    ) {
        buffer.set_text(
            &mut self.font_system,
            segment.as_str(),
            attrs,
            Shaping::Advanced,
        );
        for run in buffer.layout_runs() {
            for glyph in run.glyphs.iter() {
                let physical_glyph = glyph.physical((0., 0.), 1.0);
                let glyph_color = glyph.color_opt.unwrap_or(cosmic_text::Color::rgba(
                    self.color[0],
                    self.color[1],
                    self.color[2],
                    self.color[3],
                ));
                let xd = |x| physical_glyph.x + x - x_offset + *x_advance;
                let yd = |y| run.line_y as i32 + physical_glyph.y + y;
                self.swash_cache.with_pixels(
                    &mut self.font_system,
                    physical_glyph.cache_key,
                    glyph_color,
                    |x, y, color| {
                        f((xd(x), yd(y)), color.as_rgba());
                    },
                );
            }

            *x_advance += run.line_w.ceil() as i32;
        }
    }

    fn draw_emoji_segment(
        &mut self,
        segment: EmojiSegment,
        mut f: impl FnMut((i32, i32), [u8; 4]),
        x_advance: &mut i32,
        x_offset: i32,
        buffer: &mut PixmapMut,
        (capital_height, capital_line_y): (u32, f32),
    ) {
        let tree = self.tree(segment);
        buffer.fill(tiny_skia::Color::TRANSPARENT);
        let scale = capital_height as f32 / tree.size().width();
        // remember to do the same x_spacer in `width`
        let x_spacer = (capital_height as f32 * 0.1) as i32;
        *x_advance += x_spacer;
        let transform = Transform::from_scale(scale, scale);
        resvg::render(tree, transform, buffer);
        let pixels = buffer
            .pixels_mut()
            .into_iter()
            .zip(pixel_iter(capital_height, capital_height));
        for (pixel, (x, y, _)) in pixels {
            let pixel = [pixel.red(), pixel.green(), pixel.blue(), pixel.alpha()];
            let x = x as i32 - x_offset + *x_advance;
            let y = y as i32 + capital_line_y as i32 - capital_height as i32;
            f((x, y), pixel);
        }
        *x_advance += capital_height as i32 + x_spacer;
    }

    /// Returns the estimated height and width of a capital letter
    /// for the current `font_size` and `line_height`.
    ///
    /// The tuple contains:
    /// - `u32`: the pixel height of the glyph, this is the number of vertical
    ///   pixels required to render it.
    /// - `f32`: the glyph's layout width, which may include subpixel precision.
    ///
    /// This is useful for aligning or sizing capital text in layouts.
    ///
    /// Internally, the capital letter `H` is used as a representative glyph, as
    /// it typically has consistent dimensions across fonts.
    /// The result is cached and reused for the same font size and line height.
    pub fn capital_info(&mut self) -> (u32, f32) {
        let key = (self.font_size.to_bits(), self.line_height.to_bits());
        if let Some(info) = self.capital_info.get(&key) {
            return info.clone();
        };

        let info = (|| {
            let metrics = Metrics::new(self.font_size, self.line_height);
            let attrs = Attrs::new();
            let mut buffer = Buffer::new_empty(metrics);
            buffer.set_text(&mut self.font_system, "H", attrs, Shaping::Advanced);
            let run = buffer.layout_runs().next()?;
            let glyph = run.glyphs.iter().next()?;
            let glyph = glyph.physical((0., 0.), 1.0);
            let swash_image = self
                .swash_cache
                .get_image(&mut self.font_system, glyph.cache_key)
                .as_ref()?;
            let height = swash_image.placement.height;
            let line_y = run.line_y;
            Some((height, line_y))
        })()
        .unwrap_or((0, 0.0));

        self.capital_info.insert(key, info.clone());
        return info;
    }

    pub fn tree(&mut self, segment: EmojiSegment) -> &Tree {
        self.tree_cache.entry(segment.emoji()).or_insert_with(|| {
            let options = Options::default();
            Tree::from_str(segment.svg(), &options).expect("twemoji-assets are valid svg")
        })
    }
}

impl Default for DrawingContext {
    fn default() -> Self {
        Self::new()
    }
}

fn pixel_iter(width: u32, height: u32) -> impl Iterator<Item = (u32, u32, usize)> {
    let x_iter = 0..width;
    let y_iter = 0..height;
    y_iter
        .enumerate()
        .map(move |(yi, y)| {
            x_iter
                .clone()
                .enumerate()
                .map(move |(xi, x)| (x, y, yi * width as usize + xi))
        })
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixel_iter_works() {
        let iter = pixel_iter(3, 4);
        let expected = vec![
            (0, 0, 00),
            (1, 0, 01),
            (2, 0, 02),
            (0, 1, 03),
            (1, 1, 04),
            (2, 1, 05),
            (0, 2, 06),
            (1, 2, 07),
            (2, 2, 08),
            (0, 3, 09),
            (1, 3, 10),
            (2, 3, 11),
        ];

        assert_eq!(iter.collect::<Vec<(u32, u32, usize)>>(), expected);
    }

    #[test]
    fn width_estimation_matches_drawn_output() {
        let mut ctx = DrawingContext::new();
        ctx.font_size(18.0);
    
        let samples = vec![
            "Hello",
            "W🎉ide",
            "👨‍👩‍👧‍👦", // complex emoji
            "你好，世界", // CJK characters
            "Hello 👋🌍 with more text",
        ];
    
        for sample in samples {
            let segments = Segments::new(sample);
            let mut max_x = 0;
    
            ctx.draw(&segments, |(x, _y), _color| {
                max_x = max_x.max(x);
            });
    
            let expected_width = ctx.width(&segments) as i32;
    
            assert!(
                max_x < expected_width,
                "Draw used out-of-bounds pixel: max_x = {max_x}, width = {expected_width}, text = \"{sample}\""
            );
    
            assert!(
                expected_width - max_x <= 1,
                "Width overestimated too much: width = {expected_width}, max_x = {max_x}, text = \"{sample}\""
            );
        }
    }
    
}
