use twemoji_assets::svg::SvgTwemojiAsset;
use unicode_segmentation::UnicodeSegmentation;

/// `TextSegment` holds a reference to a snippet of the input string.
/// 
/// When the input string is split at emojis, each snippet of text found between emojis
/// is captured as a separate `TextSegment`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TextSegment<'s>(&'s str);

impl<'s> TextSegment<'s> {
    /// Returns the contained string slice.
    pub fn as_str(&'s self) -> &'s str {
        self.0
    }
}

/// `EmojiSegment` represents a single emoji from the input string.
/// 
/// Unlike [`TextSegment`], it doesn't hold a reference to the input text but stores an 
/// [`SvgTwemojiAsset`].
/// Use [`svg()`](EmojiSegment::svg) to get the SVG string for the emoji and 
/// [`emoji()`](EmojiSegment::emoji) to get the emoji character.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EmojiSegment(&'static SvgTwemojiAsset);

impl EmojiSegment {
    /// Returns the SVG string for this emoji.
    pub fn svg(&self) -> &'static str {
        self.0
    }

    /// Returns the emoji character represented by this segment.
    pub fn emoji(&self) -> &'static str {
        self.0.emoji
    }
}

/// Segment represents either a text or an emoji segment.
/// 
/// It can hold a [`TextSegment`] or an [`EmojiSegment`]. 
/// The methods [`as_text`](Segment::as_text) and [`as_emoji`](Segment::as_emoji) let you easily 
/// get the inner value as an [`Option`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Segment<'s> {
    Text(TextSegment<'s>),
    Emoji(EmojiSegment),
}

impl<'s> Segment<'s> {
    /// Returns the inner [`TextSegment`] if this is a text segment, or [`None`] otherwise.
    pub fn as_text(self) -> Option<TextSegment<'s>> {
        match self {
            Segment::Text(text_segment) => Some(text_segment),
            Segment::Emoji(_) => None,
        }
    }

    /// Returns the inner [`EmojiSegment`] if this is an emoji segment, or [`None`] otherwise.
    pub fn as_emoji(self) -> Option<EmojiSegment> {
        match self {
            Segment::Text(_) => None,
            Segment::Emoji(emoji_segment) => Some(emoji_segment),
        }
    }
}

/// Iterator over grapheme clusters and text segments of a string.
/// 
/// Use [`Segments::new`] as the main entry point for converting text into segments ready for 
/// rendering.
/// The same `Segments` instance can be used multiple times for rendering the same string multiple 
/// times, avoiding recalculating the clusters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segments<'s>(Vec<Segment<'s>>);

impl<'s> Segments<'s> {
    /// Creates a new Segments by splitting the input string into text and emoji segments.
    pub fn new(s: &'s str) -> Self {
        let mut segments = Vec::new();

        let mut current_offset = None;
        let graphemes = s.grapheme_indices(true);
        for (grapheme_offset, grapheme) in graphemes {
            if let Some(emoji) = SvgTwemojiAsset::from_emoji(grapheme) {
                let segment = Segment::Emoji(EmojiSegment(emoji));
                if let Some(offset) = current_offset {
                    let prev_text_segment = &s[offset..grapheme_offset];
                    let prev_text_segment = Segment::Text(TextSegment(prev_text_segment));
                    segments.push(prev_text_segment);
                    current_offset = None;
                }
                segments.push(segment);
                continue;
            }

            if current_offset.is_none() {
                current_offset = Some(grapheme_offset);
            }
        }

        if let Some(current_offset) = current_offset {
            let last = &s[current_offset..];
            let last = Segment::Text(TextSegment(last));
            segments.push(last);
        }

        Segments(segments)
    }

    /// Returns a slice of the computed segments.
    pub fn as_slice(&'s self) -> &'s [Segment<'s>] {
        &self.0
    }
}

#[cfg(test)]
#[test]
#[rustfmt::skip]
fn segmentation_works() {
    use twemoji_assets::svg_twemoji_asset;

    macro_rules! text { ($s:literal) => { Segment::Text(TextSegment($s)) }}
    macro_rules! emoji { ($e:tt) => { Segment::Emoji(EmojiSegment(svg_twemoji_asset!($e))) }}
    macro_rules! segments { ($($e:expr),*) => { Segments(vec![$($e),*])} }

    let input = "abc";
    let expected = segments![text!("abc")];
    assert_eq!(Segments::new(input), expected);

    let input = "🌈";
    let expected = segments![emoji!("🌈")];
    assert_eq!(Segments::new(input), expected);

    let input = "duck 🦆";
    let expected = segments![text!("duck "), emoji!("🦆")];
    assert_eq!(Segments::new(input), expected);

    let input = "Hello, world! 🌍🎉";
    let expected = segments![text!("Hello, world! "), emoji!("🌍"), emoji!("🎉")];
    assert_eq!(Segments::new(input), expected);

    let input = "🔥🔥🔥 Fire emoji!";
    let expected = segments![emoji!("🔥"), emoji!("🔥"), emoji!("🔥"), text!(" Fire emoji!")];
    assert_eq!(Segments::new(input), expected);

    let input = "🤔 What do you think? 😄";
    let expected = segments![emoji!("🤔"), text!(" What do you think? "), emoji!("😄")];
    assert_eq!(Segments::new(input), expected);

    let input = "😊🌼🌟 Happy flowers and stars!";
    let expected = segments![emoji!("😊"), emoji!("🌼"), emoji!("🌟"), text!(" Happy flowers and stars!")];
    assert_eq!(Segments::new(input), expected);

    let input = "🚀Launch!🚀";
    let expected = segments![emoji!("🚀"), text!("Launch!"), emoji!("🚀")];
    assert_eq!(Segments::new(input), expected);

    let input = "";
    let expected = segments![];
    assert_eq!(Segments::new(input), expected);
}
