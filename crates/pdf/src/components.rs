/// How the current page number is displayed.
#[derive(Clone)]
pub enum PageNumberFormat {
    /// The shortest way to write the current page number, i.e. "1/3"
    Minimal,
    /// Long way of writing the current page, i.e. "Page 1 of 3"
    Long,
}

/// A horizontal position within a region
#[derive(Clone)]
pub enum Position {
    Left,
    Centre,
    Right,
}

/// Configuration for a page footer.
#[derive(Clone)]
pub struct FooterOptions {
    /// How the page number is written, e.g. "1/3" or "Page 1 of 3".
    pub format: PageNumberFormat,
    /// Where the page number appears in the footer.
    pub page_numbering_position: Position,
    /// Text shown at the left of the footer, if any.
    pub left_text: Option<String>,
    /// Text shown at the centre of the footer, if any.
    pub centre_text: Option<String>,
    /// Text shown at the right of the footer, if any.
    pub right_text: Option<String>,
}

impl FooterOptions {
    /// Creates footer options with the page number on the left.
    pub const fn new_page_number_left(
        format: PageNumberFormat,
        centre_text: Option<String>,
        right_text: Option<String>,
    ) -> Self {
        Self {
            format,
            page_numbering_position: Position::Left,
            left_text: None,
            centre_text,
            right_text,
        }
    }

    /// Creates footer options with the page number in the centre.
    pub const fn new_page_number_centre(
        format: PageNumberFormat,
        left_text: Option<String>,
        right_text: Option<String>,
    ) -> Self {
        Self {
            format,
            page_numbering_position: Position::Centre,
            left_text,
            centre_text: None,
            right_text,
        }
    }

    /// Creates footer options with the page number on the right.
    pub const fn new_page_number_right(
        format: PageNumberFormat,
        left_text: Option<String>,
        centre_text: Option<String>,
    ) -> Self {
        Self {
            format,
            page_numbering_position: Position::Right,
            left_text,
            centre_text,
            right_text: None,
        }
    }
}
