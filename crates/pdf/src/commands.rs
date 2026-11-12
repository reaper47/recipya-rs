use krilla::text::Font;

/// A positioned drawing operation in the PDF layout.
#[derive(Clone)]
pub enum DrawCommand {
    Header {
        left_text: Option<String>,
        right_text: Option<String>,
    },
    Title {
        text: String,
        y: f32,
    },
    MetaData {
        key: String,
        value: String,
        max_width: f32,
        y: f32,
    },
    SectionLabel {
        text: String,
        x: Option<f32>,
        y: f32,
    },
    Checkbox {
        y: f32,
    },
    ItemText {
        text: String,
        font: Font,
        font_size: f32,
        x: Option<f32>,
        y: f32,
    },
    Line {
        start_x: f32,
        end_x: f32,
        y: f32,
    },
    Footer,
}
