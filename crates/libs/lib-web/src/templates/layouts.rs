use maud::{html, Markup, DOCTYPE};

pub fn auth(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" class="h-full bg-indigo-100 dark:bg-gray-800" {
            (super::core::head(title))
            body .h-full.grid.place-content-center {
                (content)
                (super::core::toast())
            }
        }
    }
}
