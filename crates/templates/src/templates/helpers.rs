use maud::{Markup, html};

/// Truncates a string to the specified number of characters and appends an ellipsis (`…`) if truncated.
pub(super) fn cut_string(s: &str, num_chars: usize) -> String {
    if s.chars().count() <= num_chars {
        return s.to_string();
    }

    let truncated: String = s.chars().take(num_chars).collect();
    format!("{truncated}…")
}

/// Injects HTML markup into a Fluent message.
pub(super) fn inject_markup_in_message(text: &str, slots: &[(&str, Markup)]) -> Markup {
    let first = slots
        .iter()
        .enumerate()
        .filter_map(|(i, (marker, _))| text.find(marker).map(|pos| (pos, i)))
        .min();

    if let Some((pos, i)) = first {
        let (marker, inner) = &slots[i];
        html! {
            (&text[..pos])
            (inner)
            (inject_markup_in_message(&text[pos + marker.len()..], slots))
        }
    } else {
        html! {
            (text)
        }
    }
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
