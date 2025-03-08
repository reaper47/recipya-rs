use crate::server::templates::data::SearchbarData;
use crate::server::templates::icons::{
    icon_bars_3_bottom_left, icon_information_circle, icon_magnifying_glass, icon_x_mark,
};
use maud::{html, Markup, PreEscaped};

/// Renders the searchbar.
pub(super) fn searchbar(data: &SearchbarData) -> Markup {
    html! {
        div class="w-full" {
            label class="input input-sm flex justify-between px-0 gap-2 z-20" {
                button id="search-shortcut" type="button" class="pl-2" popovertarget="search-help" _="on click toggle .hidden on #search-help" {
                    (icon_information_circle())
                }

                input id="search_recipes" class="w-full" type="search" name="q" placeholder="Search for recipes..." value=(data.term)
                        _=(PreEscaped("on keyup
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
        }
        div class="dropdown dropdown-left ml-1" {
            div tabindex="0" role="button" class="btn btn-sm p-1" {
                (icon_bars_3_bottom_left())
            }
            div tabindex="0" class="dropdown-content z-10 menu menu-sm p-2 shadow bg-base-200 w-52 sm:menu-md prose" {
                h4 { "Sort" }
                (search_sort_option("Default", None, "default", &data.sort))
                (search_sort_option("Name:", Some("A to Z"), "a-z", &data.sort))
                (search_sort_option("Name:", Some("Z to A"), "z-a", &data.sort))
                (search_sort_option("Date created:", Some("Newest to oldest"), "new-old", &data.sort))
                (search_sort_option("Date created:", Some("Oldest to newest"), "old-new", &data.sort))
                (search_sort_option("Random", None, "random", &data.sort))
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
        fieldset class="fieldset" {
            label class="label cursor-pointer" for=(id) {
                (title)
                @if let Some(sub) = subtitle {
                    br;
                    (sub)
                }
            }

            @if selected_opt == value {
                input id=(id) type="radio" name="sort" class="radio radio-sm sort-option" value=(value) checked;
            } @else {
                input id=(id) type="radio" name="sort" class="radio radio-sm sort-option" value=(value);
            }
        }
    }
}

/// Renders the search help pop up.
pub(super) fn search_help() -> Markup {
    let data = [
        ("Any field", "big green squash"),
        ("By category", "cat:dinner"),
        ("Multiple categories", "cat:breakfast,dinner"),
        ("Subcategory", "cat:beverages:cocktails"),
        ("Any field of category", "chicken cat:dinner"),
        ("By name", "name:chicken kyiv"),
        ("By name and category", "name:chicken kyiv cat:lunch"),
        (
            "Any field, name and category",
            "best name:chicken kyiv cat:lunch",
        ),
        ("By description", "desc:tender savory stacked"),
        (
            "Multiple descriptions",
            "desc:tender savory stacked,juicy crispy pieces chicken",
        ),
        ("By cuisine", "cuisine:ukrainian"),
        ("Multiple cuisines", "cuisine:ukrainian,japanese"),
        ("By ingredient", "ing:onions"),
        ("Multiple ingredients", "ing:olive oil,thyme,butter"),
        ("By instruction", "ins:preheat oven 350"),
        ("Multiple instructions", "ins:preheat oven 350,melt butter"),
        ("By keyword", "tag:biscuits"),
        ("Multiple keywords", "tag:biscuits,mardi gras"),
        ("By tool", "tool:wok"),
        ("Multiple tools", "tool:wok,blender"),
        ("By source", "src:allrecipes.com"),
        ("Multiple sources", "src:allrecipes.com,tasteofhome.com"),
    ];

    html! {
        div id="search_help" popover class="hidden card p-0 w-80 bg-base-100 shadow-xl max-h-[28rem] z-20 sm:w-[30rem] " style="position: fixed; inset: unset; bottom: 0.5rem; right: 0.5rem;" {
            div class="card-body max-h-96 p-4" {
                div class="card-actions justify-between" {
                    h2 class="card-title " { "Search Help" }
                    button class="btn btn-square btn-sm" _="on click toggle .hidden on #search_help" {
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
