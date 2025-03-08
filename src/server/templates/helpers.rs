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
pub(super) fn mul_all<T>(vals: impl IntoIterator<Item = T>) -> T
where
    T: Mul<Output = T> + Product + From<u8>,
{
    vals.into_iter().product()
}
