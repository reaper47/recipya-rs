use std::iter::Product;
use std::ops::Mul;

/// Truncates a string to the specified number of characters and appends an ellipsis (`…`) if truncated.
pub(super) fn cut_string(s: &str, num_chars: usize) -> String {
    if s.chars().count() <= num_chars {
        return s.to_string();
    }

    let truncated: String = s.chars().take(num_chars).collect();
    format!("{truncated}…")
}

/// Multiplies all values in the iterable and returns the product.
pub(super) fn mul_all<T>(vals: impl IntoIterator<Item=T>) -> T
where
    T: Mul<Output=T> + Product + From<u8>,
{
    vals.into_iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cut_string_num_chars_in_range_ok() {
        let got = cut_string("Let's go Canada!", 5);

        assert_eq!(got, "Let's…");
    }

    #[test]
    fn test_cut_string_num_chars_out_of_range_ok() {
        let got = cut_string("Let's go Canada!", 100);

        assert_eq!(got, "Let's go Canada!");
    }

    #[test]
    fn test_mul_all_ok() {
        let got = mul_all(vec![1u8, 2u8, 3u8, 4u8]);

        pretty_assertions::assert_eq!(got, 24u8);
    }
}
