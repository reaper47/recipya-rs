use maud::{html, Markup, DOCTYPE};

use crate::server::templates::core::{head, toast};

/// Renders the authentication layout template.
pub fn auth(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" class="h-full bg-indigo-100 dark:bg-gray-800" {
            (head(title))
            body .h-full.grid.place-content-center {
                (content)
            }
            (toast())
        }
    }
}
