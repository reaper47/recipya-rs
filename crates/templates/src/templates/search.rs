use fluent_static::Message;
use l10n::Messages;
use maud::{Markup, PreEscaped, html};

use models::data::SearchbarData;

use super::icons;

/// Renders the searchbar.
pub(super) fn searchbar(data: &SearchbarData, messages: &Messages) -> Markup {
    html! {
        div class="relative w-full" {
            label class="input input-sm flex justify-between px-0 gap-2 z-20 w-full" {
                button #search-shortcut type="button" .pl-2 popovertarget="search-help" _="on click toggle .hidden on #search-help" {
                    (icons::information_circle(true))
                }
                input #search-recipes type="search" name="q"
                    placeholder=(messages.searchbar_input_placeholder())
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
                    (icons::magnifying_glass())
                    span .sr-only {
                        (messages.searchbar_action())
                    }
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
                (icons::bars_3_bottom_left())
            }
            div tabindex="0" class="dropdown-content z-10 menu menu-sm p-2 shadow bg-base-200 w-36 sm:menu-md prose" {
                h4 class="text-center underline" {
                    (messages.sort_recipes_action())
                }
                (search_sort_option(&messages.sort_recipes_default(), None, "default", &data.sort))
                (search_sort_option(&messages.sort_recipes_name(), Some(&messages.sort_recipes_a_to_z()), "a-z", &data.sort))
                (search_sort_option(&messages.sort_recipes_name(), Some(&messages.sort_recipes_z_to_a()), "z-a", &data.sort))
                (search_sort_option(&messages.sort_recipes_date_created(), Some(&messages.sort_recipes_new_to_old()), "new-old", &data.sort))
                (search_sort_option(&messages.sort_recipes_date_created(), Some(&messages.sort_recipes_old_to_new()), "old-new", &data.sort))
                (search_sort_option(&messages.sort_recipes_random(), None, "random", &data.sort))
            }
        }
        (render_search_favourites_button(data.is_favourites, false, messages))
    }
}

/// Renders the button to search recipes marked as favourites.
pub fn render_search_favourites_button(
    is_show_favourites: bool,
    is_oob_swap: bool,
    messages: &Messages,
) -> Markup {
    html! {
        div #search-favourites
            hx-swap-oob=[is_oob_swap.then_some("true")] {
            input #fav type="hidden" name="fav" value=(is_show_favourites);
            button #toggle-favourites-button type="submit"
                    title=(messages.search_favourites_button_title())
                    class="btn btn-square btn-sm ml-1 hover:text-secondary"
                    aria-label=(messages.search_favourites_button_aria_label())
                    aria-pressed=(is_show_favourites)
                    _=(format!("on mousedown set #fav.value to '{}'", !is_show_favourites)) {
                (icons::heart(is_show_favourites))
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
                span .ml-1 {
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
pub(super) fn search_help(messages: &Messages) -> Markup {
    let data = search_help_data(messages);

    html! {
        div #search-help popover class="hidden card p-0 w-80 bg-base-100 shadow-xl max-h-[28rem] z-20 sm:w-[30rem] " style="position: fixed; inset: unset; bottom: 0.5rem; right: 0.5rem;" {
            div class="card-body max-h-96 p-4" {
                div class="card-actions justify-between" {
                    h2 .card-title {
                        (messages.search_help_title())
                    }
                    button class="btn btn-square btn-sm" _="on click toggle .hidden on #search-help" {
                        (icons::x_mark())
                    }
                }
                div {
                    p class="text-xs mb-2" {
                        (messages.search_help_description())
                    }
                    div class="overflow-x-auto max-h-64" {
                        table class="table table-xs table-pin-rows" {
                            thead {
                                tr {
                                    th { (messages.search_help_search()) }
                                    th { (messages.search_help_example()) }
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

fn search_help_data(messages: &Messages) -> [(Message, String); 20] {
    let biscuits = messages.search_help_biscuits();
    let chicken_kyiv = messages.search_help_chicken_kyiv();
    let dinner = messages.search_help_dinner();
    let lunch = messages.search_help_lunch();
    let preheat_oven = messages.search_help_preheat_oven_350();
    let ukrainian = messages.search_help_ukrainian();
    let wok = messages.search_help_wok();

    [
        (
            messages.search_help_any_field(),
            messages.search_help_big_green_squash().to_string(),
        ),
        (messages.search_help_by("category"), format!("cat:{dinner}")),
        (
            messages.search_help_by("name"),
            format!("name:{chicken_kyiv}"),
        ),
        (
            messages.search_help_by_name_category(),
            format!("name:{chicken_kyiv} cat:{lunch}"),
        ),
        (
            messages.search_help_by("cuisine"),
            format!("cui:{ukrainian}"),
        ),
        (
            messages.search_help_by("ingredient"),
            format!("ing:{}", messages.search_help_onions()),
        ),
        (
            messages.search_help_by("instruction"),
            format!("ins:{preheat_oven}"),
        ),
        (messages.search_help_by("keyword"), format!("kw:{biscuits}")),
        (messages.search_help_by("tool"), format!("tool:{wok}")),
        (
            messages.search_help_by("source"),
            "src:allrecipes.com".into(),
        ),
        (
            messages.search_help_any_field_name_category(),
            format!(
                "{} name:{chicken_kyiv} cat:{lunch}",
                messages.search_help_best(),
            ),
        ),
        (
            messages.search_help_by("subcategory"),
            format!(
                "cat:{}:{}",
                messages.search_help_beverages(),
                messages.search_help_cocktails()
            ),
        ),
        (
            messages.search_help_any_field_category(),
            format!("{} cat:{dinner}", messages.search_help_chicken()),
        ),
        (
            messages.search_help_multiple("categories"),
            format!("cat:{},{dinner}", messages.search_help_breakfast()),
        ),
        (
            messages.search_help_multiple("cuisines"),
            format!("cui:{ukrainian},{}", messages.search_help_japanese()),
        ),
        (
            messages.search_help_multiple("ingredients"),
            format!(
                "ing:{},{},{}",
                messages.search_help_olive_oil(),
                messages.search_help_thyme(),
                messages.search_help_butter()
            ),
        ),
        (
            messages.search_help_multiple("instructions"),
            format!("ins:{preheat_oven},{}", messages.search_help_melt_butter()),
        ),
        (
            messages.search_help_multiple("keywords"),
            format!("kw:{biscuits},{}", messages.search_help_mardi_gras()),
        ),
        (
            messages.search_help_multiple("sources"),
            "src:allrecipes.com,tasteofhome.com".into(),
        ),
        (
            messages.search_help_multiple("tools"),
            format!("tool:{wok},{}", messages.search_help_blender()),
        ),
    ]
}

/// Renders the component to display when there are no search results.
pub fn no_results(is_favourites: bool, messages: &Messages) -> Markup {
    html! {
        div #list-recipes class="grid place-content-center text-sm text-center h-3/5 md:text-base" {
            p .pt-2 {
                (messages.searchbar_no_results())
            }
        }
        (render_search_favourites_button(is_favourites, true, messages))
    }
}
