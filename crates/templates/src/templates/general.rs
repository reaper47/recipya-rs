use maud::{DOCTYPE, Markup, html};

use super::layouts;

/// Renders content for the print view of a component.
pub fn render_print_view(content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { "Print View" }
            }
            body class="p-8" {
                (content)
                script {
                    "window.onload = function() { window.print(); window.close(); }"
                }
            }
        }
    }
}

/// Renders a share link component.
pub fn share_link(url: &str) -> Markup {
    html! {
        div class="grid grid-flow-col gap-2" {
            label {
                input type="url" value=(url) .input readonly="readonly";
            }
            button #copy-button .btn.btn-neutral title="Copy to clipboard" onClick=(format!("copyToClipboard('{url}')")) { "Copy" }
        }
    }
}

/// Renders a simple page that displays a message.
pub fn simple(title: &str, content: &str) -> Markup {
    layouts::auth(
        title,
        &html! {
            div class="card w-80 sm:w-96 bg-base-100 shadow-xl" {
                div class="card-body" {
                    h2 class="card-title underline self-center" {
                        (title)
                    }
                    p {
                        (content)
                    }
                    div class="card-actions justify-end" {
                        a href="/" class="btn btn-primary btn-block btn-sm" {
                            "Back Home"
                        }
                    }
                }
            }
        },
    )
}
