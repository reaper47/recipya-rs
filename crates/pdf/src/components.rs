use printpdf::{
    FontId, Line, LinePoint, Mm, Op, PaintMode, ParsedFont, PdfFontHandle, Point, Polygon,
    PolygonRing, Pt, TextItem, WindingOrder,
};

use crate::math::{measure_text_width_pt, pt_to_mm};

/// Options for a component.
#[derive(Clone)]
pub struct ComponentOptions {
    pub font: ParsedFont,
    pub font_id: FontId,
    pub font_size: f32,
    pub font_height: f32,
    pub page_width_mm: f32,
    pub page_height_mm: f32,
    pub margin_mm: f32,
}

/// Draws a checkbox at the given margin and current y position.
pub fn checkbox(margin_mm: f32, current_y_mm: f32) -> Op {
    Op::DrawPolygon {
        polygon: Polygon {
            rings: vec![PolygonRing {
                points: vec![
                    LinePoint {
                        p: Point {
                            x: Mm(margin_mm - 5.0).into_pt(),
                            y: Mm(current_y_mm).into_pt(),
                        },
                        bezier: false,
                    },
                    LinePoint {
                        p: Point {
                            x: Mm(margin_mm - 2.5).into_pt(),
                            y: Mm(current_y_mm).into_pt(),
                        },
                        bezier: false,
                    },
                    LinePoint {
                        p: Point {
                            x: Mm(margin_mm - 2.5).into_pt(),
                            y: Mm(current_y_mm + 2.5).into_pt(),
                        },
                        bezier: false,
                    },
                    LinePoint {
                        p: Point {
                            x: Mm(margin_mm - 5.0).into_pt(),
                            y: Mm(current_y_mm + 2.5).into_pt(),
                        },
                        bezier: false,
                    },
                ],
            }],
            mode: PaintMode::Stroke,
            winding_order: WindingOrder::NonZero,
        },
    }
}

/// Returns the header section as a vector of PDF operations.
pub fn header(
    left_text: Option<&str>,
    right_text: Option<&str>,
    options: &ComponentOptions,
) -> Vec<Op> {
    let left_ops: &[Op] = if let Some(s) = left_text {
        &[
            Op::SaveGraphicsState,
            Op::SetFont {
                font: PdfFontHandle::External(options.font_id.clone()),
                size: Pt(options.font_size),
            },
            Op::SetLineHeight {
                lh: Pt(options.font_height),
            },
            Op::StartTextSection,
            Op::SetTextCursor {
                pos: Point::new(Mm(10.0), Mm(options.page_height_mm - 9.0)),
            },
            Op::ShowText {
                items: vec![TextItem::Text(s.into())],
            },
            Op::EndTextSection,
        ]
    } else {
        &[
            Op::SaveGraphicsState,
            Op::SetFont {
                font: PdfFontHandle::External(options.font_id.clone()),
                size: Pt(options.font_size),
            },
            Op::SetLineHeight {
                lh: Pt(options.font_height),
            },
        ]
    };

    let right_ops: &[Op] = if let Some(s) = right_text {
        &[
            Op::StartTextSection,
            Op::SetTextCursor {
                pos: Point::new(
                    Mm(options.page_width_mm - options.margin_mm),
                    Mm(options.page_height_mm - 9.0),
                ),
            },
            Op::ShowText {
                items: vec![TextItem::Text(s.into())],
            },
            Op::EndTextSection,
            Op::RestoreGraphicsState,
        ]
    } else {
        &[Op::EndTextSection, Op::RestoreGraphicsState]
    };

    [left_ops, right_ops].concat()
}

pub fn title(title: &str, mut current_y_mm: f32, options: ComponentOptions) -> Vec<Op> {
    let title_width_pt = measure_text_width_pt(title, &options.font, options.font_size);
    let y_pos_mm = options.page_height_mm - options.margin_mm;
    let center_x_mm = pt_to_mm((Mm(options.page_width_mm).into_pt().0 - title_width_pt) / 2.0);

    vec![
        Op::SaveGraphicsState,
        Op::SetFont {
            font: PdfFontHandle::External(options.font_id),
            size: Pt(options.font_size),
        },
        Op::StartTextSection,
        Op::SetTextCursor {
            pos: Point::new(Mm(center_x_mm), Mm(y_pos_mm)),
        },
        Op::ShowText {
            items: vec![TextItem::Text(title.into())],
        },
        Op::EndTextSection,
        Op::SetOutlineThickness { pt: Pt(1.0) },
        {
            current_y_mm -= 0.5;

            Op::DrawLine {
                line: Line {
                    points: vec![
                        LinePoint {
                            p: Point {
                                x: Mm(center_x_mm).into_pt(),
                                y: Mm(current_y_mm).into_pt(),
                            },
                            bezier: false,
                        },
                        LinePoint {
                            p: Point {
                                x: Mm(center_x_mm).into_pt() + Pt(title_width_pt),
                                y: Mm(current_y_mm).into_pt(),
                            },
                            bezier: false,
                        },
                    ],
                    is_closed: false,
                },
            }
        },
        Op::RestoreGraphicsState,
    ]
}
