use crate::server::templates::data::PaginationData;
use crate::server::templates::helpers::mul_all;
use maud::{html, Markup};

/// Renders the pagination strip.
pub(super) fn pagination(p: &PaginationData) -> Markup {
    html! {
        footer #pagination class={
                "footer footer-center bg-base-200 pb-12 p-2 md:pb-2 text-base-content gap-2"
                @if p.is_hidden { " hidden" }
            }
            onload=(format!("updateAddCookbookUrl({})", p.selected))
            {@if p.htmx.is_swap { r##"hx-swap-oob="outerHTML:#pagination""##}} {
            div class="join gap-0" {
                @if p.selected == 1 {
                    button class="join-item btn btn-disabled" { "«" }
                } @else {
                    button
                        class="join-item btn"
                        hx-get=(format!("{}?page={}{}", p.url, p.prev, p.url_queries))
                        hx-target=(p.htmx.target)
                        hx-trigger="mousedown"
                        hx-push-url=(format!("{}?page={}", p.url, p.prev))
                        hx-swap="innerHTML show:window:top transition:true" { "«" }
                }

                // Left Section
                @for (i, l) in p.left.iter().enumerate() {
                    @if p.selected == *l {
                        button aria-current="page" class="join-item btn btn-active" { (l) }
                    } @else {
                        button class=(
                            if p.left.len() > 3 {
                                let mut class = String::from("join-item btn");
                                if i > 3 {
                                    class.push_str(" hidden sm:block");
                                }
                                class
                            } else {
                                let mut class = String::from("join-item btn");
                                if (i == 1 && p.left.len() != 2) || i > 2 {
                                    class.push_str(" hidden sm:block");
                                }
                                class
                            })
                            hx-get=(format!("{}?page={}{}", p.url, l, p.url_queries))
                            hx-target=(p.htmx.target)
                            hx-trigger="mousedown"
                            hx-push-url=(format!("{}?page={}{}", p.url, l, p.url_queries))
                            hx-swap="innerHTML show:window:top transition:true" { (l) }
                    }
                }

                // Middle Section
                @if !p.middle.is_empty() {
                    button class="hidden sm:block join-item btn btn-disabled" { "..." }
                }
                @for (i, m) in p.middle.iter().enumerate() {
                    @if p.selected == *m {
                        button class="join-item btn btn-active"
                            aria-current="page"
                            hx-get=(format!("{}?page={}{}", p.url, m, p.url_queries))
                            hx-target=(p.htmx.target)
                            hx-trigger="mousedown"
                            hx-push-url=(format!("{}?page={}{}", p.url, m, p.url_queries))
                            hx-swap="innerHTML show:window:top transition:true" { (m) }
                    } @else {
                        button class={
                                "join-item btn"
                                @if i == 1 || i > 3 { " hidden sm:block" }
                            }
                            hx-get=(format!("{}?page={}{}", p.url, m, p.url_queries))
                            hx-target=(p.htmx.target)
                            hx-trigger="mousedown"
                            hx-push-url=(format!("{}?page={}{}", p.url, m, p.url_queries))
                            hx-swap="innerHTML show:window:top transition:true" { (m) }
                    }
                }
                @if !p.middle.is_empty() {
                    button class="hidden sm:block join-item btn btn-disabled" { "..." }
                }

                // Right Section
                @if !p.right.is_empty() && p.middle.is_empty() {
                    button class="hidden sm:block join-item btn btn-disabled" { "..." }
                }
                @for (i, r) in p.right.iter().enumerate() {
                    @if p.selected == *r {
                        button class="join-item btn btn-active"
                            aria-current="page"
                            hx-get=(format!("{}?page={}{}", p.url, r, p.url_queries))
                            hx-target=(p.htmx.target)
                            hx-trigger="mousedown"
                            hx-push-url=(format!("{}?page={}{}", p.url, r, p.url_queries))
                            hx-swap="innerHTML show:window:top transition:true" { (r) }
                    } @else {
                        button class=(
                            if p.right.len() > 3 {
                                let mut class = String::from("join-item btn");
                                if i == 0 || i > 4 {
                                    class.push_str(" hidden sm:block");
                                }
                                class
                            } else {
                                let mut class = String::from("join-item btn");
                                if i == 1 || i > 2 {
                                    class.push_str(" hidden sm:block");
                                }
                                class
                            })
                            hx-get=(format!("{}?page={}{}", p.url, r, p.url_queries))
                            hx-target=(p.htmx.target)
                            hx-trigger="mousedown"
                            hx-push-url=(format!("{}?page={}{}", p.url, r, p.url_queries))
                            hx-swap="innerHTML show:window:top transition:true" { (r) }
                    }
                }
                @if p.selected == p.num_pages {
                    button class="join-item btn btn-disabled" { "»" }
                } @else {
                    button class="join-item btn"
                        hx-get=(format!("{}?page={}{}", p.url, p.next, p.url_queries))
                        hx-target=(p.htmx.target)
                        hx-trigger="mousedown"
                        hx-push-url=(format!("{}?page={}{}", p.url, p.next, p.url_queries))
                        hx-swap="innerHTML show:window:top transition:true" { "»" }
                }
            }
            div class="text-center" {
                p class="text-sm" {
                    "Showing"
                    span class="font-medium" {
                        @if p.selected == p.prev {
                            (format!("{}", p.selected))
                        } @else {
                            (format!("{}", mul_all(vec![p.selected - 1, p.results_per_page]) + 1))
                        }
                    }
                    "to"
                    span class="font-medium" {
                        @if p.selected == p.num_pages {
                            (p.num_results)
                        } @else {
                            (mul_all(vec![p.selected, p.results_per_page]))
                        }
                    }
                    "of" span #search-count class="font-medium" {
                        (p.num_results)
                    }
                    "results"
                }
            }
        }
    }
}
