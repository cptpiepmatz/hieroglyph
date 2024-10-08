use cosmic_text::fontdb::Database;
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};

use crate::{fonts, Segment, Segments};

#[derive(Debug)]
pub struct DrawingContext {
    font_system: FontSystem,
    swash_cache: SwashCache,
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
                    for run in buffer.layout_runs() {
                        for glyph in run.glyphs.iter() {
                            let physical_glyph = glyph.physical((0., 0.), 1.0);
                            let glyph_color = glyph.color_opt.unwrap_or(cosmic_text::Color::rgba(
                                self.color[0],
                                self.color[1],
                                self.color[2],
                                self.color[3],
                            ));
                            let xd = |x| physical_glyph.x + x - x_offset;
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
                    }
                }
            }
        }
    }
}

impl Default for DrawingContext {
    fn default() -> Self {
        Self::new()
    }
}
