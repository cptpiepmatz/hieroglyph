use twemoji_assets::svg::SvgTwemojiAsset;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TextSegment<'s>(&'s str);

impl<'s> TextSegment<'s> {
    pub fn as_str(&'s self) -> &'s str {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EmojiSegment(&'static SvgTwemojiAsset);

impl EmojiSegment {
    pub fn svg(&self) -> &'static str {
        self.0
    }

    pub fn emoji(&self) -> &'static str {
        self.0.emoji
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Segment<'s> {
    Text(TextSegment<'s>),
    Emoji(EmojiSegment),
}

impl<'s> Segment<'s> {
    pub fn as_text(self) -> Option<TextSegment<'s>> {
        match self {
            Segment::Text(text_segment) => Some(text_segment),
            Segment::Emoji(_) => None,
        }
    }

    pub fn as_emoji(self) -> Option<EmojiSegment> {
        match self {
            Segment::Text(_) => None,
            Segment::Emoji(emoji_segment) => Some(emoji_segment),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segments<'s>(Vec<Segment<'s>>);

impl<'s> Segments<'s> {
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
