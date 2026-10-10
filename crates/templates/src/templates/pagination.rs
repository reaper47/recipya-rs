use std::cmp;

use maud::{Markup, html};

use l10n::Messages;
use models::data::{PageSlot, PaginationData};

use crate::templates::helpers::{Replace, inject_markup_in_message};

/// Renders the pagination strip.
pub(super) fn pagination(p: &PaginationData, messages: &Messages) -> Markup {
    let oob_attr = if p.htmx.is_swap {
        Some("innerHTML:#pagination-anchor")
    } else {
        None
    };

    if p.is_hidden {
        html! {
            footer id=(p.id) .hidden hx-swap-oob=[oob_attr] {}
        }
    } else {
        html! {
            footer id=(p.id)
                class={
                    "footer footer-center bg-base-200 p-2 gap-2 md:pb-2 mt-auto shrink-0"
                    @if let Some(css) = p.additional_css.as_ref() { " " (css) }
                }
                style="grid-auto-flow: row;"
                onload=(format!("updateAddCookbookUrl({})", p.selected))
                hx-swap-oob=[oob_attr] {
                div class="join gap-0" {
                    // Previous page button
                    @let prev_page = messages.pagination_prev_page();
                    @if p.selected == 1 {
                        button class="join-item btn btn-disabled btn-xs md:btn-sm w-8 md:w-12" title=(prev_page) aria-label=(prev_page) { "‹" }
                    } @else {
                        @let prev_url = format!("{}?page={}{}", p.url, p.prev, p.url_queries);
                        button class="join-item btn btn-xs md:btn-sm w-8 md:w-12"
                            title=(prev_page)
                            aria-label=(prev_page)
                            hx-get=(prev_url)
                            hx-target=(p.htmx.target)
                            hx-trigger="mousedown"
                            hx-push-url=(prev_url)
                            hx-swap="innerHTML show:window:top transition:true" { "‹" }
                    }

                    @for slot in p.slots.iter() {
                        @match slot {
                            PageSlot::Page(page_num) => {
                                @if p.selected == *page_num {
                                    button class="join-item btn btn-active btn-xs md:btn-sm w-8 md:w-12"
                                        aria-current="page"
                                        aria-label=(messages.pagination_aria_label(page_num)) { (page_num) }
                                } @else {
                                    @let goto_page = messages.pagination_goto_page(page_num);
                                    @let get_page = format!("{}?page={page_num}{}", p.url, p.url_queries);
                                    button class="join-item btn btn-xs md:btn-sm w-8 md:w-12"
                                        title=(goto_page)
                                        aria-label=(goto_page)
                                        hx-get=(get_page)
                                        hx-target=(p.htmx.target)
                                        hx-trigger="click"
                                        hx-push-url=(get_page)
                                        hx-swap="innerHTML show:window:top transition:true" { (page_num) }
                                }
                            }
                            PageSlot::Ellipsis => {
                                button class="join-item btn btn-disabled btn-xs md:btn-sm w-8 md:w-12" aria-hidden="true" {
                                    "⋯"
                                }
                            }
                        }
                    }

                    // Next button
                    @let next_page = messages.pagination_next_page();
                    @if p.selected == p.num_pages {
                        button class="join-item btn btn-disabled btn-xs md:btn-sm w-8 md:w-12"
                            title=(next_page)
                            aria-label=(next_page) { "›" }
                    } @else {
                        @let next_url = format!("{}?page={}{}", p.url, p.next, p.url_queries);
                        button class="join-item btn btn-xs md:btn-sm w-8 md:w-12"
                            title=(next_page)
                            aria-label=(next_page)
                            hx-get=(next_url)
                            hx-target=(p.htmx.target)
                            hx-trigger="mousedown"
                            hx-push-url=(next_url)
                            hx-swap="innerHTML show:window:top transition:true" { "›" }
                    }
                }

                // Hide the count row when there are no results
                @if p.num_results > 0 {
                    div class="text-center" {
                        p class="text-xs md:text-sm" {
                            (render_summary(p, messages))
                        }
                    }
                }
            }
        }
    }
}

fn render_summary(p: &PaginationData, messages: &Messages) -> Markup {
    let from_num = calc_start_result(p.selected, p.results_per_page);
    let from = messages.count(from_num);
    let from_markup = html! {
        span class="font-semibold text-base-content" {
            (from)
        }
    };

    let to_num = calc_end_result(p.selected, p.results_per_page, p.num_results);
    let to = messages.count(to_num);
    let to_markup = html! {
        span class="font-semibold text-base-content" {
            (to)
        }
    };

    let total = messages.count(p.num_results);
    let total_markup = html! {
        span #search-count .font-medium {
            (total)
        }
    };

    let summary = messages.pagination_summary(
        from_num.to_string(),
        to_num.to_string(),
        p.num_results.to_string(),
        p.num_results.to_string(),
    );

    inject_markup_in_message(
        &summary,
        &[
            (&from, from_markup),
            (&to, to_markup),
            (&total, total_markup),
        ],
        &Replace::All,
    )
}

const fn calc_start_result(page: u64, per_page: u64) -> u64 {
    (page - 1) * per_page + 1
}

fn calc_end_result(page: u64, per_page: u64, total: u64) -> u64 {
    cmp::min(page * per_page, total)
}
