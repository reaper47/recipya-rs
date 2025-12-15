use std::sync::Arc;
use std::time::Duration;

use maud::{Markup, PreEscaped, html};
use models::recipe::structs::section::SectionComponents;
use serde_json::json;
use url::Url;

use config::DataDir;
use models::data::{Data, PaginationData};
use models::settings::UserSettingDetails;
use models::{Recipe, RecipeDetails};
use support::fs::FsSupport;

use crate::recipes::common::render_rating;
use crate::recipes::timeline::render_dialog;
use crate::templates::icons::{
    icon_alarm_clock, icon_bulb_off, icon_bulb_on, icon_clock, icon_cooking_pot,
    icon_cutting_board, icon_document_duplicate, icon_ellipsis_vertical, icon_fire, icon_heart,
    icon_pause, icon_pencil, icon_play, icon_plus_circle, icon_printer, icon_share, icon_stop,
    icon_timeline, icon_trash,
};
use crate::templates::layouts;
use crate::templates::pagination::pagination;
use crate::{Error, Result};

/// Renders the details of a recipe.
pub fn view_recipe(
    fs_support: Arc<dyn FsSupport + Sync + Send>,
    path: &str,
    data_dir: DataDir,
    data: Data,
    user_setting: UserSettingDetails,
) -> Result<Markup> {
    let view = data
        .recipes
        .first()
        .ok_or("Must have at least one recipe.")
        .map_err(|_| Error::NoRecipe)?;

    Ok(html! {
        @if data.is_hx_request {
            title hx-swap-oob="true" {
                 (view.recipe_details.recipe.name) " | Recipya"
            }
            span #data-layout data-layout="no-aside" hx-swap-oob="true" {}
            (view_recipe_helper(fs_support, data_dir, &data)?)
        } @else {
            (layouts::main(
                &view.recipe_details.recipe.name,
                path,
                &data,
                view_recipe_helper(fs_support, data_dir, &data)?,
                user_setting,
                true,
            ))
        }
        (pagination(&PaginationData::hidden()))
    })
}

pub fn view_recipe_helper(
    fs_support: Arc<dyn FsSupport + Sync + Send>,
    data_dir: DataDir,
    data: &Data,
) -> Result<Markup> {
    let view = data
        .recipes
        .first()
        .ok_or("Must have at least one recipe.")
        .map_err(|_| Error::NoRecipe)?;

    let recipe_id = view.recipe_details.recipe.id;
    let recipe_details = &view.recipe_details;
    let recipe = &recipe_details.recipe;

    Ok(html! {
        @if !matches!(&data.share, Some(share) if share.is_shared) {
             dialog #share-dialog .modal {
                div class="modal-box w-4/5 sm:w-96" {
                    div #share-dialog-result {}
                    div class="modal-action block mt-4" {
                        form method="dialog" {
                            button class="btn btn-block btn-outline btn-sm" {
                                "Close"
                            }
                        }
                    }
                }
            }
        }

        section class={
            @if !data.is_preview { "p-2" }
        } data-layout="no-aside" {
            div class="flex justify-center" {
                div class="card card-border bg-base-100 shadow-none w-full border-gray-700 xl:w-[72rem] print:rounded-none"
                    dir=(if recipe_details.is_rtl() { "rtl" } else { "ltr" }) {
                    div class="card-body contents" style="padding: 0" {
                        (render_header(recipe_id, &data, recipe_details, recipe.is_favourite))
                        div class="grid md:grid-flow-col md:grid-cols-6" {
                            (render_media(fs_support, &view.recipe_details, &data_dir))
                            div class="grid grid-cols-3 col-span-3 md:grid-flow-row md:grid-rows-4 print:grid-rows-2" style="grid-template-rows: auto" {
                                div class="grid grid-flow-col border-gray-700 col-span-6 md:border-t md:row-span-1 print:border-none" {
                                    div class={
                                        "flex justify-center items-center"
                                        @if data.is_preview { " md:hidden" }
                                    } {
                                        (render_rating("rating", recipe.rating, None, true, None))
                                    }
                                }
                                div class="grid grid-flow-col col-span-6 md:col-span-3 md:row-span-1 md:border-y md:border-gray-700 print:row-span-1 print:grid-cols-2 print:border-b-black print:border" {
                                    div class="col-span-2 grid place-items-center md:col-span-1 print:col-span-1 print:float-left print:ml-2 print:border-r print:border-black" {
                                        div class={
                                            "badge badge-primary badge-outline"
                                            @if data.is_preview { " badge-sm" }
                                        } {
                                            (view.recipe_details.category)
                                        }
                                    }
                                    div class="grid col-span-2 border-gray-700 place-items-center text-sm border-x md:col-span-1 print:hidden" {
                                        @if data.is_authenticated && !data.is_preview {
                                            form autocomplete="off" _="on submit halt the event" class="print:hidden" {
                                                fieldset class="fieldset" {
                                                    legend { "Servings" }
                                                    input #yield
                                                        type="number"
                                                        min="1"
                                                        name="yield"
                                                        value=(if recipe.yield_ == 0 {
                                                            "1".into()
                                                        } else {
                                                            recipe.yield_.to_string()
                                                        })
                                                        class="input md:max-w-24"
                                                        hx-get=(format!("/recipes/{recipe_id}/scale"))
                                                        hx-trigger="input"
                                                        hx-target="#ingredients-instructions-container";
                                                }
                                            }
                                        } @else {
                                            p class="text-sm text-center" {
                                                (format!("{} servings", recipe.yield_))
                                            }
                                        }
                                    }
                                    p class="hidden p-0 pt-2 md:col-span-1 print:grid print:text-center print:place-content-center" {
                                        (recipe.yield_.to_string()) " servings"
                                    }
                                    div class={
                                        "flex items-center justify-center col-span-2 text-sm md:col-span-1 print:hidden"
                                        @if data.is_preview { " md:hidden" }
                                    } {
                                        (render_source(&recipe.source))
                                    }
                                }
                                @if data.is_preview {
                                    div class="col-span-6 border-b" {
                                        (render_source(&recipe.source))
                                    }
                                }
                                @if !recipe_details.keywords.is_empty() {
                                    div class="border-gray-700 border-y col-span-6 md:border-t-0 md:grid-cols-3 print:border-none" {
                                        div class="p-4" {
                                            @for kw in recipe_details.keywords.iter() {
                                                div class="badge badge-sm badge-neutral m-1 flex-auto" { (kw) }
                                            }
                                        }
                                    }
                                }
                                div class={
                                        "grid grid-flow-col border-gray-700 col-span-6 py-1 md:border-b md:row-span-1 print:border-none"
                                        @if recipe_details.nutrition.is_none() { " print:hidden" }
                                        @if data.is_preview { " md:grid-cols-3" } @else { " md:grid-cols-4" }
                                    } {
                                    div class="contents grid grid-flow-col md:col-span-6" {
                                            div class="flex justify-self-center items-center gap-1 cursor-default" title="Prep time" {
                                                (icon_cutting_board())
                                                time datetime=(view.formatted_times.prep_datetime) { (view.formatted_times.prep) }
                                            }
                                            div class="flex justify-self-center items-center gap-1 cursor-default" title="Cooking time" {
                                                (icon_cooking_pot())
                                                time datetime=(view.formatted_times.cook_datetime) { (view.formatted_times.cook) }
                                            }
                                            div class="flex justify-self-center items-center gap-1 cursor-default" title="Total time" {
                                                (icon_clock())
                                                time datetime=(view.formatted_times.total_datetime) { (view.formatted_times.total) }
                                            }
                                    }
                                }
                                @if data.is_preview {
                                    div class="col-span-6 text-center border-b " {
                                        (render_rating("rating", recipe.rating, None, true, None))
                                    }
                                }
                                div class={
                                    "grid-flow-col border-gray-700 col-span-6 print:border-none"
                                    @if data.is_preview { " flex flex-col" } @else { " grid" }
                                    @if recipe_details.nutrition.is_none() { " print:hidden" }
                                } {
                                    div class={
                                        "col-span-3 md:h-full md:border-r md:row-span-1 print:hidden"
                                        @if data.is_preview { " border-b" } @else { " min-h-40" }
                                    } {
                                        textarea readonly class="textarea textarea-ghost w-full h-full resize-none rounded-none focus:outline-none" {
                                            @if let Some(description) = &recipe.description {
                                                (description)
                                            } @else {
                                                "No description"
                                            }
                                        }
                                    }
                                    div class="col-span-3" {
                                        (render_nutrition(&recipe_details))
                                    }
                                }
                            }
                        }
                        (print_description(&recipe))
                        div class="border-gray-700 md:border-t" {
                            (render_ingredients_instructions(&recipe_details))
                            div class="hidden print:grid col-span-6 ml-2 my-1" {
                                (render_tools(recipe_details))
                                (render_ingredients(recipe_details))
                            }
                            div class="hidden col-span-5 overflow-visible print:inline" {
                                (render_instructions(recipe_details))
                            }
                        }
                        (print_source(recipe))
                        @if let Some(notes) = &recipe_details.recipe.notes {
                            details class="border-t w-full" {
                                summary class="font-semibold p-2 select-none" { "Notes" }
                                div class="prose gap-2 py-2 px-4" {
                                    (PreEscaped(markdown::to_html(notes)))
                                }
                            }
                        }
                    }
                }
            }
        }

        (render_dialog(recipe.id, vec![]))
    })
}

fn print_description(recipe: &Recipe) -> Markup {
    html! {
        @if let Some(description) = &recipe.description {
            div class="hidden print:block print:mx-2" {
                h1 class="text-sm print:mb-1" {
                    b { "Description" }
                }
                (description)
            }
        }
    }
}

fn print_source(recipe: &Recipe) -> Markup {
    html! {
        div class="hidden print:block print:mx-2 print:mb-2 print:text-sm" {
            h1 class="print:mb-1" {
                b { "Source" }
            }
            @if !&recipe.source.is_empty() {
                 @if Url::parse(&recipe.source).is_ok() {
                    p class="print:overflow-hidden" {
                        (&recipe.source)
                    }
                 } @else {
                       p { "Source: Unknown" }
                }
            } @else {
                p { "Source: Unknown" }
            }
        }
    }
}

fn render_header(
    recipe_id: i64,
    data: &Data,
    recipe_details: &RecipeDetails,
    is_favourite: bool,
) -> Markup {
    html! {
        h2 class="card-title bg-base-200 px-2 pt-2 place-content-center rounded-t-2xl print:border-b print:border-black" style="justify-content: space-between" {
            @if !data.is_preview {
                (render_left_controls(recipe_id, data))
            }
            span class={
                "text-center pb-2 print:w-full"
                @if data.is_preview { " w-full" }
            } itemprop="name" {
                    (recipe_details.recipe.name)
            }
            @if !data.is_preview {
                (render_right_controls(recipe_id, is_favourite, data))
            }
        }
    }
}

fn render_left_controls(recipe_id: i64, data: &Data) -> Markup {
    html! {
        span class="grid grid-flow-col place-items-center pb-2 print:hidden" {
            button title="Toggle screen lock" _="on load if not navigator.wakeLock hide me end on click call toggleWakeLock()" {
                span #bulb-on .hidden {
                    (icon_bulb_on())
                }
                span #bulb-off {
                    (icon_bulb_off())
                }
            }

            @if data.is_authenticated && matches!(&data.share, Some(share) if share.is_from_host) {
                button #edit-recipe class="ml-2 hidden sm:block"
                    title="Edit recipe"
                    hx-get=(format!("/recipes/{recipe_id}/edit"))
                    hx-push-url="true"
                    hx-target="#content"
                    hx-swap="innerHTML transition:true" {
                    (icon_pencil(true))
                }
            }
        }
    }
}

fn render_right_controls(recipe_id: i64, is_favourite: bool, data: &Data) -> Markup {
    html! {
        span class="md:hidden" {
            button title="Open recipe options menu" popovertarget="recipe_menu" popovertargetaction="toggle" {
                (icon_ellipsis_vertical())
            }
            div #recipe-menu
                popover
                style="inset: unset; top: 3.5rem; right: 0.5rem;"
                class="rounded-box z-10 shadow bg-base-100"
                _="on click if me.matches(':popover-open') then me.hidePopover()" {
                ul tabindex="0" class="menu w-full" {
                    li {
                        a #edit-recipe title="Edit recipe"
                            hx-get=(format!("/recipes/{recipe_id}/edit"))
                            hx-push-url="true"
                            hx-target="#content"
                            hx-swap="innerHTML transition:true" {
                            (icon_pencil(false))
                            "Edit"
                        }
                    }
                    @if !matches!(&data.share, Some(share) if share.is_shared) {
                        li {
                            a title="Share recipe"
                                hx-post=(format!("/recipes/{recipe_id}/share"))
                                hx-target="#share-dialog-result"
                                hx-push-url="false"
                                _="on htmx:afterRequest from me
                                        if event.detail.successful
                                            if navigator.canShare
                                                set name to document.querySelector('[itemprop=name]').textContent then
                                                set data to {title: name, text: name, url: document.querySelector('#share-dialog-result input').value} then
                                                call navigator.share(data)
                                            else
                                                call #share-dialog.showModal()
                                        end" {
                                (icon_share())
                                "Share"
                            }
                        }
                        li {
                            a #duplicate-recipe title="Duplicate recipe"
                                hx-push-url="/recipes/add/manual"
                                hx-get=(format!("/recipes/{recipe_id}/duplicate"))
                                hx-target="#content" {
                                (icon_document_duplicate())
                                "Duplicate"
                            }
                        }
                    }
                    li title="Print recipe" _="on click print()" {
                        a {
                            (icon_printer())
                            "Print"
                        }
                    }
                    @if matches!(&data.share, Some(share) if share.is_from_host) {
                        li {
                            a title="Mark or unmark as favourite" {
                                "Favourite"
                            }

                        }
                        li {
                            a title="Delete recipe"
                                hx-delete=(format!("/recipes/{recipe_id}"))
                                hx-swap="none"
                                hx-confirm="Are you sure you wish to delete this recipe?"
                                hx-indicator="#fullscreen-loader" {
                                (icon_trash())
                                "Delete"
                            }
                        }
                    }
                }
            }
        }
        span class="grid grid-flow-col place-items-center pb-2 print:hidden" {
            @if matches!(&data.share, Some(share) if share.is_shared) {
                @if !matches!(&data.share, Some(share) if share.is_from_host) {
                    button class="mr-2"
                        title="Add recipe to collection"
                        hx-get=(format!("/recipes/{recipe_id}/share"))
                        hx-push-url="true" {
                        (icon_plus_circle())
                    }
                }
            } @else {
                (render_favourite_button(recipe_id, is_favourite, false, true))
                button type="button" title="Open timeline" aria-label="Open timeline" class="btn btn-xs btn-ghost"
                        hx-get=(format!("/recipes/{recipe_id}/timeline"))
                        hx-target="#timeline-dialog-result"
                        hx-push-url="false"
                        _="on htmx:afterRequest from me call #timeline-dialog.showModal()" {
                    (icon_timeline())
                }
            }
            div class="dropdown" {
                div tabindex="0" role="button" class="btn btn-xs btn-ghost" {
                    (icon_ellipsis_vertical())
                }
                ul tabindex="0" class="dropdown-content menu bg-base-100 rounded-box z-1 w-40 p-2 shadow-sm" {
                    @if matches!(&data.share, Some(share) if !share.is_shared) {
                        li _="on click document.activeElement.blur()" {
                            button _="on click call #timeline-new-event-dialog.showModal()" {
                                (icon_fire())
                                "Recipe made"
                            }
                        }
                        li _="on click document.activeElement.blur()" {
                            button hx-post=(format!("/recipes/{recipe_id}/share"))
                                hx-target="#share-dialog-result"
                                hx-push-url="false"
                                _="on htmx:afterRequest from me
                                    if event.detail.successful
                                        if navigator.canShare
                                            set name to document.querySelector('[itemprop=name]').textContent then
                                            set data to {title: name, text: name, url: document.querySelector('#share-dialog-result input').value} then
                                            call navigator.share(data)
                                        else
                                            call #share-dialog.showModal()
                                    end" {
                                (icon_share())
                                "Share"
                            }
                        }
                    }
                    li  _="on click document.activeElement.blur()" {
                        button #duplicate-recipe hx-push-url="/recipes/add/manual" hx-get=(format!("/recipes/{recipe_id}/duplicate")) hx-target="#content" {
                            (icon_document_duplicate())
                            "Duplicate"
                        }
                    }
                    li {
                        button _="on click print()" {
                            (icon_printer())
                            "Print"
                        }
                    }
                    @if !matches!(&data.share, Some(share) if share.is_shared) {
                        li {
                            button hx-delete=(format!("/recipes/{recipe_id}"))
                                hx-swap="none"
                                hx-confirm="Are you sure you wish to delete this recipe?"
                                hx-indicator="#fullscreen-loader" {
                                (icon_trash())
                                "Delete"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_ingredients(recipe_details: &RecipeDetails) -> Markup {
    html! {
        h1 class="text-sm print:mb-1" {
            b { "Ingredients" }
        }
        ol class="col-span-6 w-full print:mb-2"
            style=(if recipe_details.ingredients.len() > 10 {
                "column-count: 2"
            } else {
                "column-count: 1"
            }) {
                @match &recipe_details.ingredients {
                    SectionComponents::Grouped(section) => {
                        @for section in section.iter() {
                            @for ing in section.items.iter() {
                                li class="text-sm" {
                                    label {
                                        input type="checkbox";
                                    }
                                    span class="pl-2" { (ing.text) }
                                }
                            }
                        }
                    },
                    SectionComponents::Flat(items) => {
                        @for ing in items.iter() {
                            li class="text-sm" {
                                label {
                                    input type="checkbox";
                                }
                                span class="pl-2" { (ing.text) }
                            }
                        }
                    },
                }
        }
    }
}

fn render_instructions(recipe_details: &RecipeDetails) -> Markup {
    html! {
        h1 class="text-sm print:ml-2 print:mb-1" {
            b { "Instructions" }
        }
        ol class="col-span-6 list-decimal w-full ml-6" {
            @match &recipe_details.instructions {
                SectionComponents::Grouped(section) => {
                    @for section in section.iter() {
                        @for ins in section.items.iter() {
                            li class="print:mr-4" {
                                span class="text-sm whitespace-pre-line" {
                                    (ins.text)
                                }
                            }
                        }
                    }
                },
                SectionComponents::Flat(items) => {
                    @for ins in items.iter() {
                        li class="print:mr-4" {
                            span class="text-sm whitespace-pre-line" {
                                (ins.text)
                            }
                        }
                    }
                },
            }
        }
    }
}

fn render_media(
    fs_support: Arc<dyn FsSupport + Sync + Send>,
    recipe_details: &RecipeDetails,
    data_dir: &DataDir,
) -> Markup {
    html! {
        div class="w-[95vw] md:w-full text-center border-b border-gray-700 md:col-span-3 md:border-r md:border-b-0 flex items-center justify-center print:hidden" {
            @match recipe_details.num_media()  {
                0 => {
                    img style="object-fit: cover"
                        alt="Image of the recipe"
                        class="w-full max-h-80 md:max-h-[34rem]"
                        src="/data/images/Placeholders/placeholder.recipe.webp" {}
                },
                1 => {
                    @if recipe_details.num_images() == 1 {
                        @if let Some(image) = recipe_details.recipe.image {
                            img #output style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src=(format!("/data/images/{image}.webp"));
                        } @else {
                           img #output style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src="/data/images/Placeholders/placeholder.recipe.webp";
                        }
                    } @else if let Some(video) = recipe_details.videos.first() {
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
                        @for (idx, &img) in recipe_details.all_images().iter().enumerate() {
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
                                            format!("#media-{}", recipe_details.num_media() - 1)
                                        } else {
                                            format!("#media-{}", idx-1)
                                        }) {
                                        "❮"
                                    }
                                    a class="btn btn-soft btn-sm"
                                      href=(if idx == recipe_details.num_media() - 1 {
                                            "#media-0".into()
                                      } else {
                                            format!("#media-{}", idx+1)
                                        }) {
                                        "❯"
                                    }
                                }
                            }
                        }
                        @for (idx, v) in recipe_details.videos.iter().enumerate() {
                            div id=(format!("media-{}", idx+recipe_details.num_images())) class="carousel-item relative w-full" {
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
                                    a class="btn btn-soft btn-sm" href=(format!("#media-{}", idx.checked_add(recipe_details.num_images()).and_then(|val| val.checked_sub(1)).unwrap_or(1))) { "❮" }
                                    a class="btn btn-soft btn-sm"
                                      href=(if idx == recipe_details.num_videos() - 1 {
                                            "#media-0".into()
                                        } else {
                                            format!("#media-{}", idx+recipe_details.num_images() +1)
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
    }
}

fn render_nutrition(recipe_details: &RecipeDetails) -> Markup {
    html! {
        table class="table table-zebra table-xs print:hidden" {
            thead {
                tr {
                    th {
                        "Nutrition (per"
                        @if let Some(nutrition) = &recipe_details.nutrition {
                            @if nutrition.serving_size.as_deref() != Some("100g") {
                                " serving)"
                            } @else {
                                " 100g)"
                            }
                        } @else {
                            " 100g)"
                        }
                    }
                    th {
                        "Amount"
                    }
                }
            }
            tbody {
                @if let Some(nutrition) = &recipe_details.nutrition {
                    @let rows = [
                        ("Calories:", nutrition.calories_kcal.map(|v| format!("{v} kcal")).unwrap_or("-".into())),
                        ("Total carbs:", nutrition.total_carbohydrates.map(|v| format!("{v} g")).unwrap_or("-".into())),
                        ("Sugars:", nutrition.sugars_g.map(|v| format!("{v} g")).unwrap_or("-".into())),
                        ("Protein:", nutrition.protein_g.map(|v| format!("{v} g")).unwrap_or("-".into())),
                        ("Total fat:", nutrition.total_fat_g.map(|v| format!("{v} g")).unwrap_or("-".into())),
                        ("Saturated fat:", nutrition.saturated_fat_g.map(|v| format!("{v} g")).unwrap_or("-".into())),
                        ("Unsaturated fat:", nutrition.unsaturated_fat_g.map(|v| format!("{v} g")).unwrap_or("-".into())),
                        ("Trans fat:", nutrition.trans_fat_g.map(|v| format!("{v} g")).unwrap_or("-".into())),
                        ("Cholesterol:", nutrition.cholesterol_mg.map(|v| format!("{v} mg")).unwrap_or("-".into())),
                        ("Sodium:", nutrition.sodium_mg.map(|v| format!("{v} mg")).unwrap_or("-".into())),
                        ("Fiber:", nutrition.fiber_g.map(|v| format!("{v} g")).unwrap_or("-".into())),
                    ];
                    @for (name, value) in rows {
                        tr {
                            td { (name) }
                            td { (value) }
                        }
                    }
                } @else {
                    @let rows = [
                        "Calories:",
                        "Total carbs:",
                        "Sugars:",
                        "Protein:",
                        "Total fat:",
                        "Saturated fat:",
                        "Unsaturated fat:",
                        "Trans fat:",
                        "Cholesterol:",
                        "Sodium:",
                        "Fiber:",
                    ];
                    @for name in rows {
                        tr {
                            td { (name) }
                            td { "-" }
                        }
                    }
                }
            }
        }
        @if let Some(nutrition) = &recipe_details.nutrition {
            div class="hidden pt-2 print:block print:mx-2 print:my-1" {
                h1 class="text-sm print:mb-1" {
                    b { "Nutrition Facts" }
                }
                p class="text-xs" {
                    (nutrition.to_line())
                }
            }
        }
    }
}

fn render_source(source: &String) -> Markup {
    html! {
        @if !source.is_empty() {
           @if Url::parse(source).is_ok() {
                a class="btn btn-sm btn-outline no-underline print:hidden" href=(source) target="_blank" { "Source" }
                p class="hidden print:block print:whitespace-nowrap print:overflow-hidden print:text-ellipsis print:max-w-xs" { (source) }
           } @else {
                p class="text-center" {
                    "Source:"
                    br;
                    (source)
                }
           }
        } @else {
            p class="text-center" {
                "Source:"
                br;
                "Unknown"
            }
        }
    }
}

fn render_tools(recipe_details: &RecipeDetails) -> Markup {
    html! {
        @if !recipe_details.tools.is_empty() {
            h1 class="text-sm print:mb-1" {
                b { "Tools" }
            }
            ol class="col-span-6 w-full mb-4"
                style=(if recipe_details.tools.len() > 10 {
                    "column-count: 2"
                } else {
                    "column-count: 1"
                }) {
                @for t in recipe_details.tools.iter() {
                    li class="text-sm" {
                        label class="flex items-center w-full" {
                            input type="checkbox";
                        }
                        span class="pl-2" {
                            (t.quantity.to_string()) (t.name)
                        }
                    }
                }
            }
        }
    }
}

/// Renders the ingredient and the instruction lists.
pub fn render_ingredients_instructions(recipe: &RecipeDetails) -> Markup {
    html! {
        div #ingredients-instructions-container class="grid text-sm md:grid-cols-6 md:col-span-6" {
            div class="col-span-6 border-gray-700 border-y px-4 py-2 md:col-span-2 md:border-r md:border-y-0 print:hidden" {
                @if !recipe.tools.is_empty() {
                    h2 class="font-semibold text-center underline pb-1" { "Tools" }
                    ul class="list grid gap-1" {
                        @for tool in recipe.tools.iter() {
                            li class="list-row py-1 no-after select-none grid grid-cols-1 hover:bg-base-300" {
                                label class="flex items-center w-full" {
                                    input type="checkbox" class="checkbox";
                                    span class="px-2" { (tool.quantity.to_string()) " " (tool.name) }
                                }
                            }
                        }
                    }
                }
                h2 class="font-semibold text-center underline pb-1" { "Ingredients" }
                ul class="list grid gap-1" {
                    @match &recipe.ingredients {
                        SectionComponents::Grouped(section) => {
                            @for section in section.iter() {
                                @for ingredient in section.items.iter() {
                                     li class="list-row py-1 no-after select-none grid grid-cols-1 hover:bg-base-300" {
                                        label class="flex items-center w-full" {
                                            input type="checkbox" class="checkbox";
                                            span class="px-2" { (ingredient.text) }
                                        }
                                    }
                                }
                            }
                        },
                        SectionComponents::Flat(items) => {
                            @for ingredient in items.iter() {
                                 li class="list-row py-1 no-after select-none grid grid-cols-1 hover:bg-base-300" {
                                    label class="flex items-center w-full" {
                                        input type="checkbox" class="checkbox";
                                        span class="px-2" { (ingredient.text) }
                                    }
                                }
                            }
                        },
                    }
                }
            }
            div class="col-span-6 px-8 py-2 border-gray-700 md:rounded-bl-none md:col-span-4 print:hidden" {
                h2 class="font-semibold text-center underline pb-1" { "Instructions" }
                ol class="grid list-decimal" {
                    @match &recipe.instructions {
                        SectionComponents::Grouped(section) => {
                            @for section in section.iter() {
                                @for (idx, instruction) in section.items.iter().enumerate() {
                                    li class="min-w-full py-2 select-none hover:bg-base-300" {
                                        div class="flex" {
                                            div class="whitespace-pre-line w-full" _="on mousedown toggle .line-through" {
                                                (instruction.text)
                                            }
                                             @if let Some(d) = instruction.duration_seconds {
                                                div id=(format!("timer-container-{idx}")) class="timer-container" {
                                                    button class="timer btn btn-sm btn-circle btn-ghost" title=(format_timer_label(d))
                                                           _="on click add .hidden to me
                                                              remove .hidden from the next <div/>
                                                              call initTimer(event)" {
                                                        (icon_alarm_clock())
                                                    }
                                                    (render_countdown(&format!("countdown-step-{idx}"), d))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        SectionComponents::Flat(items) => {
                            @for (idx, instruction) in items.iter().enumerate() {
                                li class="min-w-full py-2 select-none hover:bg-base-300" {
                                    div class="flex" {
                                        div class="whitespace-pre-line w-full" _="on mousedown toggle .line-through" {
                                            (instruction.text)
                                        }
                                         @if let Some(d) = instruction.duration_seconds {
                                            div id=(format!("timer-container-{idx}")) class="timer-container" {
                                                button class="timer btn btn-sm btn-circle btn-ghost" title=(format_timer_label(d))
                                                       _="on click add .hidden to me
                                                          remove .hidden from the next <div/>
                                                          call initTimer(event)" {
                                                    (icon_alarm_clock())
                                                }
                                                (render_countdown(&format!("countdown-step-{idx}"), d))
                                            }
                                        }
                                    }
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}

fn format_timer_label(seconds: i32) -> String {
    let duration = Duration::from_secs(seconds as u64);
    format!("Start {} timer", humantime::format_duration(duration))
}

fn render_countdown(id: &str, num_seconds: i32) -> Markup {
    let hours = num_seconds / 3600;
    let minutes = (num_seconds % 3600) / 60;
    let seconds = num_seconds % 60;

    html! {
        div class="countdown-container hidden flex flex-col gap-2 items-center" {
            div id=(id) class="countdown font-mono text-2xl" {
                span style=(format!("--value:{hours};")) aria-live="polite" aria-label=(hours) { (hours) }
                ":"
                span style=(format!("--value:{minutes}; --digits: 2;")) aria-live="polite" aria-label=(minutes) { (minutes) }
                ":"
                span style=(format!("--value:{seconds}; --digits: 2;")) aria-live="polite" aria-label=(seconds) { (seconds) }
            }
            div class="flex gap-2 w-full justify-center" {
                button class="timer-play hidden btn btn-sm btn-soft btn-success btn-square"
                    _="on click
                       add .hidden to me
                       remove .hidden from next <button/>
                       call playTimer(event)"
                { (icon_play()) }

                button class="timer-pause btn btn-sm btn-soft btn-warning btn-square"
                    _="on click
                       add .hidden to me
                       remove .hidden from previous <button/>
                       call pauseTimer(event)"
                { (icon_pause()) }

                button class="timer-stop btn btn-sm btn-soft btn-error btn-square"
                    _="on click
                       remove .hidden from previous <button.timer-pause/>
                       add .hidden to previous <button.timer-play/>
                       add .hidden to closest .countdown-container
                       remove .hidden from the first <button/> in closest .timer-container
                       call stopTimer(event)"
                { (icon_stop()) }

                button class="timer-end btn btn-sm btn-soft btn-error btn-square btn-block hidden"
                    _="on click
                       add .hidden to me
                       remove .hidden from previous <button.timer-pause/>
                        remove .hidden from previous <button.timer-stop/>
                       add .hidden to previous <button.timer-play/>
                       add .hidden to closest .countdown-container
                       remove .hidden from the first <button/> in closest .timer-container
                       call stopTimer(event)"
                { "End" }
            }
        }
    }
}

/// Renders the favourite button.
pub fn render_favourite_button(
    recipe_id: i64,
    is_favourite: bool,
    is_deletable: bool,
    is_view_recipe: bool,
) -> Markup {
    let id = format!("favourite-{recipe_id}");

    let class = if is_view_recipe {
        "mr-2 hidden sm:block hover:text-secondary"
    } else {
        "btn btn-square btn-sm rounded-md absolute top-2 right-2 cursor-default hover:text-secondary"
    };

    let (hx_target, hx_swap) = if is_deletable && is_favourite && !is_view_recipe {
        ("closest section".to_string(), "delete".to_string())
    } else {
        (format!("#{id}"), "outerHTML".to_string())
    };

    html! {
        button id=(id) class=(class) title="Add to favourites"
                hx-post=(format!("/recipes/{recipe_id}/favourite"))
                hx-target=(hx_target)
                hx-swap=(hx_swap)
                hx-push-url="false"
                hx-vals=(json!({
                    "view-recipe": is_view_recipe
                }))
                aria-label="Add to favorites"
                aria-pressed=(is_favourite.to_string())
                _="on mousedown halt the event" {
            (icon_heart(is_favourite))
        }
    }
}
