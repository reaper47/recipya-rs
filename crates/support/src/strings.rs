use std::{
    borrow::Cow,
    str::{FromStr, from_utf8},
};

use derive_more::From;
use encoding_rs::{ISO_8859_15, WINDOWS_1252};
use itertools::Itertools;
use rapidfuzz::distance::levenshtein;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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

/// Calculates the number of seconds from a string in the format `hh:mm:ss`.
pub fn calc_seconds_from_parts(s: &str) -> i32 {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() == 3 {
        let [hours, minutes, seconds]: [i32; 3] = parts
            .iter()
            .map(|&part| part.parse::<i32>().unwrap_or(0))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or([0, 0, 0]);

        hours * 60 * 60 + minutes * 60 + seconds
    } else {
        0
    }
}

/// Extracts a number from a string, if it exists.
pub fn extract_number<T>(s: &str) -> Result<T>
where
    T: FromStr + std::fmt::Debug,
{
    s.replace('-', " ")
        .split_whitespace()
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

/// Inserts a space after a leading number in a string.
pub fn insert_space_after_leading_number(s: &str) -> Cow<'_, str> {
    let Some(boundary) = s
        .char_indices()
        .take_while(|(_, c)| c.is_ascii_digit())
        .last()
        .map(|(i, c)| i + c.len_utf8())
    else {
        return Cow::Borrowed(s);
    };

    if s[boundary..]
        .chars()
        .next()
        .is_none_or(|c| !c.is_alphabetic())
    {
        return Cow::Borrowed(s);
    }

    let mut result = String::with_capacity(s.len() + 1);
    result.push_str(&s[..boundary]);
    result.push(' ');
    result.push_str(&s[boundary..]);
    Cow::Owned(result)
}

/// Find the start/end byte indexes of all target elements in each text.
///
/// The Levenshtein distance is used to find the indexes.
/// Returns one `Vec<(start, end)>` per input text in the same order as `texts`.
pub fn find_indexes(texts: &[&str], targets: &[&str]) -> Result<Vec<Vec<(i32, i32)>>> {
    if texts.is_empty() || targets.is_empty() {
        return Err(Error::InvalidInput);
    }

    Ok(texts
        .par_iter()
        .map(|text| calculate_levenshtein_disance(text, targets))
        .collect::<Vec<_>>())
}

fn calculate_levenshtein_disance(haystack_original: &str, needles: &[&str]) -> Vec<(i32, i32)> {
    let mut results = Vec::new();
    let haystack_original = haystack_original.to_lowercase();
    let haystack_parts = haystack_original.split_whitespace().collect_vec();

    for needle in needles.iter().map(|s| s.to_lowercase()) {
        let len_needle = needle.chars().count();

        let exact_matches = haystack_original
            .match_indices(&needle)
            .filter(|(idx, _)| {
                let before = haystack_original[..*idx].chars().last();
                let after = haystack_original[*idx + len_needle..].chars().next();
                !before.map_or(false, |c| c.is_alphanumeric() || c == '_')
                    && !after.map_or(false, |c| c.is_alphanumeric() || c == '_')
            })
            .map(|(idx, _)| {
                (
                    i32::try_from(idx).unwrap_or_default(),
                    i32::try_from(idx + len_needle).unwrap_or_default(),
                )
            })
            .collect_vec();

        if !exact_matches.is_empty() {
            results.extend(exact_matches);
            continue;
        }

        for hay in &haystack_parts {
            let distances = needle
                .split_whitespace()
                .map(|s| levenshtein::distance(s.chars(), hay.chars()))
                .filter(|d| d < &2)
                .collect_vec()
                .iter()
                .fold(Vec::new(), |mut acc, &curr| {
                    match acc.last() {
                        Some(&prev) if curr < prev + 3 => {}
                        _ => acc.push(curr),
                    }
                    acc
                });

            if !distances.is_empty() {
                let indexes = haystack_original
                    .match_indices(&needle)
                    .filter(|(idx, _)| {
                        let before = haystack_original[..*idx].chars().last();
                        let after = haystack_original[*idx + len_needle..].chars().next();
                        !before.map_or(false, |c| c.is_alphanumeric() || c == '_')
                            && !after.map_or(false, |c| c.is_alphanumeric() || c == '_')
                    })
                    .map(|(idx, _)| {
                        (
                            i32::try_from(idx).unwrap_or_default(),
                            i32::try_from(idx + len_needle).unwrap_or_default(),
                        )
                    })
                    .collect_vec();

                // Removed partial word fallback to avoid matching partial ingredients like "baking" from "baking soda"

                results.extend(indexes);
            }
        }
    }

    results.iter().unique().copied().collect()
}

/// Result type for errors related to strings.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to time.
#[derive(Debug, From)]
pub enum Error {
    Conversion,
    InvalidInput,
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
            bom_bytes.extend(b"Hello");

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
        fn test_no_number_err() {
            let res = extract_number::<i16>("");

            assert!(res.is_err());
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

    mod tests_insert_space_after_leading_number {
        use super::*;

        #[test]
        fn test_insert_space_after_leading_number() {
            let input = "1eggs whole fresh, beaten";

            let got = insert_space_after_leading_number(input);

            assert_eq!(got, "1 eggs whole fresh, beaten");
        }
    }

    mod tests_find_indexes {
        use super::*;

        fn recipe1_ingredients<'a>() -> Vec<&'a str> {
            vec![
                "butter",
                "white sugar",
                "brown sugar",
                "eggs",
                "vanilla extract",
                "baking soda",
                "hot water",
                "salt",
                "all-purpose flour",
                "semisweet chocolate chips",
                "chopped walnuts",
            ]
        }

        #[test]
        fn test_empty_text() {
            let got = find_indexes(&[], &["hello"]);

            assert!(matches!(got, Err(Error::InvalidInput)));
        }

        #[test]
        fn test_empty_targets() {
            let got = find_indexes(&["hello"], &[]);

            assert!(matches!(got, Err(Error::InvalidInput)));
        }

        #[test]
        fn test_no_matches_ok() -> Result<()> {
            let got = find_indexes(
                &["Store in an airtight container or serve immediately and enjoy!"],
                recipe1_ingredients().as_slice(),
            )?;

            assert_eq!(got, vec![vec![]]);
            Ok(())
        }

        #[test]
        fn test_multiple_matches_recipe1() -> Result<()> {
            let got = find_indexes(
                &[
                    "Gather your ingredients, making sure your butter is softened, and your eggs are room temperature.",
                    "Preheat the oven to 350 degrees F (175 degrees C). Beat butter, white sugar, and brown sugar together in a large bowl with an electric mixer until smooth and creamy.",
                    "Beat in eggs, one at a time, then stir in vanilla.",
                    "Dissolve baking soda in hot water; add to batter along with salt and mix until combined.",
                    "Stir in flour, chocolate chips, and walnuts until a soft dough forms.",
                    "Drop rounded spoonfuls of cookie dough 2 inches apart onto ungreased baking sheets.",
                    "Bake in the preheated oven until edges are lightly browned, about 10 minutes.",
                    "Cool on the baking sheets briefly before transferring to a wire rack to cool completely.",
                    "Store in an airtight container or serve immediately and enjoy!",
                ],
                recipe1_ingredients().as_slice(),
            )?;

            pretty_assertions::assert_eq!(
                got,
                vec![
                    vec![
                        (42, 48), // butter
                        (71, 75), // large eggs
                    ],
                    vec![
                        (56, 62), // butter
                        (64, 75), // white sugar
                        (81, 92), // brown sugar
                    ],
                    vec![
                        (8, 12), // eggs (vanilla is NOT matched since it's only a partial ingredient from "vanilla extract")
                    ],
                    vec![
                        (9, 20),  // baking soda
                        (24, 33), // hot water
                        (60, 64), // salt
                    ],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                ]
            );
            Ok(())
        }

        /// Regression test for issue #322:
        /// The algorithm was matching substrings inside words (e.g., "a" inside "another",
        /// "or" inside "form", "and" inside "hands"), producing broken HTML.
        /// This test ensures word-boundary matching is enforced.
        #[test]
        fn test_word_boundary_matching_issue_322() -> Result<()> {
            let ingredients = vec!["chicken", "oil", "a", "or", "and"];
            let texts = vec![
                "In another bowl, combine chicken with remaining ingredients. \
                 Lightly oil hands and form 4 patties. Oil patties and season \
                 surface. Barbecue 12 to 15 minutes or until chicken is cooked.",
            ];
            let got = find_indexes(&texts, &ingredients)?;
            // "a" should NOT match inside "another", "remaining", "hands", "patties",
            // "season", "surface", "barbecue"
            // "or" should NOT match inside "form"
            // "and" should NOT match inside "hands"
            assert_eq!(
                got,
                vec![vec![
                    (25, 32),   // chicken
                    (166, 173), // chicken (second occurrence)
                    (69, 72),   // oil
                    (99, 102),  // Oil (standalone "Oil")
                    (157, 159), // or (standalone "or")
                    (79, 82),   // and (standalone "and")
                    (111, 114), // and (standalone "and")
                ]]
            );
            Ok(())
        }
    }
}
