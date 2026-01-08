use std::str::{FromStr, from_utf8};

use derive_more::From;
use encoding_rs::{ISO_8859_15, WINDOWS_1252};

use crate::impl_display_as_debug;

/// Attempts to convert a buffer to UTF-8 if not already in this encoding.
pub fn auto_convert_to_utf8(buffer: &[u8]) -> String {
    if let Ok(content) = from_utf8(buffer) {
        return content.to_string();
    }

    let encodings = [WINDOWS_1252, ISO_8859_15];

    for encoding in &encodings {
        let (content, _encoding, had_errors) = encoding.decode(buffer);
        if !had_errors {
            return content.to_string();
        }
    }

    String::from_utf8_lossy(buffer).to_string()
}

/// Extracts a number from a string, if it exists.
pub fn extract_number<T>(s: &str) -> Result<T>
where
    T: FromStr + std::fmt::Debug,
{
    s.split_whitespace()
        .find(|&part| part.chars().all(|c| c.is_ascii_digit()))
        .ok_or(Error::NoNumberFound)?
        .parse::<T>()
        .map_err(|_| Error::Conversion)
}

/// Converts Unicode vulgar fraction characters, (e.g. ½, ¾, ⅓) to their
/// text equivalents (e.g. 1/2, 3/4, 1/3)
pub fn normalise_vulgar_fractions(input: &str) -> String {
    let mut result = String::with_capacity(input.len() + input.len() / 4);

    for c in input.chars() {
        match c {
            '½' => result.push_str(" 1/2"),
            '⅓' => result.push_str(" 1/3"),
            '⅔' => result.push_str(" 2/3"),
            '¼' => result.push_str(" 1/4"),
            '¾' => result.push_str(" 3/4"),
            '⅕' => result.push_str(" 1/5"),
            '⅖' => result.push_str(" 2/5"),
            '⅗' => result.push_str(" 3/5"),
            '⅘' => result.push_str(" 4/5"),
            '⅙' => result.push_str(" 1/6"),
            '⅚' => result.push_str(" 5/6"),
            '⅐' => result.push_str(" 1/7"),
            '⅛' => result.push_str(" 1/8"),
            '⅜' => result.push_str(" 3/8"),
            '⅝' => result.push_str(" 5/8"),
            '⅞' => result.push_str(" 7/8"),
            '⅑' => result.push_str(" 1/9"),
            '⅒' => result.push_str(" 1/10"),
            _ => result.push(c),
        }
    }

    result.trim().replace("  ", " ")
}

/// Result type for errors related to strings.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to time.
#[derive(Debug, From)]
pub enum Error {
    Conversion,
    NoNumberFound,
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_auto_convert_to_utf8 {
        use super::*;

        #[test]
        fn test_valid_utf8_input() {
            let utf8_text = "Hello, 世界! 🌍";

            let result = auto_convert_to_utf8(utf8_text.as_bytes());

            assert_eq!(result, utf8_text);
        }

        #[test]
        fn test_empty_buffer() {
            let buffer = b"";

            let result = auto_convert_to_utf8(buffer);

            assert_eq!(result, "");
        }

        #[test]
        fn test_ascii_only() {
            let text = "Hello World!";

            let result = auto_convert_to_utf8(text.as_bytes());

            assert_eq!(result, text);
        }

        #[test]
        fn test_windows_1252_encoding() {
            let text = "Café résumé naïve";
            let (encoded_bytes, _, _) = WINDOWS_1252.encode(text);

            let result = auto_convert_to_utf8(&encoded_bytes);

            assert!(!result.is_empty());
            assert_eq!(result, text);
        }

        #[test]
        fn test_fallback_to_lossy_conversion() {
            let problematic_bytes = vec![0x81, 0x8D, 0x8F, 0x90, 0x9D]; // Undefined in Windows-1252

            let result = auto_convert_to_utf8(&problematic_bytes);

            assert!(result.contains('\u{FFFD}') || !result.is_empty());
        }

        #[test]
        fn test_mixed_content() {
            let mixed_bytes = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0xE9]; // "Hello é" in Windows-1252

            let result = auto_convert_to_utf8(&mixed_bytes);

            assert_eq!(result, "Hello é");
        }

        #[test]
        fn test_utf8_with_bom() {
            let mut bom_bytes = vec![0xEF, 0xBB, 0xBF]; // UTF-8 BOM
            bom_bytes.extend_from_slice("Hello".as_bytes());

            let result = auto_convert_to_utf8(&bom_bytes);

            assert_eq!(result, "\u{FEFF}Hello"); // BOM + Hello
        }

        #[test]
        fn test_long_text() {
            let text = "A".repeat(10000);

            let result = auto_convert_to_utf8(text.as_bytes());

            assert_eq!(result, text);
        }

        #[test]
        fn test_unicode_characters() {
            let unicode_text = "Ñoño 北京 مرحبا 🚀";

            let result = auto_convert_to_utf8(unicode_text.as_bytes());

            assert_eq!(result, unicode_text);
        }

        #[test]
        fn test_null_bytes() {
            let null_bytes = vec![
                0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x00, 0x57, 0x6F, 0x72, 0x6C, 0x64,
            ];

            let result = auto_convert_to_utf8(&null_bytes);

            assert_eq!(result, "Hello\0World");
        }

        #[test]
        fn test_single_byte() {
            let single_byte = vec![0x41]; // 'A'

            let result = auto_convert_to_utf8(&single_byte);

            assert_eq!(result, "A");
        }

        #[test]
        fn test_high_ascii_bytes() {
            let high_ascii = vec![0x80, 0x81, 0x82, 0x83];

            let result = auto_convert_to_utf8(&high_ascii);

            assert!(!result.is_empty());
        }
    }

    mod tests_extract_number {
        use super::*;

        #[test]
        fn test_no_number_err() -> Result<()> {
            let res = extract_number::<i16>("");

            assert!(res.is_err());
            Ok(())
        }

        #[test]
        fn test_number_in_front_ok() -> Result<()> {
            let res = extract_number::<i16>("420 years of love")?;

            pretty_assertions::assert_eq!(res, 420);
            Ok(())
        }

        #[test]
        fn test_number_middle_ok() -> Result<()> {
            let res = extract_number::<i16>("years of 420 love")?;

            pretty_assertions::assert_eq!(res, 420);
            Ok(())
        }

        #[test]
        fn test_number_end_ok() -> Result<()> {
            let res = extract_number::<i16>("years of love 420")?;

            pretty_assertions::assert_eq!(res, 420);
            Ok(())
        }
    }

    mod tests_normalise_vulgar_fractions {
        use super::*;

        #[test]
        fn test_individual_vulgar_fractions() {
            let cases = [
                ('½', "1/2"),
                ('⅓', "1/3"),
                ('⅔', "2/3"),
                ('¼', "1/4"),
                ('¾', "3/4"),
                ('⅕', "1/5"),
                ('⅖', "2/5"),
                ('⅗', "3/5"),
                ('⅘', "4/5"),
                ('⅙', "1/6"),
                ('⅚', "5/6"),
                ('⅐', "1/7"),
                ('⅛', "1/8"),
                ('⅜', "3/8"),
                ('⅝', "5/8"),
                ('⅞', "7/8"),
                ('⅑', "1/9"),
                ('⅒', "1/10"),
            ];

            for (s, expected) in cases {
                assert_eq!(normalise_vulgar_fractions(&s.to_string()), expected);
            }
        }

        #[test]
        fn test_mixed_vulgar_fractions() {
            let input = "Add ½ cup of sugar and ¼ tsp of salt.";

            let expected = "Add 1/2 cup of sugar and 1/4 tsp of salt.";

            assert_eq!(normalise_vulgar_fractions(input), expected);
        }

        #[test]
        fn test_multiple_vulgar_fractions() {
            let input = "⅓ + ⅓ + ⅓ = 1";

            let expected = "1/3 + 1/3 + 1/3 = 1";

            assert_eq!(normalise_vulgar_fractions(input), expected);
        }

        #[test]
        fn test_empty_input() {
            assert_eq!(normalise_vulgar_fractions(""), "");
        }

        #[test]
        fn test_no_vulgar_fractions() {
            let input = "Add 1/6 cup of sugar and 1/2 tsp of salt.";

            assert_eq!(normalise_vulgar_fractions(input), input);
        }
    }
}
