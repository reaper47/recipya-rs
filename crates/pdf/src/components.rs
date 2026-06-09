use krilla::{
    geom::PathBuilder,
    surface::Surface,
    text::{Font, TextDirection},
};

use crate::math::measure_text_width_pt;

/// Adds a page title to a PDF page.
///
/// # Panics
///
/// Panics if the font data is invalid or the font index is out of bounds.
pub fn add_page_title(
    title: &str,
    font: (&Font, &[u8], f32),
    surface: &mut Surface,
    curr_y: f32,
    page_width: f32,
) {
    let (font, font_data, font_size) = font;

    let title_width = measure_text_width_pt(title, font, font_data, 0, font_size);

    surface.draw_text(
        krilla::geom::Point::from_xy((page_width - title_width) / 2.0, curr_y),
        font.clone(),
        font_size,
        title,
        false,
        TextDirection::Auto,
    );

    let underline_offset = font_size * 0.10;
    let underline_thickness = font_size * 0.06;
    let underline_y = curr_y + underline_offset;

    let underline = {
        let mut pb = PathBuilder::new();
        pb.move_to((page_width - title_width) / 2.0, underline_y);
        pb.line_to(
            ((page_width - title_width) / 2.0) + title_width,
            underline_y,
        );
        pb.finish().unwrap()
    };

    surface.set_stroke(Some(krilla::paint::Stroke {
        paint: krilla::color::rgb::Color::new(0, 0, 0).into(),
        width: underline_thickness,
        ..Default::default()
    }));
    surface.draw_path(&underline);
    surface.set_stroke(None);
}

/// Adds the header to a PDF page.
pub fn add_header(
    left_text: Option<&str>,
    right_text: Option<&str>,
    surface: &mut Surface,
    font: (&Font, &[u8], f32),
    margin: f32,
    page_width: f32,
) {
    let (font, font_data, font_size) = font;
    let y = margin * 0.3333;

    if let Some(s) = left_text {
        surface.draw_text(
            krilla::geom::Point::from_xy(y, y),
            font.clone(),
            font_size,
            s,
            false,
            TextDirection::Auto,
        );
    }

    if let Some(s) = right_text {
        let text_width = measure_text_width_pt(s, font, font_data, 0, font_size);

        surface.draw_text(
            krilla::geom::Point::from_xy(page_width - margin.mul_add(0.3333, text_width), y),
            font.clone(),
            font_size,
            s,
            false,
            TextDirection::Auto,
        );
    }
}
