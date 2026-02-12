use maud::{Markup, html};
use models::{
    data::Data,
    recipe::structs::{
        recipe::{RecipeField, RecipeForCreate},
        types::Source,
    },
    settings::UserSettingDetails,
};

use crate::{
    recipes::common::render_rating,
    templates::{icons::icon_globe_alt, layouts},
};

const OLD: &str = "old";
const NEW: &str = "new";

/// Renders the rescrape recipe difference page.
pub fn rescrape_recipe_diff(
    data: &Data,
    user_setting: &UserSettingDetails,
    recipe_id: i64,
    old_recipe_c: &RecipeForCreate,
    new_recipe_c: &RecipeForCreate,
    changes: RecipeField,
) -> Markup {
    let page_title = format!("Rescrape {}", old_recipe_c.name);
    let content = render_rescrape(recipe_id, old_recipe_c, new_recipe_c, changes);

    html! {
        @if data.is_hx_request {
            title hx-swap-oob="true" {
                (page_title) " | Recipya"
            }
            span #data-layout data-layout="no-aside" hx-swap-oob="true" {}
            (content)
        } @else {
            (layouts::main(&page_title, &format!("/{recipe_id}/rescrape"), data, &content, user_setting, true))
        }
    }
}

fn render_rescrape(
    recipe_id: i64,
    old_recipe_c: &RecipeForCreate,
    new_recipe_c: &RecipeForCreate,
    changes: RecipeField,
) -> Markup {
    html! {
        section .p-2 {
            div class="flex justify-center" {
                div class="card card-border bg-base-100 w-full border-gray-700 xl:w-[72rem]" {
                    form .card-body.contents style="padding: 0" enctype="multipart/form-data" hx-put=(&format!("/recipes/{recipe_id}/edit")) hx-indicator="#fullscreen-loader" {
                        (render_title(&old_recipe_c.name, &new_recipe_c.name, changes))
                        div {
                            div class="grid md:grid-flow-col md:grid-cols-6" {
                                div #media-container class="grid grid-flow-col w-full text-center grid-cols-7 md:col-span-3 md:border-r dark:border-gray-700" {
                                    div class="buttons-container flex flex-col gap-1 p-1" {
                                        // @if view.recipe_details.videos.is_empty() && view.recipe_details.recipe.image.is_none() {
                                        //     button #media-button-1 type="button" class="btn btn-sm btn-ghost btn-active" onclick="switchMedia(event)" {
                                        //         "Media 1"
                                        //     }
                                        // } @else {
                                        //     @for i in 0..view.recipe_details.num_media() {
                                        //         button id=(format!("media-button-{}", i+1)) type="button" class={
                                        //             "btn btn-sm btn-ghost"
                                        //             @if i == 0 { " btn-active" }
                                        //         } onclick="switchMedia(event)" {
                                        //             (format!("Media {}", i + 1))
                                        //         }
                                        //     }
                                        // }
                                        // button #add-media-button type="button" class="btn btn-sm btn-ghost" onclick="addMedia(event)" {
                                        //     (icon_plus_circle())
                                        //     "Add"
                                        // }
                                    }
                                    // (render_media(view, fs_support, data_dir))
                                }
                                div class="grid grid-cols-3 col-span-3 text-sm md:grid-flow-row md:grid-rows-4" style="grid-template-rows: auto" {
                                    div class="grid grid-flow-col border-gray-700 col-span-6" {
                                        div class="flex justify-center items-center" {
                                            (render_rating_diff(old_recipe_c.rating, new_recipe_c.rating, changes))
                                        }
                                    }
                                    div class="grid col-span-6 pb-2 md:grid-cols-3 md:pb-0 md:border-gray-700 md:border-t" {
                                        div class="grid grid-flow-col grid-cols-3 border-b border-t border-gray-700 px-2 md:col-span-2 md:border-b-0 md:border-r md:border-t-0 md:px-0" {
                                            div class="col-span-2 border-r border-gray-700 grid place-items-center" {
                                                (render_category(old_recipe_c.category.as_deref(), new_recipe_c.category.as_deref(), changes))
                                            }
                                            div class="col-span-1" {
                                                (render_yield(
                                                    &old_recipe_c.r#yield.unwrap_or_default().to_string(),
                                                    &new_recipe_c.r#yield.unwrap_or_default().to_string(),
                                                    changes)
                                                )
                                            }
                                        }
                                        div class="relative px-2 pb-2 md:pr-0 grid place-items-center" {
                                            (render_source(&old_recipe_c.source))
                                        }
                                    }
                                    div class="border-gray-700 border-y col-span-6 md:grid-cols-3" {
                                        (render_keywords(
                                            old_recipe_c.keywords.iter().map(String::as_str).collect(),
                                            new_recipe_c.keywords.iter().map(String::as_str).collect(),
                                            changes,
                                        ))
                                    }
                                    div class="grid grid-flow-col col-span-6 py-1 border-b" {
                                        div class="contents grid grid-flow-col" {
                                            // (render_times(view))
                                        }
                                    }
                                    div class="grid grid-flow-col col-span-6" {
                                        div class="grid col-span-6 border-gray-700" {
                                            (render_description(old_recipe_c.description.as_deref(), new_recipe_c.description.as_deref(), changes))
                                        }
                                         div class="grid grid-flow-col col-span-6 border-gray-700 overflow-x-auto" {
                                            // (render_nutrition(view))
                                        }
                                    }
                                }
                            }
                        }
                        div #ingredients-instructions-container class="md:border-t grid text-sm md:grid-flow-col md:col-span-6 dark:border-gray-700" {
                            div class="col-span-6 px-2 py-2 border-y md:col-span-2 md:border-r md:border-y-0 dark:border-gray-700" {
                                // (render_tools(view))
                                // div .divider {}
                                // (render_ingredients(view))
                            }
                            div class="col-span-6 px-6 py-2 border-gray-700 md:rounded-bl-none md:col-span-4" {
                                //(render_instructions(view))
                            }
                        }
                        @let notes = "Hello"; //view.recipe_details.recipe.notes.as_ref().map_or(String::new(), ToString::to_string);
                        div class="col-span-6 dark:border-gray-700"
                            data-notes=(notes)
                            _="on load call initNotes(me.dataset.notes)" {
                             textarea #notes name="notes" placeholder="Write some notes about the recipe..." rows="8" class="textarea textarea-ghost w-full h-full resize-none rounded-none focus:outline-none" {}
                        }
                        div class="card-actions justify-end" {
                            button class="btn btn-primary btn-block btn-sm" { "Submit" }
                        }
                    }
                }
            }
        }
    }
}

fn render_title(old_title: &str, new_title: &str, changes: RecipeField) -> Markup {
    const TITLE_SOURCE: &str = "title-source";
    const TITLE_OLD: &str = "title-old";
    const TITLE_NEW: &str = "title-new";

    if changes.contains(RecipeField::NAME) {
        html! {
            h2 class="card-title place-content-center rounded-t-2xl gap-0" {
                label class="w-full diff-minus" {
                    input type="radio" name=(TITLE_SOURCE) value=(OLD) class="radio radio-sm radio-error";
                    input type="text" readonly class="input w-full text-center bg-base-200 pointer-events-none focus:outline-none" name=(TITLE_OLD) value=(old_title);
                }
                label class="w-full diff-plus" {
                    input type="radio" name=(TITLE_SOURCE) value=(NEW) class="radio radio-sm radio-success" checked;
                    input type="text" readonly class="input w-full text-center bg-base-200 pointer-events-none focus:outline-none" name=(TITLE_NEW) value=(new_title);
                }
            }
        }
    } else {
        html! {
            h2 class="card-title bg-base-200 px-2 pt-2 place-content-center rounded-t-2xl gap-0" style="justify-content: space-between" {
                span class="w-full text-center pb-2" itemprop="name" {
                    (old_title)
                }
                input type="hidden" name=(TITLE_OLD) value=(old_title);
                input type="hidden" name=(TITLE_SOURCE) value=(OLD);
            }
        }
    }
}

// fn render_tools(view: &ViewRecipe) -> Markup {
//     html! {
//         h2 class="font-semibold text-center pb-2" {
//             span .underline { "Tools" }
//         }
//         ol #tools-list class="pl-4" {
//             @if !view.recipe_details.tools.is_empty() {
//                 @for tool in &view.recipe_details.tools {
//                     (add_tool(Some(tool)))
//                 }
//             } @else {
//                 (add_tool(None))
//             }
//         }
//     }
// }

fn render_category(
    old_category: Option<&str>,
    new_category: Option<&str>,
    changes: RecipeField,
) -> Markup {
    const DEFAULT_CATEGORY: &str = "uncategorized";
    const CATEGORY_SOURCE: &str = "category-source";
    const CATEGORY_OLD: &str = "category-old";
    const CATEGORY_NEW: &str = "category-new";

    let old_value = old_category.unwrap_or(DEFAULT_CATEGORY);
    let new_value = new_category.unwrap_or(DEFAULT_CATEGORY);

    if changes.contains(RecipeField::CATEGORY) {
        html! {
            div class="grid w-full" {
                label class="w-full py-2 diff-minus" {
                    input type="radio" name=(CATEGORY_SOURCE) value=(OLD) class="radio radio-sm radio-error mx-2";
                    div class="badge badge-primary badge-outline" {
                        (old_value)
                    }
                    input type="hidden" name=(CATEGORY_OLD) value=(old_value);
                }
                label class="w-full py-2 diff-plus" {
                    input type="radio" name=(CATEGORY_SOURCE) value=(NEW) class="radio radio-sm radio-success mx-2" checked;
                    div class="badge badge-primary badge-outline" {
                        (new_value)
                    }
                    input type="hidden" name=(CATEGORY_NEW) value=(new_value);
                }
            }
        }
    } else {
        html! {
            div class="badge badge-primary badge-outline" {
                (old_value)
            }
            input type="hidden" name=(CATEGORY_OLD) value=(old_value);
            input type="hidden" name=(CATEGORY_SOURCE) value=(OLD);
        }
    }
}

fn render_description(
    old_description: Option<&str>,
    new_description: Option<&str>,
    changes: RecipeField,
) -> Markup {
    const DESCRIPTION_SOURCE: &str = "description-source";
    const DESCRIPTION_OLD: &str = "description-old";
    const DESCRIPTION_NEW: &str = "description-new";

    const DEFAULT_DESCRIPTION: &str = "No description";
    let old_description = old_description.unwrap_or(DEFAULT_DESCRIPTION);
    let new_description = new_description.unwrap_or(DEFAULT_DESCRIPTION);

    if changes.contains(RecipeField::DESCRIPTION) {
        html! {
            label class="w-full diff-minus" {
                input type="radio" name=(DESCRIPTION_SOURCE) value=(OLD) class="radio radio-sm radio-error mx-2";
                textarea readonly class="textarea w-full resize-none rounded-none focus:outline-none" {
                    (old_description)
                }
                input type="hidden" name=(DESCRIPTION_OLD) value=(old_description);
            }
            label class="w-full diff-plus" {
                input type="radio" name=(DESCRIPTION_SOURCE) value=(NEW) class="radio radio-sm radio-error mx-2" checked;
                textarea readonly class="textarea w-full resize-none rounded-none focus:outline-none" {
                    (new_description)
                }
                input type="hidden" name=(DESCRIPTION_NEW) value=(new_description);
            }
        }
    } else {
        html! {
            textarea name="description" placeholder="This Thai curry chicken will make you drool." class="textarea w-full h-full resize-none rounded-none focus:outline-none" {
                (old_description)
            }
            input type="hidden" name=(DESCRIPTION_OLD) value=(old_description);
            input type="hidden" name=(DESCRIPTION_SOURCE) value=(OLD);
        }
    }
}

fn render_keywords(
    old_keywords: Vec<&str>,
    new_keywords: Vec<&str>,
    changes: RecipeField,
) -> Markup {
    const KEYWORDS_SOURCE: &str = "keywords-source";
    const KEYWORDS_OLD: &str = "keywords-old";
    const KEYWORDS_NEW: &str = "keywords-new";

    if changes.contains(RecipeField::KEYWORDS) {
        html! {
            label class="flex items-center w-full diff-minus" {
                input type="radio" name=(KEYWORDS_SOURCE) value=(OLD) class="radio radio-sm radio-error mx-2";
                div class="p-4" {
                    @if old_keywords.is_empty() {
                        p .select-none { "No keywords" }
                    } @else {
                        @for kw in old_keywords {
                            div class="badge badge-sm badge-neutral m-1 flex-auto select-none" { (kw) }
                            input type="hidden" name=(KEYWORDS_OLD) value=(kw);
                        }
                    }
                }
            }
            label class="flex items-center w-full diff-plus" {
                input type="radio" name=(KEYWORDS_SOURCE) value=(NEW) class="radio radio-sm radio-error mx-2" checked;
                div class="p-4" {
                    @if new_keywords.is_empty() {
                        p .select-none { "No keywords" }
                    } @else {
                        @for kw in new_keywords {
                            div class="badge badge-sm badge-neutral m-1 flex-auto select-none" { (kw) }
                            input type="hidden" name=(KEYWORDS_NEW) value=(kw);
                        }
                    }
                }
            }
        }
    } else if old_keywords.is_empty() {
        html! {}
    } else {
        html! {
            div class="p-4" {
                @for kw in old_keywords {
                    div class="badge badge-sm badge-neutral m-1 flex-auto" { (kw) }
                    input type="hidden" name=(KEYWORDS_OLD) value=(kw);
                }
            }
            input type="hidden" name=(KEYWORDS_SOURCE) value=(OLD);
        }
    }
}

fn render_rating_diff(
    old_rating: Option<i16>,
    new_rating: Option<i16>,
    changes: RecipeField,
) -> Markup {
    const RATING_SOURCE: &str = "rating-source";
    const RATING_OLD: &str = "rating-old";
    const RATING_NEW: &str = "rating-new";

    if changes.contains(RecipeField::RATING) {
        html! {
            label class="w-full py-2 diff-minus" {
                input type="radio" name=(RATING_SOURCE) value=(OLD) class="radio radio-sm radio-error mx-2";
                (render_rating("rating", old_rating, None, true, None));
                input type="hidden" name=(RATING_OLD) value=(old_rating.unwrap_or(0));
            }
            label class="w-full py-2 diff-plus" {
                input type="radio" name=(RATING_SOURCE) value=(NEW) class="radio radio-sm radio-success mx-2" checked;
                (render_rating("rating", new_rating, None, true, None));
                input type="hidden" name=(RATING_NEW) value=(new_rating.unwrap_or(0));
            }
        }
    } else {
        html! {
            (render_rating("rating", old_rating, None, true, None))
            input type="hidden" name=(RATING_OLD) value=(old_rating.unwrap_or(0));
            input type="hidden" name=(RATING_SOURCE) value=(OLD);
        }
    }
}

fn render_source(source: &Source) -> Markup {
    html! {
        a class="btn btn-sm btn-outline no-underline" href=(source.as_str()) target="_blank" {
            (icon_globe_alt())
            "Source"
        }
    }
}

fn render_yield(old_yield: &str, new_yield: &str, changes: RecipeField) -> Markup {
    const YIELD_SOURCE: &str = "yield-source";
    const YIELD_OLD: &str = "yield-old";
    const YIELD_NEW: &str = "yield-new";

    if changes.contains(RecipeField::YIELD) {
        html! {
            div class="grid w-full h-full" {
                label class="label grid" {
                    fieldset class="flex diff-minus h-full place-items-center pl-2" {
                        input type="radio" name=(YIELD_SOURCE) value=(OLD) class="radio radio-sm radio-error";
                        p class="text-center" {
                            (old_yield) " servings"
                        }
                        input type="hidden" name=(YIELD_OLD) value=(old_yield);
                    }
                }
                label class="label grid" {
                    fieldset class="flex diff-plus h-full place-items-center pl-2" {
                        input type="radio" name=(YIELD_SOURCE) value=(NEW) class="radio radio-sm radio-error" readonly checked;
                        p class="text-center" {
                            (new_yield) " servings"
                        }
                        input type="hidden" name=(YIELD_NEW) value=(new_yield);
                    }
                }
            }
        }
    } else {
        html! {
            p class="text-center p-0 pt-2 md:col-span-1" {
                (old_yield) " servings"
            }
            input type="hidden" name=(YIELD_OLD) value=(old_yield);
            input type="hidden" name=(YIELD_SOURCE) value=(OLD);
        }
    }
}
