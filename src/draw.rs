use std::collections::HashMap;

use cosmic_text::fontdb::Database;
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};
use resvg::tiny_skia::{self, Pixmap, PixmapMut};
use resvg::usvg::{Options, Transform, Tree};

use crate::{fonts, EmojiSegment, Segment, Segments};

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
}

impl DrawingContext {
    pub fn new() -> Self {
        use fonts::{NOTO_REST, NOTO_SANS, NOTO_SERIF};

        let mut font_db = Database::new();
        NOTO_SANS
            .iter()
            .chain(NOTO_SERIF.iter())
            .chain(NOTO_REST.iter())
            .for_each(|&bytes| font_db.load_font_data(Vec::from(bytes)));
        let font_system = FontSystem::new_with_locale_and_db("en".to_string(), font_db);

        Self {
            font_size: 12.0,
            color: [0, 0, 0, 255],

            font_system,
            swash_cache: SwashCache::new(),
            tree_cache: TreeCache::new(),
            line_height: 50.0, // irrelevant for users right now as we do not handle newlines
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

        let mut width = 0;
        for segment in segments.as_slice() {
            match segment {
                Segment::Emoji(emoji_segment) => todo!(),
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
                        .ceil() as u32;
                }
            }
        }

        width
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
                    self.draw_emoji_segment(*emoji_segment, &mut f, &mut x_advance, x_offset, &mut emoji_buffer, capital_info);
                }
                Segment::Text(text_segment) => {
                    buffer.set_text(
                        &mut self.font_system,
                        text_segment.as_str(),
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
                            let xd = |x| physical_glyph.x + x - x_offset + x_advance;
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

                        x_advance += run.line_w.ceil() as i32;
                    }
                }
            }
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
        let x_spacer = (capital_height as f32 * 0.1) as i32;
        *x_advance += x_spacer;
        let transform = Transform::from_scale(scale, scale);
        resvg::render(tree, transform, buffer);
        let pixels = buffer.pixels_mut().into_iter().zip(pixel_iter(capital_height, capital_height));
        for (pixel, (x, y, _)) in pixels {
            let pixel = [pixel.red(), pixel.green(), pixel.blue(), pixel.alpha()];
            let x = x as i32 - x_offset + *x_advance;
            let y = y as i32 + capital_line_y as i32 - capital_height as i32;
            f((x, y), pixel);
        }
        *x_advance += capital_height as i32 + x_spacer;
    }

    pub fn capital_info(&mut self) -> (u32, f32) {
        (|| {
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
        .unwrap_or((0, 0.0))
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
}
