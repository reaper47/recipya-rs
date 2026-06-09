use krilla::text::Font;
use skrifa::{
    FontRef, MetadataProvider,
    instance::{Location, Size},
};

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
