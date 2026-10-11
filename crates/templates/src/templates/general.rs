use l10n::Messages;
use maud::{DOCTYPE, Markup, PreEscaped, html};

use models::paper::PaperSize;

use super::layouts;
use crate::settings::SEARCH_INPUT_JS;

/// Renders the paper sizes table.
pub fn render_paper_sizes_table(data: &[(usize, &str, &PaperSize)], messages: &Messages) -> Markup {
    html! {
        div class="card bg-base-100 shadow-sm p-2 min-w-[50vw]" {
            div .card-body {
                h3 class="mb-1 grid grid-flow-col" {
                    label class="input input-sm" {
                        svg class="h-[1em] opacity-50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" {
                            g stroke-linejoin="round" stroke-linecap="round" stroke-width="2.5" fill="none" stroke="currentColor" {
                                circle cx="11" cy="11" r="8" {}
                                path d="m21 21-4.3-4.3" {}
                            }
                        }
                        input type="search" placeholder=(messages.paper_sizes_search_placeholder()) _=(PreEscaped(SEARCH_INPUT_JS));
                    }
                }
                div class="overflow-auto h-[50vh]" {
                    table class="table table-zebra table-sm" {
                        thead {
                            tr .text-center {
                                th class="py-1 text-left" {}
                                th .py-1 {
                                    (messages.column_name())
                                }
                                th class="py-1 text-left" {
                                    (messages.column_category())
                                }
                                th .py-1 {
                                    (messages.paper_sizes_table_size_mm())
                                }
                                th .py-1 {
                                    (messages.paper_sizes_table_size_in())
                                }
                            }
                        }
                        tbody #search-results {
                            @for (idx, category, paper) in data {
                                tr {
                                    td .py-1 {
                                        (idx)
                                    }
                                    td .py-1 {
                                        (paper.name)
                                    }
                                    td class="py-1 text-center select-none" {
                                        (category)
                                    }
                                    td class="py-1 text-center" {
                                        (messages.paper_size_dimensions(paper.width_mm, paper.height_mm))
                                    }
                                    td class="py-1 text-center" {
                                        (messages.paper_size_dimensions(paper.width_in, paper.height_in))
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div class="card-actions justify-end" {
                button class="btn btn-sm" onclick="this.closest('dialog').close()" {
                    (messages.action_close())
                }
            }
        }
    }
}

/// Renders content for the print view of a component.
pub fn render_print_view(content: &Markup, messages: &Messages) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title {
                    (messages.print_view_title())
                }
            }
            body .p-8 {
                (content)
                script {
                    "window.onload = function() { window.print(); window.close(); }"
                }
            }
        }
    }
}

/// Renders a share link component.
pub fn share_link(url: &str, messages: &Messages) -> Markup {
    html! {
        div class="grid grid-flow-col gap-2" {
            label {
                input type="url" .input value=(url) readonly;
            }
            button #copy-button class="btn btn-neutral" title=(messages.share_link_copy_clipboard()) onClick=(format!("copyToClipboard('{url}')")) {
                (messages.action_copy())
            }
        }
    }
}

/// Renders a simple page that displays a message.
pub fn simple(title: &str, content: &str, messages: &Messages) -> Markup {
    layouts::auth(
        title,
        &html! {
            div class="card w-80 sm:w-96 bg-base-100 shadow-xl" {
                div .card-body {
                    h2 class="card-title underline self-center" {
                        (title)
                    }
                    p {
                        (content)
                    }
                    div class="card-actions justify-end" {
                        a href="/" class="btn btn-primary btn-block btn-sm" {
                            (messages.simple_page_back_home())
                        }
                    }
                }
            }
        },
        messages,
    )
}
