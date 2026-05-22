use printpdf::ParsedFont;

const PT_TO_MM_FACTOR: f32 = 0.352_777_78;

/// Measures the height of text in millimetres.
#[allow(clippy::cast_precision_loss)]
pub fn measure_text_height_mm(font_size_pt: f32, line_height_factor: f32, num_lines: usize) -> f32 {
    let line_height_pt = font_size_pt * line_height_factor;
    let total_height = line_height_pt * num_lines as f32;
    pt_to_mm(total_height)
}

/// Measures the width of a text string in points.
#[allow(clippy::cast_precision_loss)]
pub fn measure_text_width_pt(text: &str, font: &ParsedFont, font_size_pt: f32) -> f32 {
    let units_per_em = f32::from(font.font_metrics.units_per_em);

    text.chars()
        .map(|c| {
            let glyph_index = font.lookup_glyph_index(c as u32).unwrap_or(0);
            let glyph_width = font.get_glyph_width_internal(glyph_index).unwrap_or(0) as f32;
            glyph_width / units_per_em * font_size_pt
        })
        .sum()
}

/// Converts points to millimeters.
pub fn pt_to_mm(pt: f32) -> f32 {
    pt * PT_TO_MM_FACTOR
}
