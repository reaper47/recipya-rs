use std::cmp;

use maud::{Markup, html};

use models::data::{PageSlot, PaginationData};

/// Renders the pagination strip.
pub(super) fn pagination(p: &PaginationData) -> Markup {
    html! {
        footer #pagination class={
                "footer footer-center bg-base-200 pb-12 p-2 md:pb-2 text-base-content gap-2"
                @if p.is_hidden { " hidden" }
            }
            style="grid-auto-flow: row;"
            onload=(format!("updateAddCookbookUrl({})", p.selected))
            hx-swap-oob=[if p.htmx.is_swap { Some("outerHTML:#pagination") } else { None }] {
            div class="join gap-0" {
                // Previous page button
                @if p.selected == 1 {
                    button class="join-item btn btn-disabled w-12" title="Previous page" aria-label="Previous page" { "‹" }
                } @else {
                    @let prev_url = format!("{}?page={}{}", p.url, p.prev, p.url_queries);

                    button
                        class="join-item btn w-12"
                        title="Previous page"
                        aria-label="Previous page"
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
                                button class="join-item btn btn-active w-12"
                                    aria-current="page"
                                    aria-label=(format!("Page {}, current page", page_num)) { (page_num) }
                            } @else {
                                @let goto_page = format!("Go to page {}", page_num);
                                @let get_page = format!("{}?page={}{}", p.url, page_num, p.url_queries);

                                button class="join-item btn w-12"
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
                            button class="join-item btn btn-disabled w-12"
                                aria-hidden="true" { "⋯" }
                        }
                    }
                }

                // Next button
                @if p.selected == p.num_pages {
                   button class="join-item btn btn-disabled w-12"
                        title="Next page"
                        aria-label="Next page" { "›" }
                } @else {
                    @let next_url = format!("{}?page={}{}", p.url, p.next, p.url_queries);

                    button class="join-item btn w-12"
                        title="Next page"
                        aria-label="Next page"
                        hx-get=(next_url)
                        hx-target=(p.htmx.target)
                        hx-trigger="mousedown"
                        hx-push-url=(next_url)
                        hx-swap="innerHTML show:window:top transition:true" { "›" }
                }
            }

            div class="text-center mt-2" {
                p class="text-sm text-base-content/70" {
                    "Showing "
                    span class="font-semibold text-base-content" {
                        (format_number(calc_start_result(p.selected, p.results_per_page)))
                    }
                    "-"
                    span class="font-semibold text-base-content" {
                        (format_number(calc_end_result(p.selected, p.results_per_page, p.num_results)))
                    }
                    " of " span #search-count class="font-medium" {
                        (format_number(p.num_results))
                    }
                    " results"
                }
            }
        }
    }
}

fn format_number(num: u64) -> String {
    if num < 1000 {
        num.to_string()
    } else {
        let mut result = String::new();
        let chars = num.to_string().chars().collect::<Vec<_>>();

        for (i, c) in chars.iter().enumerate() {
            if i > 0 && (chars.len() - i) % 3 == 0 {
                result.push(',');
            }
            result.push(*c);
        }

        result
    }
}

fn calc_start_result(page: u64, per_page: u64) -> u64 {
    (page - 1) * per_page + 1
}

fn calc_end_result(page: u64, per_page: u64, total: u64) -> u64 {
    cmp::min(page * per_page, total)
}
