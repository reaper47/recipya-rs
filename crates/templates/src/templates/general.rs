use maud::{DOCTYPE, Markup, PreEscaped, html};

use models::paper::PaperSize;
use support::numbers::fmt_without_trailing_zeroes;

use crate::settings::SEARCH_INPUT_JS;

use super::layouts;

/// Renders the paper sizes table.
pub fn render_paper_sizes_table(data: &[(usize, &str, &PaperSize)]) -> Markup {
    html! {
        div class="card bg-base-100 shadow-sm p-2 min-w-[50vw]" {
            div class="card-body" {
                h3 class="mb-1 grid grid-flow-col" {
                    label class="input input-sm" {
                        svg class="h-[1em] opacity-50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" {
                            g stroke-linejoin="round" stroke-linecap="round" stroke-width="2.5" fill="none" stroke="currentColor" {
                                circle cx="11" cy="11" r="8" {}
                                path d="m21 21-4.3-4.3" {}
                            }
                        }
                        input type="search" placeholder="Search a paper size" _=(PreEscaped(SEARCH_INPUT_JS));
                    }
                }
                div class="overflow-auto h-[50vh]" {
                    table class="table table-zebra table-sm" {
                        thead {
                            tr class="text-center" {
                                th class="py-1 text-left" {}
                                th class="py-1" { "Name" }
                                th class="py-1 text-left" { "Category" }
                                th class="py-1" { "Size (mm)" }
                                th class="py-1" { "Size (inches)" }
                            }
                        }
                        tbody #search-results {
                            @for (idx, category, paper) in data {
                                tr {
                                    td class="py-1" { (idx) }
                                    td class="py-1" { (paper.name) }
                                    td class="py-1 text-center select-none" { (category) }
                                    td class="py-1 text-center" {
                                        (format!(
                                            "{} x {}",
                                            fmt_without_trailing_zeroes(paper.width_mm),
                                            fmt_without_trailing_zeroes(paper.height_mm),
                                        ))
                                    }
                                    td class="py-1 text-center" {
                                        (format!("{:.2} x {:.2}", paper.width_in, paper.height_in))
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div class="card-actions justify-end" {
                button class="btn btn-sm" onclick="this.closest('dialog').close()" { "Close" }
            }
        }
    }
}

/// Renders content for the print view of a component.
pub fn render_print_view(content: &Markup) -> Markup {
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
