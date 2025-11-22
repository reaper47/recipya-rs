/// Truncates a string to the specified number of characters and appends an ellipsis (`…`) if truncated.
pub(super) fn cut_string(s: &str, num_chars: usize) -> String {
    if s.chars().count() <= num_chars {
        return s.to_string();
    }

    let truncated: String = s.chars().take(num_chars).collect();
    format!("{truncated}…")
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
}
