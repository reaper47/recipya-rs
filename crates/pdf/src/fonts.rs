use krilla::text::Font;

pub const ROBOTO_LIGHT_FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/Roboto-Light.ttf");
pub const ROBOTO_MEDIUM_FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/Roboto-Medium.ttf");
pub const ROBOTO_REGULAR_FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/Roboto-Regular.ttf");
pub const ROBOTO_SEMIBOLD_FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/Roboto-SemiBold.ttf");

/// A family of fonts with different weights.
pub struct DocumentFont {
    light: Font,
    medium: Font,
    regular: Font,
    semibold: Font,
}

impl DocumentFont {
    /// Loads the Roboto fonts from the embedded font data.
    ///
    /// # Panics
    ///
    /// Panics when the font cannot be loaded.
    pub fn roboto() -> Self {
        Self {
            light: Font::new(ROBOTO_LIGHT_FONT_BYTES.into(), 0).unwrap(),
            medium: Font::new(ROBOTO_MEDIUM_FONT_BYTES.into(), 0).unwrap(),
            regular: Font::new(ROBOTO_REGULAR_FONT_BYTES.into(), 0).unwrap(),
            semibold: Font::new(ROBOTO_SEMIBOLD_FONT_BYTES.into(), 0).unwrap(),
        }
    }

    /// Returns the light weight font.
    pub fn light(&self) -> Font {
        self.light.clone()
    }

    /// Returns the medium weight font.
    pub fn medium(&self) -> Font {
        self.medium.clone()
    }

    /// Returns the regular weight font.
    pub fn regular(&self) -> Font {
        self.regular.clone()
    }

    /// Returns the semibold weight font.
    pub fn semibold(&self) -> Font {
        self.semibold.clone()
    }
}
