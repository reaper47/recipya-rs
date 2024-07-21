use maud::{DOCTYPE, html, Markup};

use crate::web;

pub fn auth(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" class="h-full bg-indigo-100 dark:bg-gray-800" {
            (web::templates::core::head(title))
            body .h-full.grid.place-content-center {
                (content)
                (web::templates::core::toast())
            }
        }
    }
}
