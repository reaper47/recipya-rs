use std::{cmp::Ordering, marker::PhantomData, mem::take};

use krilla::{
    geom::{PathBuilder, Rect},
    surface::Surface,
    text::{Font, TextDirection},
};

use crate::{
    commands::DrawCommand,
    components::{FooterOptions, PageNumberFormat, Position},
    fonts::{DocumentFont, ROBOTO_REGULAR_FONT_BYTES, ROBOTO_SEMIBOLD_FONT_BYTES},
    math::{
        FONT_SIZE_BODY_PT, FONT_SIZE_BODY_SMALL_PT, FONT_SIZE_TITLE_PT, LineMetrics, MARGIN_PT,
        measure_text_width_pt, wrap_text,
    },
};

/// Initial state of a document, before any content has been prepared.
pub struct InitialState(pub !);

/// State in which content is being prepared before drawing.
pub struct PrepareState(pub !);

/// State in which prepared content is drawn to the page.
pub struct DrawState(pub !);

/// Marker trait for the states a document can be in.
pub trait DocumentState {}
impl DocumentState for InitialState {}
impl DocumentState for PrepareState {}
impl DocumentState for DrawState {}

/// The size of a page, in points.
pub struct PageDimensions(Width, Height);

/// A width in points.
pub struct Width(f32);

/// A height in points.
pub struct Height(f32);

impl PageDimensions {
    /// Creates page dimensions from a width and height in points.
    pub const fn new(width: f32, height: f32) -> Self {
        Self(Width(width), Height(height))
    }

    /// Returns the width available to each of two side-by-side columns after subtracting the margins.
    pub fn half_width(&self) -> f32 {
        (self.0.0 - MARGIN_PT) / 2.0
    }

    /// Returns the page width in points.
    pub const fn width(&self) -> f32 {
        self.0.0
    }

    /// Returns the page height in points.
    pub const fn height(&self) -> f32 {
        self.1.0
    }
}

/// The components of a PDF document being laid out.
pub struct Document<S: DocumentState> {
    ctx: DocumentContext,
    marker: PhantomData<S>,
}

struct DocumentContext {
    curr_y: f32,
    line_metrics: LineMetrics,
    font: DocumentFont,
    current_page: Vec<DrawCommand>,
    pages: Vec<Vec<DrawCommand>>,
    page_dimensions: PageDimensions,
    footer: FooterOptions,
}

impl Document<InitialState> {
    /// Creates an empty `Document` with no pages.
    pub fn new(page_dimensions: PageDimensions, footer: FooterOptions) -> Document<PrepareState> {
        Document {
            ctx: DocumentContext {
                curr_y: MARGIN_PT,
                line_metrics: LineMetrics::default(),
                font: DocumentFont::roboto(),
                current_page: Vec::new(),
                pages: Vec::new(),
                page_dimensions,
                footer,
            },
            marker: PhantomData,
        }
    }
}

impl Document<PrepareState> {
    /// Add a drawing command the current page.
    pub fn push(&mut self, command: DrawCommand) {
        self.ctx.current_page.push(command);
    }

    // Adds a header drawing command to the current page.
    pub fn push_header(&mut self, left_text: Option<String>, right_text: Option<String>) {
        self.ctx.current_page.push(DrawCommand::Header {
            left_text,
            right_text,
        });
    }

    // Add a title drawing command to the current page.
    pub fn push_title(&mut self, text: &str) {
        self.ctx.current_page.push(DrawCommand::Title {
            text: text.into(),
            y: self.ctx.curr_y,
        });
        self.increment_y(self.ctx.line_metrics.title);
    }

    /// Add a metadata drawing command to the current page.
    pub fn push_metadata(&mut self, data: Vec<(String, String)>) {
        let max_width = data
            .iter()
            .map(|(key, _)| {
                measure_text_width_pt(
                    key,
                    &self.ctx.font.semibold(),
                    ROBOTO_SEMIBOLD_FONT_BYTES,
                    0,
                    FONT_SIZE_BODY_SMALL_PT,
                )
            })
            .max_by(|x, y| x.abs().partial_cmp(&y.abs()).unwrap_or(Ordering::Equal))
            .unwrap_or_default();

        for (key, value) in data {
            self.push(DrawCommand::MetaData {
                key,
                value,
                max_width,
                y: self.ctx.curr_y,
            });
            self.increment_y(self.ctx.line_metrics.small);
        }
    }

    /// Adds a label drawing commamnd to the current page.
    pub fn push_label(&mut self, text: &str, x_offset: Option<f32>) {
        self.ctx.current_page.push(DrawCommand::SectionLabel {
            text: text.into(),
            x: x_offset,
            y: self.ctx.curr_y,
        });
        self.increment_y(self.ctx.line_metrics.body);
    }

    /// Adds a horizontal line drawing command to the current page.
    pub fn push_line_across_page(&mut self) {
        self.ctx.current_page.push(DrawCommand::Line {
            start_x: MARGIN_PT,
            end_x: self.page_dimensions().width() - MARGIN_PT,
            y: self.ctx.curr_y,
        });
    }

    /// Adds a list with checkable items and returns the width of the longest item.
    ///
    /// # Panics
    ///
    /// Panics if an item wraps to zero lines, or if a measured width is `NaN`.
    pub fn push_checkbox_list(&mut self, items: Vec<String>, list_width_pt: f32) -> f32 {
        let mut max_item_width = 0.0;
        for item in items {
            self.push(DrawCommand::Checkbox { y: self.ctx.curr_y });
            let texts = wrap_text(&item, list_width_pt);

            let max_width = texts
                .iter()
                .map(|s| {
                    measure_text_width_pt(
                        s,
                        &self.ctx.font.regular(),
                        ROBOTO_REGULAR_FONT_BYTES,
                        0,
                        FONT_SIZE_BODY_PT,
                    )
                })
                .max_by(|x, y| x.abs().partial_cmp(&y.abs()).unwrap())
                .unwrap()
                + 10.0;
            if max_width > max_item_width {
                max_item_width = max_width;
            }

            for s in texts {
                self.push_text(&s, self.ctx.font.regular(), FONT_SIZE_BODY_PT, None);
                self.increment_y(self.ctx.line_metrics.body);
            }
        }
        max_item_width
    }

    /// Adds multiple paragraphs to the current page of the document.
    pub fn push_paragraphs(&mut self, texts: &[String], x_offset: f32) {
        const ROW_STEP: f32 = 7.5 * 2.834_645_7;
        let max_y = self.ctx.page_dimensions.height() - MARGIN_PT;

        for (idx, text) in texts.iter().enumerate() {
            if ROW_STEP.mul_add(3.0, self.ctx.curr_y) >= max_y {
                self.new_page();
            }

            let text = format!("{}. {}", idx + 1, text);
            let texts = wrap_text(&text, self.page_dimensions().width() - x_offset - MARGIN_PT);
            for (j, text) in texts.iter().enumerate() {
                if ROW_STEP.mul_add(3.0, self.ctx.curr_y) >= max_y {
                    self.new_page();
                }

                self.push_text(
                    text,
                    self.ctx.font.regular(),
                    FONT_SIZE_BODY_PT,
                    Some(x_offset),
                );

                if j < texts.len() - 1 {
                    self.increment_y(self.ctx.line_metrics.body);
                }
            }
            self.increment_y(1.5 * self.ctx.line_metrics.body);
        }
    }

    /// Adds a text drawing command to the current page of the document.
    pub fn push_text(&mut self, text: &str, font: Font, font_size: f32, x_offset: Option<f32>) {
        self.ctx.current_page.push(DrawCommand::ItemText {
            text: text.into(),
            font,
            font_size,
            x: x_offset,
            y: self.ctx.curr_y,
        });
    }

    /// Gets the document's font.
    pub const fn font(&self) -> &DocumentFont {
        &self.ctx.font
    }

    /// Increments the current y position of the document by a number of points.
    pub fn increment_y(&mut self, pt: f32) {
        self.ctx.curr_y += pt;
    }

    /// Gets the current y position of the document.
    pub const fn y(&self) -> f32 {
        self.ctx.curr_y
    }

    /// Sets the current y position of the document.
    pub const fn set_y(&mut self, pt: f32) {
        self.ctx.curr_y = pt;
    }

    pub const fn page_dimensions(&self) -> &PageDimensions {
        &self.ctx.page_dimensions
    }

    /// Gets the context's line metrics.
    pub const fn line_metrics(&self) -> &LineMetrics {
        &self.ctx.line_metrics
    }

    /// Finishes the current page and creates a new one.
    pub fn new_page(&mut self) {
        let header = self
            .ctx
            .current_page
            .iter()
            .position(|i| matches!(i, DrawCommand::Header { .. }))
            .map(|idx| self.ctx.current_page[idx].clone());

        self.push(DrawCommand::Footer);
        let finished = take(&mut self.ctx.current_page);
        self.ctx.pages.push(finished);

        if let Some(cmd) = header {
            self.ctx.current_page.push(cmd);
        }
        self.set_y(MARGIN_PT);
    }

    /// Finishes the document and moves the document into the draw state.
    ///
    /// This consumes the `Document`, so it cannot be used afterwards.
    pub fn finalise(mut self) -> Document<DrawState> {
        self.push(DrawCommand::Footer);
        let last_page = take(&mut self.ctx.current_page);
        self.ctx.pages.push(last_page);
        Document {
            ctx: self.ctx,
            marker: PhantomData,
        }
    }
}

impl Document<DrawState> {
    /// Gets the document's pages.
    pub fn pages(&self) -> &[Vec<DrawCommand>] {
        &self.ctx.pages
    }

    /// Draws the component on the PDF's [`Surface`].
    pub fn draw(&self, command: &DrawCommand, surface: &mut Surface, page_number: usize) {
        match command {
            DrawCommand::Header {
                left_text,
                right_text,
            } => self.draw_header(surface, left_text.as_deref(), right_text.as_deref()),
            DrawCommand::Title { text, y } => self.draw_title(surface, text, *y),
            DrawCommand::MetaData {
                key,
                value,
                max_width,
                y,
            } => self.draw_metadata(surface, key, value, *max_width, *y),
            DrawCommand::SectionLabel { text, x, y } => self.draw_label(surface, text, *x, *y),
            DrawCommand::Checkbox { y } => Self::draw_checkbox(surface, *y),
            DrawCommand::ItemText {
                text,
                font,
                font_size,
                x,
                y,
            } => Self::draw_text(surface, text, font.clone(), *font_size, *x, *y),
            DrawCommand::Line { start_x, end_x, y } => {
                Self::draw_line(surface, *start_x, *end_x, *y);
            }
            DrawCommand::Footer => {
                self.draw_footer(surface, page_number);
            }
        }
    }

    fn draw_header(
        &self,
        surface: &mut Surface,
        left_text: Option<&str>,
        right_text: Option<&str>,
    ) {
        let font = self.ctx.font.regular();
        let font_size = FONT_SIZE_TITLE_PT / 2.0;
        let font_data = ROBOTO_REGULAR_FONT_BYTES;
        let y = MARGIN_PT * 0.3333;

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
            let text_width = measure_text_width_pt(s, &font, font_data, 0, font_size);

            surface.draw_text(
                krilla::geom::Point::from_xy(
                    self.ctx.page_dimensions.width() - MARGIN_PT.mul_add(0.3333, text_width),
                    y,
                ),
                font,
                font_size,
                s,
                false,
                TextDirection::Auto,
            );
        }
    }

    fn draw_title(&self, surface: &mut Surface, title: &str, y: f32) {
        let page_width = self.ctx.page_dimensions.width();
        let (font, font_data, font_size) = (
            &self.ctx.font.regular(),
            ROBOTO_REGULAR_FONT_BYTES,
            FONT_SIZE_TITLE_PT,
        );

        let title_width = measure_text_width_pt(title, font, font_data, 0, font_size);

        surface.draw_text(
            krilla::geom::Point::from_xy((page_width - title_width) / 2.0, y),
            font.clone(),
            font_size,
            title,
            false,
            TextDirection::Auto,
        );

        let underline_offset = font_size * 0.10;
        let underline_thickness = font_size * 0.06;
        let underline_y = y + underline_offset;

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

    fn draw_metadata(&self, surface: &mut Surface, key: &str, value: &str, max_width: f32, y: f32) {
        surface.draw_text(
            krilla::geom::Point::from_xy(MARGIN_PT, y),
            self.ctx.font.semibold(),
            FONT_SIZE_BODY_SMALL_PT,
            key,
            false,
            TextDirection::Auto,
        );
        surface.draw_text(
            krilla::geom::Point::from_xy(MARGIN_PT + max_width + 3.0, y),
            self.ctx.font.regular(),
            FONT_SIZE_BODY_SMALL_PT,
            value,
            false,
            TextDirection::Auto,
        );
    }

    fn draw_label(&self, surface: &mut Surface, text: &str, x: Option<f32>, y: f32) {
        surface.draw_text(
            krilla::geom::Point::from_xy(MARGIN_PT + x.unwrap_or_default(), y),
            self.ctx.font.semibold(),
            FONT_SIZE_BODY_PT,
            text,
            false,
            TextDirection::Auto,
        );
    }

    fn draw_checkbox(surface: &mut Surface, y: f32) {
        surface.set_stroke(Some(krilla::paint::Stroke {
            paint: krilla::color::rgb::Color::new(0, 0, 0).into(),
            width: FONT_SIZE_BODY_PT * 0.06,
            ..Default::default()
        }));
        let mut path = PathBuilder::new();
        path.push_rect(
            Rect::from_ltrb(MARGIN_PT - 12.0, y - 8.0, MARGIN_PT - 12.0 + 7.5, y - 0.5).unwrap(),
        );
        surface.draw_path(&path.finish().unwrap());
        surface.set_stroke(None);
    }

    fn draw_text(
        surface: &mut Surface,
        text: &str,
        font: Font,
        font_size: f32,
        x: Option<f32>,
        y: f32,
    ) {
        surface.draw_text(
            krilla::geom::Point::from_xy(MARGIN_PT + x.unwrap_or_default(), y),
            font,
            font_size,
            text,
            false,
            TextDirection::Auto,
        );
    }

    fn draw_line(surface: &mut Surface, x1: f32, x2: f32, y: f32) {
        let underline_offset = FONT_SIZE_BODY_PT * 0.10;
        let underline_y = y + underline_offset;

        let underline = {
            let mut pb = PathBuilder::new();
            pb.move_to(x1, underline_y);
            pb.line_to(x2, underline_y);
            pb.finish().unwrap()
        };

        surface.set_stroke(Some(krilla::paint::Stroke {
            paint: krilla::color::rgb::Color::new(0, 0, 0).into(),
            width: FONT_SIZE_BODY_PT * 0.06,
            ..Default::default()
        }));
        surface.draw_path(&underline);
        surface.set_stroke(None);
    }

    fn draw_footer(&self, surface: &mut Surface, current_page_number: usize) {
        let page_width = self.ctx.page_dimensions.width();
        let page_height = self.ctx.page_dimensions.height();

        let mut left_text = self.ctx.footer.left_text.as_deref();
        let mut centre_text = self.ctx.footer.centre_text.as_deref();
        let mut right_text = self.ctx.footer.right_text.as_deref();

        let num_pages = self.ctx.pages.len();
        let page_number_text = match self.ctx.footer.format {
            PageNumberFormat::Minimal => format!("{current_page_number}/{num_pages}"),
            PageNumberFormat::Long => format!("Page {current_page_number} of {num_pages}"),
        };

        match self.ctx.footer.page_numbering_position {
            Position::Left => left_text = Some(page_number_text.as_str()),
            Position::Centre => centre_text = Some(page_number_text.as_str()),
            Position::Right => right_text = Some(page_number_text.as_str()),
        }

        if let Some(text) = left_text {
            surface.draw_text(
                krilla::geom::Point::from_xy(MARGIN_PT * 0.3333, page_height - 20.0),
                self.ctx.font.regular(),
                FONT_SIZE_BODY_PT - 3.0,
                text,
                false,
                TextDirection::Auto,
            );
        }

        if let Some(text) = centre_text {
            let (font, font_data, font_size) = (
                &self.ctx.font.regular(),
                ROBOTO_REGULAR_FONT_BYTES,
                FONT_SIZE_BODY_PT - 3.0,
            );
            let text_width = measure_text_width_pt(text, font, font_data, 0, font_size);
            let centre_x = self.ctx.page_dimensions.half_width();

            surface.draw_text(
                krilla::geom::Point::from_xy(centre_x - text_width / 2.0, page_height - 20.0),
                self.ctx.font.regular(),
                font_size,
                text,
                false,
                TextDirection::Auto,
            );
        }

        if let Some(text) = right_text {
            let footer_width = measure_text_width_pt(
                text,
                &self.ctx.font.regular(),
                ROBOTO_REGULAR_FONT_BYTES,
                0,
                FONT_SIZE_BODY_PT - 3.0,
            );
            surface.draw_text(
                krilla::geom::Point::from_xy(
                    page_width - MARGIN_PT.mul_add(0.3333, footer_width),
                    page_height - 20.0,
                ),
                self.ctx.font.regular(),
                FONT_SIZE_BODY_PT - 3.0,
                text,
                false,
                TextDirection::Auto,
            );
        }
    }
}
