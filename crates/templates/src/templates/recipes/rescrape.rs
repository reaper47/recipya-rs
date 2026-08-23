use std::sync::Arc;

use config::DataDir;
use maud::{Markup, html};
use models::{
    data::Data,
    recipe::structs::{
        media::VideoForCreate,
        nutrition::{NutritionDetailsForCreate, NutritionForCreate},
        recipe::{RecipeField, RecipeForCreate},
        section::SectionComponents,
        time::{Times, TimesForCreate},
        tool::ToolForCreate,
        types::Source,
    },
    settings::UserSettingDetails,
    time::FormattedTimes,
};
use support::fs::FsSupport;
use uuid::Uuid;

use crate::{
    recipes::common::{format_nutrition, nutrition_table_header, render_rating},
    templates::{
        icons::{icon_cooking_pot, icon_cutting_board, icon_globe_alt},
        layouts,
    },
};

const OLD: &str = "old";
const NEW: &str = "new";

/// Represents the difference between two recipes.
pub struct RecipeDiff {
    pub old: RecipeForCreate,
    pub new: RecipeForCreate,
    pub changes: RecipeField,
}

/// Renders the rescrape recipe difference page.
pub fn rescrape_recipe_diff(
    data: &Data,
    data_dir: &DataDir,
    fs_support: &Arc<dyn FsSupport + Sync + Send>,
    user_setting: &UserSettingDetails,
    recipe_id: i64,
    diff: RecipeDiff,
) -> Markup {
    let page_title = format!("Rescrape {}", diff.old.name);
    let content = render_rescrape(data_dir, fs_support, recipe_id, diff);

    html! {
        @if data.is_hx_request {
            title hx-swap-oob="true" {
                (page_title) " | Recipya"
            }
            (content)
        } @else {
            (layouts::main(&page_title, &format!("/{recipe_id}/rescrape"), data, &content, user_setting))
        }
    }
}

#[allow(clippy::too_many_lines)]
fn render_rescrape(
    data_dir: &DataDir,
    fs_support: &Arc<dyn FsSupport + Sync + Send>,
    recipe_id: i64,
    diff: RecipeDiff,
) -> Markup {
    let old_recipe_c = diff.old;
    let new_recipe_c = diff.new;
    let changes = diff.changes;

    html! {
        section .p-2 {
            div class="flex justify-center" {
                div class="card card-border bg-base-100 w-full border-gray-700 xl:w-[72rem]" {
                    form .card-body.contents style="padding: 0" hx-put=(&format!("/recipes/{recipe_id}/rescrape")) hx-indicator="#fullscreen-loader" {
                        (render_title(&old_recipe_c.name, &new_recipe_c.name, changes))
                        div {
                            div class="grid md:grid-flow-col md:grid-cols-6" {
                                div #media-container class="flex flex-row w-full md:col-span-3 md:border-r dark:border-gray-700" {
                                    (render_media(
                                        fs_support,
                                        data_dir,
                                        &DiffMedia {
                                            old: Media {
                                                images: old_recipe_c.images.as_slice(),
                                                videos: old_recipe_c.videos.as_slice(),
                                            },
                                            new: Media {
                                                images: new_recipe_c.images.as_slice(),
                                                videos: new_recipe_c.videos.as_slice(),
                                            },
                                        },
                                        changes,
                                    ))
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
                                    div class="grid grid-flow-col col-span-6 border-b" {
                                        div class="contents grid grid-flow-col" {
                                            (render_times(
                                                old_recipe_c.times.as_ref(),
                                                new_recipe_c.times.as_ref(),
                                                changes,
                                            ))
                                        }
                                    }
                                    @if !changes.contains(RecipeField::DESCRIPTION) && !changes.contains(RecipeField::NUTRITION) {
                                        div class="grid grid-flow-col col-span-6" {
                                            div class="flex gray-700" {
                                                (render_description(old_recipe_c.description.as_deref(), new_recipe_c.description.as_deref(), changes))
                                            }
                                             div class="grid grid-flow-col col-span-6 border-gray-700 overflow-x-auto" {
                                                (render_nutrition(&old_recipe_c.nutrition, &new_recipe_c.nutrition, changes))
                                            }
                                        }
                                    } @else {
                                        div class="grid grid-cols-2 gap-4 col-span-6" {
                                            div class="grid col-span-6 border-gray-700" {
                                                (render_description(old_recipe_c.description.as_deref(), new_recipe_c.description.as_deref(), changes))
                                            }
                                             div class="grid grid-flow-col col-span-6 border-gray-700 overflow-x-auto" {
                                                (render_nutrition(&old_recipe_c.nutrition, &new_recipe_c.nutrition, changes))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        @if changes.contains(RecipeField::INGREDIENTS) || changes.contains(RecipeField::INSTRUCTIONS) {
                            div #ingredients-instructions-container class="md:border-t grid text-sm md:grid-flow-col md:col-span-6 dark:border-gray-700" {
                                div {
                                    div class="col-span-6 border-y md:col-span-2 md:border-r md:border-y-0 dark:border-gray-700" {
                                        (render_tools(old_recipe_c.tools.as_slice(),
                                            new_recipe_c.tools.as_slice(),
                                            changes,
                                        ))
                                        (render_ingredients(&old_recipe_c.ingredients, &new_recipe_c.ingredients, changes))
                                    }
                                    div class="col-span-6 border-gray-700 md:rounded-bl-none md:col-span-4" {
                                        (render_instructions(&old_recipe_c.instructions, &new_recipe_c.instructions,changes))
                                    }
                                }
                            }
                        } @else {
                            div #ingredients-instructions-container class="md:border-t grid text-sm md:grid-flow-col md:col-span-6 dark:border-gray-700" {
                                div class="col-span-6 border-y md:col-span-2 md:border-r md:border-y-0 dark:border-gray-700" {
                                    (render_tools(old_recipe_c.tools.as_slice(),
                                        new_recipe_c.tools.as_slice(),
                                        changes,
                                    ))
                                    (render_ingredients(&old_recipe_c.ingredients, &new_recipe_c.ingredients, changes))
                                }
                                div class="col-span-6 px-6 py-2 border-gray-700 md:rounded-bl-none md:col-span-4" {
                                    (render_instructions(&old_recipe_c.instructions, &new_recipe_c.instructions,changes))
                                }
                            }
                        }
                        div .flex {
                            (render_notes(
                                old_recipe_c.notes.as_deref().unwrap_or_default(),
                                new_recipe_c.notes.as_deref().unwrap_or_default(),
                                changes,
                            ))
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

#[allow(clippy::too_many_lines)]
fn render_ingredients(
    old_ingredients: &SectionComponents,
    new_ingredients: &SectionComponents,
    changes: RecipeField,
) -> Markup {
    const INGREDIENTS_SOURCE: &str = "ingredients-source";
    const INGREDIENTS_OLD: &str = "ingredients-old";
    const INGREDIENTS_NEW: &str = "ingredients-new";

    if changes.contains(RecipeField::INGREDIENTS) {
        html! {
            div .flex {
                label class="w-full diff-minus" {
                    input type="radio" name=(INGREDIENTS_SOURCE) value=(OLD) class="radio radio-sm radio-error mx-2";
                    div .p-2 {
                        h1 class="text-sm font-bold" {
                            "Ingredients"
                        }
                        (render_section(old_ingredients, INGREDIENTS_OLD))
                    }
                }
                label class="w-full diff-plus" {
                    input type="radio" name=(INGREDIENTS_SOURCE) value=(NEW) class="radio radio-sm radio-error mx-2" checked;
                    div .p-2 {
                        h1 class="text-sm font-bold" {
                            "Ingredients"
                        }
                        (render_section(new_ingredients, INGREDIENTS_NEW))
                    }
                }
            }
        }
    } else {
        html! {
            div .p-2 {
                h1 class="text-sm" {
                    b { "Ingredients" }
                }
                (render_section(old_ingredients, INGREDIENTS_OLD))
            }
            input type="hidden" name=(INGREDIENTS_SOURCE) value=(OLD);
        }
    }
}

fn render_instructions(
    old_instructions: &SectionComponents,
    new_instructions: &SectionComponents,
    changes: RecipeField,
) -> Markup {
    const INSTRUCTIONS_SOURCE: &str = "instructions-source";
    const INSTRUCTIONS_OLD: &str = "instructions-old";
    const INSTRUCTIONS_NEW: &str = "instructions-new";

    if changes.contains(RecipeField::INSTRUCTIONS) {
        html! {
            div .flex {
                label class="w-full diff-minus" {
                    input type="radio" name=(INSTRUCTIONS_SOURCE) value=(OLD) class="radio radio-sm radio-error mx-2";
                    div .p-2 {
                        h1 class="text-sm font-bold" {
                            "Instructions"
                        }
                        (render_section(old_instructions, INSTRUCTIONS_OLD))
                    }
                }
                label class="w-full diff-plus" {
                    input type="radio" name=(INSTRUCTIONS_SOURCE) value=(NEW) class="radio radio-sm radio-error mx-2" checked;
                    div .p-2 {
                        h1 class="text-sm font-bold" {
                            "Instructions"
                        }
                        (render_section(new_instructions, INSTRUCTIONS_NEW))
                    }
                }
            }
        }
    } else {
        html! {
            h1 class="text-sm font-bold" {
                "Instructions"
            }
            (render_section(old_instructions, INSTRUCTIONS_OLD))
            input type="hidden" name=(INSTRUCTIONS_SOURCE) value=(OLD);
        }
    }
}

fn render_section(components: &SectionComponents, input_name_base: &str) -> Markup {
    html! {
        @match components {
            SectionComponents::Grouped(sections) => {
                div {
                    @for section in sections {
                        h3 class="font-bold py-2" {
                            (section.title)
                        }
                        ol class="col-span-6 w-full list-disc list-inside"
                            style=(if components.len() > 10 {
                                "column-count: 2"
                            } else {
                                "column-count: 1"
                            }) {
                                @for ing in section.items.iter() {
                                    li class="text-sm" {
                                        (ing.text)
                                    }
                                    input type="hidden" name=(format!("{input_name_base}<>{}", section.title)) value=(ing.text);
                                }
                        }
                    }
                }
            },
            SectionComponents::Flat(items) => {
                ol class="col-span-6 w-full list-disc list-inside"
                    style=(if components.len() > 10 {
                        "column-count: 2"
                    } else {
                        "column-count: 1"
                    }) {
                        @for ing in items.iter() {
                            li class="text-sm" {
                                (ing.text)
                            }
                            input type="hidden" name=(input_name_base) value=(ing.text);
                        }
                }
            },
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

struct DiffMedia<'a> {
    old: Media<'a>,
    new: Media<'a>,
}

struct Media<'a> {
    images: &'a [Uuid],
    videos: &'a [VideoForCreate],
}

impl Media<'_> {
    const fn len(&self) -> usize {
        self.images.len() + self.videos.len()
    }

    const fn is_empty(&self) -> bool {
        self.images.is_empty() && self.videos.is_empty()
    }
}

#[allow(clippy::too_many_lines)]
fn render_media(
    fs_support: &Arc<dyn FsSupport + Sync + Send>,
    data_dir: &DataDir,
    diff: &DiffMedia,
    changes: RecipeField,
) -> Markup {
    const MEDIA_SOURCE: &str = "media-source";
    const MEDIA_NEW_IMAGE: &str = "media-new-image";
    const MEDIA_NEW_VIDEO: &str = "media-new-video";
    const MEDIA_OLD_IMAGE: &str = "media-old-image";
    const MEDIA_OLD_VIDEO: &str = "media-old-video";

    const EXT_IMAGE: &str = ".webp";
    const EXT_VIDEO: &str = ".webm";

    let old_num_images = diff.old.images.len();
    let new_num_images = diff.new.images.len();

    if changes.contains(RecipeField::MEDIA) {
        html! {
            label class="w-full py-2 diff-minus" {
                input type="radio" name=(MEDIA_SOURCE) value=(OLD) class="radio radio-sm radio-error mx-2";

                div #media-old class="col-span-6 my-2" {
                    @if diff.old.is_empty() {
                        p .text-center { "No images" }
                    } @else {
                        @for (idx, &image) in diff.old.images.iter().enumerate() {
                            @let image_exists = fs_support.is_file_exists(image, &data_dir.images.root, ".webp");
                            @let image_src = if image_exists {
                                &format!("/data/images/{image}{EXT_IMAGE}")
                            } else {
                                ""
                            };

                            label id=(format!("media-{}", idx+1)) class={
                                "block"
                                @if (idx+1) > 1 { " hidden" }
                            } {
                                div class={
                                    "cropper-wrap mb-2 w-full min-h-[20rem] relative overflow-hidden"
                                    @if image_src.is_empty() { " hidden" }
                                } {
                                    img src=(image_src) alt=(format!("Image #{} of the recipe", idx+1)) class="block w-full h-full object-contain";
                                    input type="hidden" name=(MEDIA_OLD_IMAGE) value=(image_src);
                                }
                            }
                        }
                        @for (idx, video) in diff.old.videos.iter().enumerate() {
                            @let video_exists = fs_support.is_file_exists(video.video, &data_dir.videos, ".webm");
                            @let video_url = format!("/data/videos/{}{EXT_VIDEO}", video.video);

                            label id=(format!("media-{}", idx+1+old_num_images)) class={
                                @if old_num_images > 0 || idx > 0 { "hidden" }
                            } {
                                img src="" alt="" class="mb-2";
                                @if video_exists {
                                    video controls class="mb-2" src=(video_url) type="video/webm" {}
                                    input type="hidden" name=(MEDIA_OLD_VIDEO) value=(video_url);
                                }
                            }
                        }
                    }
                }

                @if !diff.old.is_empty() {
                    div class="buttons-container-old flex flex-col gap-1 p-1" {
                        @if diff.old.videos.is_empty() && diff.old.images.is_empty() {
                            button #media-button-1 type="button" class="btn btn-sm btn-ghost btn-active" onclick="switchMedia(event, '#media-old', '.buttons-container-old')" {
                                "Media 1"
                            }
                        } @else {
                            @for i in 0..(diff.old.videos.len() + diff.old.images.len()) {
                                button id=(format!("media-button-{}", i+1)) type="button" class={
                                    "btn btn-sm btn-ghost"
                                    @if i == 0 { " btn-active" }
                                } onclick="switchMedia(event, '#media-old', '.buttons-container-old')" {
                                    (format!("Media {}", i + 1))
                                }
                            }
                        }
                    }
                }
            }
            label class="w-full py-2 diff-plus" {
                input type="radio" name=(MEDIA_SOURCE) value=(NEW) class="radio radio-sm radio-success mx-2" checked;

                div #media-new class="col-span-6 my-2" {
                    @if diff.new.is_empty() {
                        p .text-center { "No images" }
                    } @else {
                        @for (idx, &image) in diff.new.images.iter().enumerate() {
                            @let image_exists = fs_support.is_file_exists(image, &data_dir.images.root, ".webp");
                            @let image_src = if image_exists {
                                &format!("/data/images/{image}{EXT_IMAGE}")
                            } else {
                                ""
                            };

                            label id=(format!("media-{}", idx+1)) class={
                                "block"
                                @if (idx+1) > 1 { " hidden" }
                            } {
                                div class={
                                    "cropper-wrap mb-2 w-full min-h-[20rem] relative overflow-hidden"
                                    @if image_src.is_empty() { " hidden" }
                                } {
                                    img src=(image_src) alt=(format!("Image #{} of the recipe", idx+1)) class="block w-full h-full object-contain";
                                    input type="hidden" name=(MEDIA_NEW_IMAGE) value=(image_src);
                                }
                            }
                        }
                        @for (idx, video) in diff.new.videos.iter().enumerate() {
                            @let video_exists = fs_support.is_file_exists(video.video, &data_dir.videos, ".webm");
                            @let video_url = format!("/data/videos/{}{EXT_VIDEO}", video.video);

                            label id=(format!("media-{}", idx+1+new_num_images)) class={
                                @if new_num_images > 0 || idx > 0 { "hidden" }
                            } {
                                img src="" alt="" class="mb-2";
                                @if video_exists {
                                    video controls class="mb-2" src=(video_url) type="video/webm" {}
                                    input type="hidden" name=(MEDIA_NEW_VIDEO) value=(video_url);
                                }
                            }
                        }
                    }
                }

                @if !diff.new.is_empty() {
                    div class="buttons-container-new flex flex-col gap-1 p-1" {
                        @if diff.new.videos.is_empty() && diff.new.images.is_empty() {
                            button #media-button-1 type="button" class="btn btn-sm btn-ghost btn-active" onclick="switchMedia(event, '#media-new', '.buttons-container-new')" {
                                "Media 1"
                            }
                        } @else {
                            @for i in 0..(diff.new.videos.len() + diff.new.images.len()) {
                                button id=(format!("media-button-{}", i+1)) type="button" class={
                                    "btn btn-sm btn-ghost"
                                    @if i == 0 { " btn-active" }
                                } onclick="switchMedia(event, '#media-new', '.buttons-container-new')" {
                                    (format!("Media {}", i + 1))
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        html! {
            div class="col-span-full w-[95vw] md:w-full text-center border-b border-gray-700 md:border-r md:border-b-0 flex items-center justify-center" {
                @match diff.old.len()  {
                    0 => {
                        img style="object-fit: cover"
                            alt="Image of the recipe"
                            class="w-full max-h-80 md:max-h-[34rem]"
                            src="/data/images/Placeholders/placeholder.recipe.webp" {}
                    },
                    1 => {
                        @if old_num_images == 1 {
                            @let image = diff.old.images[0];
                            @if fs_support.is_file_exists(image, &data_dir.images.root, EXT_IMAGE) {
                                img #output style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src=(format!("/data/images/{image}.webp"));
                            } @else {
                               img #output style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src="/data/images/Placeholders/placeholder.recipe.webp";
                            }
                        } @else if let Some(video) = diff.old.videos.first() {
                            @if let Some(url) = &video.embed_url {
                                iframe src=(url) title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen="" style="height: 100%;width: 100%;" {}
                            } @else if let Some(url) = &video.content_url {
                                video controls preload="metadata" src=(url) {}
                            } @else if fs_support.is_file_exists(video.video, &data_dir.videos, ".webm") {
                                video controls preload="metadata" src=(format!("/data/videos/{}.webm",video.video)) type="video/webm" {}
                            } @else {
                                p {
                                    "Video is currently being processed."
                                    br;
                                    "Please refresh the page later."
                                }
                            }
                        }
                    },
                    _ => {
                        div class="carousel w-full" {
                            @for (idx, &img) in diff.old.images.iter().enumerate() {
                                div id=(format!("media-{idx}")) class="carousel-item relative w-full" {
                                    @if fs_support.is_file_exists(img, &data_dir.images.root, ".webp") {
                                         img style="object-fit: cover"
                                        alt="Image of the recipe"
                                        class="w-full max-h-80 md:max-h-[34rem]"
                                        src=(format!("/data/images/{img}.webp"));
                                    } @else {
                                          img style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]"
                                              src="/data/images/Placeholders/placeholder.recipe.webp";
                                    }
                                    div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 bottom-0" {
                                        a class="btn btn-soft btn-sm"
                                            href=(if idx == 0 {
                                                format!("#media-{}", diff.old.len() - 1)
                                            } else {
                                                format!("#media-{}", idx-1)
                                            }) {
                                            "❮"
                                        }
                                        a class="btn btn-soft btn-sm"
                                          href=(if idx == diff.old.len() - 1 {
                                                "#media-0".into()
                                          } else {
                                                format!("#media-{}", idx+1)
                                            }) {
                                            "❯"
                                        }
                                    }
                                }
                            }
                            @for (idx, v) in diff.old.videos.iter().enumerate() {
                                div id=(format!("media-{}", idx+old_num_images)) class="carousel-item relative w-full" {
                                    @if let Some(url) = &v.embed_url {
                                        iframe src=(url) title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen="" style="height: 100%;width: 100%;" {}
                                    } @else if let Some(url) = &v.content_url {
                                        video controls preload="metadata" src=(url) {}
                                    } @else if fs_support.is_file_exists(v.video, &data_dir.videos, ".webm") {
                                        video controls preload="metadata" src=(format!("/data/videos/{}.webm", v.video)) type="video/webm" {}
                                    } @else {
                                        p class="grid place-self-center" {
                                            (format!("Video #{} is currently being processed.", idx+1))
                                            br;
                                            "Please refresh the page later."
                                        }
                                    }
                                    div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 bottom-0" {
                                        a class="btn btn-soft btn-sm" href=(format!("#media-{}", idx.checked_add(old_num_images).and_then(|val| val.checked_sub(1)).unwrap_or(1))) { "❮" }
                                        a class="btn btn-soft btn-sm"
                                          href=(if idx == diff.old.videos.len() - 1 {
                                                "#media-0".into()
                                            } else {
                                                format!("#media-{}", idx+old_num_images +1)
                                            }) {
                                            "❯"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            input type="hidden" name=(MEDIA_SOURCE) value=(OLD);
            @for image in diff.old.images {
                input type="hidden" name=(MEDIA_OLD_IMAGE) value=(format!("/data/images/{image}{EXT_IMAGE}"));
            }
            @for video in diff.old.videos {
                input type="hidden" name=(MEDIA_OLD_VIDEO) value=(format!("/data/videos/{}{EXT_VIDEO}", video.video));
            }
        }
    }
}

fn render_notes(old_notes: &str, new_notes: &str, changes: RecipeField) -> Markup {
    const NOTES_SOURCE: &str = "notes-source";
    const NOTES_OLD: &str = "notes-old";
    const NOTES_NEW: &str = "notes-new";

    const PLACEHOLDER: &str = "Write some notes about the recipe...";
    const CLASS: &str =
        "textarea textarea-ghost w-full h-full resize-none rounded-none focus:outline-none";

    if changes.contains(RecipeField::NOTES) {
        html! {
            div .flex {
                label class="w-full py-2 diff-minus" {
                    input type="radio" name=(NOTES_SOURCE) value=(OLD) class="radio radio-sm radio-error mx-2";
                    div class="col-span-6 dark:border-gray-700" data-notes=(old_notes) data-textarea-id="notes-old" _="on load call initNotes(me.dataset.textareaId, me.dataset.notes)" {
                        textarea #notes-old name="notes-old" placeholder=(PLACEHOLDER) rows="8" class=(CLASS) {}
                    }
                    input type="hidden" name=(NOTES_OLD) value=(old_notes);
                }
                label class="w-full py-2 diff-plus" {
                    input type="radio" name=(NOTES_SOURCE) value=(NEW) class="radio radio-sm radio-success mx-2" checked;
                    div class="col-span-6 dark:border-gray-700" data-notes=(new_notes) data-textarea-id="notes-new" _="on load call initNotes(me.dataset.textareaId, me.dataset.notes)" {
                        textarea #notes-new name="notes-new" placeholder=(PLACEHOLDER) rows="8" class=(CLASS) {}
                    }
                    input type="hidden" name=(NOTES_NEW) value=(new_notes);
                }
            }
        }
    } else if old_notes.is_empty() {
        html! {
            input type="hidden" name=(NOTES_SOURCE) value=(OLD);
            input type="hidden" name=(NOTES_OLD) value=(old_notes);
        }
    } else {
        html! {
            div class="col-span-6 w-full dark:border-gray-700" data-notes=(old_notes) data-textarea-id="notes" _="on load call initNotes(me.dataset.textareaId, me.dataset.notes)" {
                 textarea #notes name="notes" placeholder=(PLACEHOLDER) rows="8" class=(CLASS) {
                     (old_notes)
                 }
            }
            input type="hidden" name=(NOTES_SOURCE) value=(OLD);
            input type="hidden" name=(NOTES_OLD) value=(old_notes);
        }
    }
}

struct NutritionField<'a> {
    label: &'a str,
    key: &'a str,
    unit: &'a str,
    extractor: fn(&NutritionForCreate) -> Option<f64>,
}

const NUTRITION_FIELDS: &[NutritionField] = &[
    NutritionField {
        label: "Calories",
        key: "calories",
        unit: " kcal",
        extractor: |n| n.calories_kcal.map(Into::into),
    },
    NutritionField {
        label: "Total carbs",
        key: "total-carbohydrates",
        unit: "g",
        extractor: |n| n.total_carbohydrates,
    },
    NutritionField {
        label: "Sugars",
        key: "sugars",
        unit: "g",
        extractor: |n| n.sugars_g,
    },
    NutritionField {
        label: "Protein",
        key: "protein",
        unit: "g",
        extractor: |n| n.protein_g,
    },
    NutritionField {
        label: "Total fat",
        key: "total-fat",
        unit: "g",
        extractor: |n| n.total_fat_g,
    },
    NutritionField {
        label: "Saturated fat",
        key: "saturated-fat",
        unit: "g",
        extractor: |n| n.saturated_fat_g,
    },
    NutritionField {
        label: "Unsaturated fat",
        key: "unsaturated-fat",
        unit: "g",
        extractor: |n| n.saturated_fat_g,
    },
    NutritionField {
        label: "Trans fat",
        key: "trans-fat",
        unit: "g",
        extractor: |n| n.trans_fat_g,
    },
    NutritionField {
        label: "Cholesterol",
        key: "cholesterol",
        unit: "mg",
        extractor: |n| n.cholesterol_mg,
    },
    NutritionField {
        label: "Sodium",
        key: "sodium",
        unit: "mg",
        extractor: |n| n.sodium_mg,
    },
    NutritionField {
        label: "Fiber",
        key: "fiber",
        unit: "g",
        extractor: |n| n.fiber_g,
    },
];

#[allow(clippy::too_many_lines)]
fn render_nutrition(
    old_nutrition: &NutritionDetailsForCreate,
    new_nutrition: &NutritionDetailsForCreate,
    changes: RecipeField,
) -> Markup {
    const NUTRITION_SOURCE: &str = "nutrition-source";
    const NUTRITION_OLD: &str = "nutrition-old";
    const NUTRITION_NEW: &str = "nutrition-new";

    const SERVING_SIZE: &str = "serving-size";

    let old_per_100g_vals: Vec<_> = NUTRITION_FIELDS
        .iter()
        .map(|f| {
            let val = old_nutrition
                .per_100g
                .as_ref()
                .and_then(|n| (f.extractor)(n));
            (f, format_nutrition(val, f.unit))
        })
        .collect();

    let old_per_serving_vals: Vec<_> = NUTRITION_FIELDS
        .iter()
        .map(|f| {
            let val = old_nutrition
                .per_serving
                .as_ref()
                .and_then(|n| (f.extractor)(&n.nutrition));
            (f, format_nutrition(val, f.unit))
        })
        .collect();

    let old_serving_size = old_nutrition
        .per_serving
        .as_ref()
        .map_or("-", |n| n.serving_size.as_str());

    let new_per_100g_vals: Vec<_> = NUTRITION_FIELDS
        .iter()
        .map(|f| {
            let val = new_nutrition
                .per_100g
                .as_ref()
                .and_then(|n| (f.extractor)(n));
            (f, format_nutrition(val, f.unit))
        })
        .collect();

    let new_per_serving_vals: Vec<_> = NUTRITION_FIELDS
        .iter()
        .map(|f| {
            let val = new_nutrition
                .per_serving
                .as_ref()
                .and_then(|n| (f.extractor)(&n.nutrition));
            (f, format_nutrition(val, f.unit))
        })
        .collect();

    let new_serving_size = new_nutrition
        .per_serving
        .as_ref()
        .map_or("-", |n| n.serving_size.as_str());

    if changes.contains(RecipeField::NUTRITION) {
        html! {
            label class="w-full py-2 diff-minus" {
                input type="radio" name=(NUTRITION_SOURCE) value=(OLD) class="radio radio-sm radio-error mx-2";
                table class="table table-zebra table-xs" {
                    (nutrition_table_header())
                    tbody {
                        @for (field, formatted) in &old_per_100g_vals {
                            tr data-nutrition-type="per-100g" {
                                td { (field.label) }
                                td { (formatted) }
                            }
                        }

                        @if let Some(serving) = old_nutrition.per_serving.as_ref() {
                            tr data-nutrition-type="per-serving" .hidden {
                                td { "Serving size" }
                                td { @if serving.serving_size.is_empty() { "-" } @else { (serving.serving_size) } }
                            }
                            @for (field, formatted) in &old_per_serving_vals {
                                tr data-nutrition-type="per-serving" .hidden {
                                    td { (field.label) }
                                    td { (formatted) }
                                }
                            }
                        }
                    }
                }

                @for (field, formatted) in &old_per_100g_vals {
                    input type="hidden" name={ (NUTRITION_OLD) "-" (field.key) "-per-100g" } value=(formatted);
                }

                @if old_nutrition.per_serving.is_some() {
                    input type="hidden" name=(SERVING_SIZE) value=(old_serving_size);
                    @for (field, formatted) in &old_per_serving_vals {
                        input type="hidden" name={ (NUTRITION_OLD) "-" (field.key) "-per-serving" } value=(formatted);
                    }
                }
            }
            label class="w-full py-2 diff-plus" {
                input type="radio" name=(NUTRITION_SOURCE) value=(NEW) class="radio radio-sm radio-success mx-2" checked;
                table class="table table-zebra table-xs" {
                    (nutrition_table_header())
                    tbody {
                        @for (field, formatted) in &new_per_100g_vals {
                            tr data-nutrition-type="per-100g" {
                                td { (field.label) }
                                td { (formatted) }
                            }
                        }

                        tr data-nutrition-type="per-serving" .hidden {
                            td { "Serving size" }
                            td { @if new_serving_size.is_empty() { "-" } @else { (new_serving_size) } }
                        }
                        @for (field, formatted) in &new_per_serving_vals {
                            tr data-nutrition-type="per-serving" .hidden {
                                td { (field.label) }
                                td { (formatted) }
                            }
                        }
                    }
                }

                @for (field, formatted) in &new_per_100g_vals {
                    input type="hidden" name={ (NUTRITION_NEW) "-" (field.key) "-per-100g" } value=(formatted);
                }

                @if new_nutrition.per_serving.is_some() {
                    input type="hidden" name=(SERVING_SIZE) value=(new_serving_size);
                    @for (field, formatted) in &old_per_serving_vals {
                        input type="hidden" name={ (NUTRITION_NEW) "-" (field.key) "-per-serving" } value=(formatted);
                    }
                }
            }
        }
    } else {
        html! {
            table class="table table-zebra table-xs" {
                (nutrition_table_header())
                tbody {
                    @for (field, formatted) in &old_per_100g_vals {
                        tr data-nutrition-type="per-100g" {
                            td { (field.label) }
                            td { (formatted) }
                        }
                    }

                    tr data-nutrition-type="per-serving" .hidden {
                        td { "Serving size" }
                        td { @if old_serving_size.is_empty() { "-" } @else { (old_serving_size) } }
                    }
                    @for (field, formatted) in &old_per_serving_vals {
                        tr data-nutrition-type="per-serving" .hidden {
                            td { (field.label) }
                            td { (formatted) }
                        }
                    }
                }
            }
            input type="hidden" name=(NUTRITION_SOURCE) value=(OLD);

            @for (field, formatted) in &old_per_100g_vals {
                input type="hidden" name={ (NUTRITION_OLD) "-" (field.key) "-per-100g" } value=(formatted);
            }

            @if old_nutrition.per_serving.is_some() {
                input type="hidden" name=(SERVING_SIZE) value=(old_serving_size);
                @for (field, formatted) in &old_per_serving_vals {
                    input type="hidden" name={ (NUTRITION_OLD) "-" (field.key) "-per-serving" } value=(formatted);
                }
            }
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

#[allow(clippy::too_many_lines)]
fn render_times(
    old_times: Option<&TimesForCreate>,
    new_times: Option<&TimesForCreate>,
    changes: RecipeField,
) -> Markup {
    const DEFAULT_TIME: &str = "00:15:00";
    const DEFAULT_DATETIME: &str = "PT15M";
    const DEFAULT_DISPLAY: &str = "15m";

    const TIMES_SOURCE: &str = "times-source";
    const TIME_COOK_OLD: &str = "time-cook-old";
    const TIME_COOK_NEW: &str = "time-cook-new";
    const TIME_PREP_OLD: &str = "time-prep-old";
    const TIME_PREP_NEW: &str = "time-prep-new";

    let format_times = |t: &TimesForCreate| -> FormattedTimes {
        FormattedTimes::from_times(&Times {
            id: -1,
            recipe_id: 1,
            prep_seconds: t.prep_seconds,
            cook_seconds: t.cook_seconds,
            total_seconds: t.prep_seconds + t.cook_seconds,
        })
        .unwrap_or_default()
    };

    let non_empty_or =
        |s: &str, default: &str| -> String { if s.is_empty() { default } else { s }.to_string() };

    let old_times = old_times.map(format_times);
    let new_times = new_times.map(format_times);

    let (old_prep_edit, old_cook_edit, old_prep_datetime, old_prep, old_cook_datetime, old_cook) =
        old_times.as_ref().map_or_else(
            || {
                (
                    DEFAULT_TIME.to_string(),
                    DEFAULT_TIME.to_string(),
                    DEFAULT_DATETIME.to_string(),
                    DEFAULT_DISPLAY.to_string(),
                    DEFAULT_DATETIME.to_string(),
                    DEFAULT_DISPLAY.to_string(),
                )
            },
            |t| {
                (
                    non_empty_or(&t.prep_edit, DEFAULT_TIME),
                    non_empty_or(&t.cook_edit, DEFAULT_TIME),
                    t.prep_datetime.clone(),
                    t.prep.clone(),
                    t.cook_datetime.clone(),
                    t.cook.clone(),
                )
            },
        );

    let (new_prep_edit, new_cook_edit, new_prep_datetime, new_prep, new_cook_datetime, new_cook) =
        new_times.as_ref().map_or_else(
            || {
                (
                    DEFAULT_TIME.to_string(),
                    DEFAULT_TIME.to_string(),
                    DEFAULT_DATETIME.to_string(),
                    DEFAULT_DISPLAY.to_string(),
                    DEFAULT_DATETIME.to_string(),
                    DEFAULT_DISPLAY.to_string(),
                )
            },
            |t| {
                (
                    non_empty_or(&t.prep_edit, DEFAULT_TIME),
                    non_empty_or(&t.cook_edit, DEFAULT_TIME),
                    t.prep_datetime.clone(),
                    t.prep.clone(),
                    t.cook_datetime.clone(),
                    t.cook.clone(),
                )
            },
        );

    if changes.contains(RecipeField::TIMES) {
        html! {
            label class="flex w-full py-2 diff-minus" {
                input type="radio" name=(TIMES_SOURCE) value=(OLD) class="radio radio-sm radio-error mx-2";
                div .grid {
                    div class="flex justify-self-center items-center gap-1 cursor-default" title="Prep time" {
                        (icon_cutting_board())
                        time datetime=(old_prep_datetime) { (old_prep) }
                    }
                    div class="flex justify-self-center items-center gap-1 cursor-default" title="Cooking time" {
                        (icon_cooking_pot())
                        time datetime=(old_cook_datetime) { (old_cook) }
                    }
                }
                input type="hidden" name=(TIME_PREP_OLD) value=(old_prep_edit);
                input type="hidden" name=(TIME_COOK_OLD) value=(old_cook_edit);
            }
            label class="flex w-full py-2 diff-plus" {
                input type="radio" name=(TIMES_SOURCE) value=(NEW) class="radio radio-sm radio-success mx-2" checked;
                div .grid {
                    div class="flex justify-self-center items-center gap-1 cursor-default" title="Prep time" {
                        (icon_cutting_board())
                        time datetime=(new_prep_datetime) { (new_prep) }
                    }
                    div class="flex justify-self-center items-center gap-1 cursor-default" title="Cooking time" {
                        (icon_cooking_pot())
                        time datetime=(new_cook_datetime) { (new_cook) }
                    }
                }
                input type="hidden" name=(TIME_PREP_NEW) value=(new_prep_edit);
                input type="hidden" name=(TIME_COOK_NEW) value=(new_cook_edit);
            }
        }
    } else {
        html! {
            div class="flex justify-self-center items-center gap-1 cursor-default" title="Prep time" {
                (icon_cutting_board())
                time datetime=(old_prep_datetime) { (old_prep) }
            }
            div class="flex justify-self-center items-center gap-1 cursor-default" title="Cooking time" {
                (icon_cooking_pot())
                time datetime=(old_cook_datetime) { (old_cook) }
            }

            input type="hidden" name=(TIMES_SOURCE) value=(OLD);
            input type="hidden" name=(TIME_PREP_OLD) value=(old_prep_edit);
            input type="hidden" name=(TIME_COOK_OLD) value=(old_cook_edit);
        }
    }
}

fn render_tools(
    old_tools: &[ToolForCreate],
    new_tools: &[ToolForCreate],
    changes: RecipeField,
) -> Markup {
    const TOOLS_SOURCE: &str = "tools-source";
    const TOOLS_OLD: &str = "tools-old";
    const TOOLS_NEW: &str = "tools-new";

    if changes.contains(RecipeField::TOOLS) {
        html! {
            div .flex {
                label class="w-full diff-minus" {
                    input type="radio" name=(TOOLS_SOURCE) value=(OLD) class="radio radio-sm radio-error mx-2";
                    div .p-2 {
                        h1 class="text-sm" {
                            b { "Tools" }
                        }
                        ol class="col-span-6 w-full mb-4 list-disc list-inside"
                            style=(if old_tools.len() > 10 {
                                "column-count: 2"
                            } else {
                                "column-count: 1"
                            }) {
                            @if old_tools.is_empty() {
                                p { "No tools" }
                            } @else {
                                @for t in old_tools {
                                    li class="text-sm" {
                                        (t.quantity.to_string()) " " (t.name)
                                        input type="hidden" name=(TOOLS_OLD) value=(t.name);
                                    }
                                }
                            }
                        }
                    }
                }
                label class="w-full diff-plus" {
                    input type="radio" name=(TOOLS_SOURCE) value=(NEW) class="radio radio-sm radio-error mx-2" checked;
                    div .p-2 {
                        h1 class="text-sm" {
                            b { "Tools" }
                        }
                        ol class="col-span-6 w-full mb-4 list-disc list-inside"
                            style=(if new_tools.len() > 10 {
                                "column-count: 2"
                            } else {
                                "column-count: 1"
                            }) {
                            @if new_tools.is_empty() {
                                p { "No tools" }
                            } @else {
                                @for t in new_tools {
                                    li class="text-sm" {
                                        (t.quantity.to_string()) " " (t.name)
                                        input type="hidden" name=(TOOLS_NEW) value=(t.name);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else if !old_tools.is_empty() {
        html! {
            h1 class="text-sm" {
                b { "Tools" }
            }
            ol class="col-span-6 w-full mb-4 list-disc list-inside"
                style=(if old_tools.len() > 10 {
                    "column-count: 2"
                } else {
                    "column-count: 1"
                }) {
                @for t in old_tools {
                    li class="text-sm" {
                        (t.quantity.to_string()) " " (t.name)
                        input type="hidden" name=(TOOLS_OLD) value=(t.name);
                    }
                }
            }
            input type="hidden" name=(TOOLS_SOURCE) value=(OLD);
        }
    } else {
        html! {}
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
