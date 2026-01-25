use maud::{Markup, PreEscaped, html};

use models::data::SearchbarData;

use super::icons::{
    icon_bars_3_bottom_left, icon_heart, icon_information_circle, icon_magnifying_glass,
    icon_x_mark,
};

/// Renders the searchbar.
pub(super) fn searchbar(data: &SearchbarData) -> Markup {
    html! {
        div class="relative w-full" {
            label class="input input-sm flex justify-between px-0 gap-2 z-20 w-full" {
                button #search-shortcut type="button" class="pl-2" popovertarget="search-help" _="on click toggle .hidden on #search-help" {
                    (icon_information_circle(true))
                }
                input #search-recipes
                    type="search"
                    name="q"
                    placeholder="Search for recipes..."
                    autocomplete="off"
                    value=(data.term)
                    list="search-suggestions"
                    hx-get="/search-suggestions"
                    hx-trigger="keyup changed delay:200ms"
                    hx-target="#search-suggestions-menu"
                    hx-include="this"
                    hx-swap="innerHTML"
                    hx-push-url="false"
                    _=(PreEscaped("
                        on keyup
                            if event.target.value !== '' then
                                remove .md:block from #search-shortcut
                            else
                                add .md:block to #search-shortcut then
                                if (event.key is not 'Delete' and not event.key.startsWith('Arrow')) then
                                    send submit to closest <form/> then
                                end
                            end"));

                button type="submit" class="px-2 btn btn-sm btn-primary" {
                    (icon_magnifying_glass())
                    span class="sr-only" { "Search" }
                }
            }

            ul #search-suggestions-menu
                class="hidden grid absolute top-full left-1/2 -translate-x-1/2 mt-1 menu bg-base-300 rounded-box shadow-lg z-50 max-h-60 overflow-y-auto w-full"
                _=(PreEscaped("
                    on htmx:afterSwap
                        if my.children.length > 0 then
                            remove .hidden from me
                        else
                            add .hidden to me
                        end

                    on click from elsewhere
                        add .hidden to me")) {}
        }
        div class="dropdown dropdown-left ml-1" {
            div tabindex="0" role="button" class="btn btn-sm p-1" {
                (icon_bars_3_bottom_left())
            }
            div tabindex="0" class="dropdown-content z-10 menu menu-sm p-2 shadow bg-base-200 w-36 sm:menu-md prose" {
                h4 class="text-center underline" { "Sort" }
                (search_sort_option("Default", None, "default", &data.sort))
                (search_sort_option("Name:", Some("A to Z"), "a-z", &data.sort))
                (search_sort_option("Name:", Some("Z to A"), "z-a", &data.sort))
                (search_sort_option("Date created:", Some("Newest to oldest"), "new-old", &data.sort))
                (search_sort_option("Date created:", Some("Oldest to newest"), "old-new", &data.sort))
                (search_sort_option("Random", None, "random", &data.sort))
            }
        }
        (render_search_favourites_button(data.is_favourites, false))
    }
}

/// Renders the button to search recipes marked as favourites.
pub fn render_search_favourites_button(is_show_favourites: bool, is_oob_swap: bool) -> Markup {
    html! {
        div #search-favourites
            hx-swap-oob=[if is_oob_swap { Some("true") } else { None }] {
            input #fav type="hidden" name="fav" value=(is_show_favourites);
            button #toggle-favourites-button type="submit"
                    title="View all recipes marked as favourites"
                    class="btn btn-square btn-sm ml-1 hover:text-secondary"
                    aria-label="Add to favorites"
                    aria-pressed=(is_show_favourites)
                    _=(format!("on mousedown set #fav.value to '{}'", !is_show_favourites)) {
                (icon_heart(is_show_favourites))
            }
        }
    }
}

fn search_sort_option(
    title: &str,
    subtitle: Option<&str>,
    value: &str,
    selected_opt: &str,
) -> Markup {
    let id = format!("sort-opt-{value}");
    html! {
        fieldset class="fieldset flex" {
            label class="label cursor-pointer text-inherit" for=(id) {
                @if selected_opt == value {
                    input id=(id) type="radio" name="sort" class="radio radio-sm sort-option" value=(value) checked;
                } @else {
                    input id=(id) type="radio" name="sort" class="radio radio-sm sort-option" value=(value);
                }
                span class="ml-1" {
                    (title)
                    @if let Some(sub) = subtitle {
                        (PreEscaped("<br/>"))
                        (sub)
                    }
                }
            }
        }
    }
}

/// Renders the search help pop up.
pub(super) fn search_help() -> Markup {
    let data = [
        ("Any field", "big green squash"),
        ("By category", "cat:dinner"),
        ("By name", "name:chicken kyiv"),
        ("By name and category", "name:chicken kyiv cat:lunch"),
        ("By cuisine", "cui:ukrainian"),
        ("By ingredient", "ing:onions"),
        ("By instruction", "ins:preheat oven 350"),
        ("By keyword", "kw:biscuits"),
        ("By tool", "tool:wok"),
        ("By source", "src:allrecipes.com"),
        (
            "Any field, name and category",
            "best name:chicken kyiv cat:lunch",
        ),
        ("Subcategory", "cat:beverages:cocktails"),
        ("Any field of category", "chicken cat:dinner"),
        ("Multiple categories", "cat:breakfast,dinner"),
        ("Multiple cuisines", "cui:ukrainian,japanese"),
        ("Multiple ingredients", "ing:olive oil,thyme,butter"),
        ("Multiple instructions", "ins:preheat oven 350,melt butter"),
        ("Multiple keywords", "kw:biscuits,mardi gras"),
        ("Multiple sources", "src:allrecipes.com,tasteofhome.com"),
        ("Multiple tools", "tool:wok,blender"),
    ];

    html! {
        div #search-help popover class="hidden card p-0 w-80 bg-base-100 shadow-xl max-h-[28rem] z-20 sm:w-[30rem] " style="position: fixed; inset: unset; bottom: 0.5rem; right: 0.5rem;" {
            div class="card-body max-h-96 p-4" {
                div class="card-actions justify-between" {
                    h2 class="card-title " { "Search Help" }
                    button class="btn btn-square btn-sm" _="on click toggle .hidden on #search-help" {
                        (icon_x_mark())
                    }
                }
                div {
                    p class="text-xs mb-2" {
                        "The following table provide examples of how to perform various searches. You may combine any of these in any order."
                    }
                    div class="overflow-x-auto max-h-64" {
                        table class="table table-xs table-pin-rows" {
                            thead {
                                tr {
                                    th { "Search" }
                                    th { "Example" }
                                }
                            }
                            tbody {
                                @for (term, example) in &data {
                                    tr {
                                        th { (term) }
                                        td { (example) }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Renders the component to display when there are no search results.
pub fn no_results(is_favourites: bool) -> Markup {
    html! {
        div #list-recipes class="grid place-content-center text-sm text-center h-3/5 md:text-base" {
            p class="pt-2" { "No results found." }
        }
        (render_search_favourites_button(is_favourites, true))
    }
}
