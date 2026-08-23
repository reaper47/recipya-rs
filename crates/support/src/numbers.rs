/// Converts a floating-point number to an i16 safely by clamping it to the i16 range.
pub fn float_to_i16_safe<F: Into<f64>>(n: F) -> i16 {
    let clamped = n
        .into()
        .round()
        .clamp(f64::from(i16::MIN), f64::from(i16::MAX));
    #[allow(clippy::cast_possible_truncation)]
    {
        clamped as i16
    }
}

/// Formats a floating-point number without the trailing zeroes.
pub fn fmt_without_trailing_zeroes(v: f64) -> String {
    if v.fract() == 0.0 {
        format!("{v:.0}")
    } else {
        format!("{v}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fmt_without_trailing_zeroes {
        use super::*;

        #[test]
        fn test_fraction_with_trailing_zeroes() {
            let got = fmt_without_trailing_zeroes(216.0000);

            assert_eq!(got, "216");
        }

        #[test]
        fn test_fraction_without_trailing_zeroes() {
            let got = fmt_without_trailing_zeroes(216.0001);

            assert_eq!(got, "216.0001");
        }
    }
}
