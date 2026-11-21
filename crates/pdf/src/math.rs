use std::borrow::Cow;

use krilla::text::Font;
use skrifa::{
    FontRef, MetadataProvider,
    instance::{Location, Size},
};

/// Page margin on all sides, in points (72pt = 1").
pub const MARGIN_PT: f32 = 72.0;

/// Font size for small body text, in points.
pub const FONT_SIZE_BODY_SMALL_PT: f32 = 9.0;

/// Font size for body text, in points.
pub const FONT_SIZE_BODY_PT: f32 = 11.0;

/// Font size for titles, in points.
pub const FONT_SIZE_TITLE_PT: f32 = 14.0;

/// Precomputed line heights for text styles in the PDF.
pub struct LineMetrics {
    pub title: f32,
    pub body: f32,
    pub body_tight: f32,
    pub small: f32,
    pub small_half: f32,
}

impl Default for LineMetrics {
    fn default() -> Self {
        Self {
            title: measure_text_height(FONT_SIZE_TITLE_PT, 1.25, 1) + 5.0,
            body: measure_text_height(FONT_SIZE_BODY_PT, 1.25, 1),
            body_tight: measure_text_height(FONT_SIZE_BODY_PT, 1.0, 1),
            small: measure_text_height(FONT_SIZE_BODY_SMALL_PT, 1.0, 1),
            small_half: measure_text_height(FONT_SIZE_BODY_SMALL_PT, 0.50, 1),
        }
    }
}

/// Measures the height of text in points.
#[allow(clippy::cast_precision_loss)]
pub fn measure_text_height(font_size_pt: f32, line_height_factor: f32, num_lines: usize) -> f32 {
    let line_height_pt = font_size_pt * line_height_factor;
    line_height_pt * num_lines as f32
}

/// Measures the width of text in points.
///
/// # Panics
///
/// Panics if the font data is invalid or the font index is out of bounds.
pub fn measure_text_width_pt(
    text: &str,
    font: &Font,
    font_data: &[u8],
    font_index: u32,
    font_size_pt: f32,
) -> f32 {
    let units_per_em = font.units_per_em();

    let font_ref = FontRef::from_index(font_data, font_index).unwrap();
    let loc = Location::default();
    let glyph_metrics = font_ref.glyph_metrics(Size::unscaled(), &loc);
    let charmap = font_ref.charmap();

    text.chars()
        .map(|c| {
            let gid = charmap.map(c).unwrap_or_default();
            glyph_metrics.advance_width(gid).unwrap_or(0.0) / units_per_em * font_size_pt
        })
        .sum()
}

/// Wrap a line of text at a calculated width.
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
pub fn wrap_text(text: &str, width: f32) -> Vec<Cow<'_, str>> {
    let avg_char_width_pt = FONT_SIZE_BODY_PT * 0.5;
    let max_chars = ((width / avg_char_width_pt).floor() as usize).max(1);
    textwrap::wrap(text, max_chars)
}
