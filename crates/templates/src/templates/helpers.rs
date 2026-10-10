use maud::{Markup, html};

/// Truncates a string to the specified number of characters and appends an ellipsis (`…`) if truncated.
pub(super) fn cut_string(s: &str, num_chars: usize) -> String {
    if s.chars().count() <= num_chars {
        return s.to_string();
    }

    let truncated: String = s.chars().take(num_chars).collect();
    format!("{truncated}…")
}

#[derive(PartialEq, Eq)]
pub(super) enum Replace {
    First,
    All,
}

enum Piece<'a> {
    Text(&'a str),
    Slot(&'a Markup),
}

/// Injects HTML markup into a Fluent message.
pub(super) fn inject_markup_in_message(
    text: &str,
    slots: &[(&str, Markup)],
    mode: Replace,
) -> Markup {
    let mut pieces = Vec::new();
    let mut rest = text;

    while let Some((pos, i)) = slots
        .iter()
        .enumerate()
        .filter(|(_, (marker, _))| !marker.is_empty())
        .filter_map(|(i, (marker, _))| rest.find(marker).map(|pos| (pos, i)))
        .min()
    {
        let (marker, inner) = &slots[i];
        pieces.push(Piece::Text(&rest[..pos]));
        pieces.push(Piece::Slot(inner));
        rest = &rest[pos + marker.len()..];

        if mode == Replace::First {
            break;
        }
    }
    pieces.push(Piece::Text(rest));

    html! {
        @for piece in &pieces {
            @match piece {
                Piece::Text(t) => (t),
                Piece::Slot(m) => (m),
            }
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
