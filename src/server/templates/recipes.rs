use maud::{html, Markup};
use url::Url;

use crate::core::config::DataDir;
use crate::core::model::RecipeDetails;
use crate::server::templates::data::{is_file_exists, Data};
use crate::server::templates::icons::{
    icon_bulb_on, icon_clock, icon_cooking_pot, icon_cutting_board, icon_document_duplicate,
    icon_ellipsis_vertical, icon_pencil, icon_plus_circle, icon_printer, icon_share, icon_trash,
};
use crate::server::templates::layouts;

/// Renders the details of a recipe.
pub fn view_recipe(data_dir: DataDir, data: Data) -> Markup {
    if data.is_hx_request {
        html! {
            title hx-swap-oob="true" {
                 (data.view.recipe_details.recipe.name) " | Recipya"
            }
            (view_recipe_helper(data_dir, &data))
        }
    } else {
        html! {
            (layouts::main(
                &data.view.recipe_details.recipe.name,
                &data,
                view_recipe_helper(data_dir, &data)
            ))
        }
    }
}

fn view_recipe_helper(data_dir: DataDir, data: &Data) -> Markup {
    let recipe_id = data.view.recipe_details.recipe.id;
    let recipe_details = &data.view.recipe_details;
    let recipe = &recipe_details.recipe;

    html! {
        @if data.share.is_shared {
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

        section class="p-2" {
            div class="flex justify-center" {
                div class="card card-border bg-base-100 shadow-none w-full border-gray-700 xl:w-[72rem] print:rounded-none" {
                    div class="card-body" style="padding: 0" {
                        (view_recipe_header(recipe_id, &data))
                        div class="grid md:grid-flow-col md:grid-cols-6" {
                            (view_recipe_media(&recipe_details, &data_dir))
                            div class="grid grid-cols-3 col-span-3 md:grid-flow-row md:grid-rows-4 print:grid-rows-2" style="grid-template-rows: auto" {
                                div class="grid grid-flow-col col-span-6 md:row-span-1 md:border-y md:border-gray-700 print:row-span-1 print:grid-cols-2 print:border-b-black print:border" {
                                    div class="col-span-2 grid place-items-center md:col-span-1 print:col-span-1 print:float-left print:ml-2 print:border-r print:border-black" {
                                        div class="badge badge-primary badge-outline" {
                                            (recipe_details.category)
                                        }
                                    }
                                    div class="grid col-span-2 border-gray-700 place-items-center text-sm border-x p-2 md:p-2 md:col-span-1 print:hidden" {
                                        @if data.is_authenticated {
                                            form autocomplete="off" _="on submit halt the event" class="print:hidden" {
                                                fieldset class="fieldset" {
                                                    legend { "Servings" }
                                                    label class="label" for="yield" { "Servings" }
                                                    input id="yield"
                                                        type="number"
                                                        min="1"
                                                        name="yield"
                                                        value=(if recipe.yield_ == 0 {
                                                            String::from("1")
                                                        } else {
                                                            recipe.yield_.to_string()
                                                        })
                                                        class="input"
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
                                    p class="hidden p-0 pt-2 print:grid print:text-center print:place-content-center" {
                                        (recipe.yield_.to_string()) " servings"
                                    }
                                    div class="flex items-center justify-center col-span-2 text-sm md:col-span-1 print:hidden" {
                                        @if let Some(source) = &recipe.source {
                                           @if Url::parse(source).is_ok() {
                                                a class="btn btn-sm btn-outline no-underline print:hidden" href=(source) target="_blank" { "Source" }
                                                p class="hidden print:block print:whitespace-nowrap print:overflow-hidden print:text-ellipsis print:max-w-xs" { (source) }
                                           } @else {
                                               p class="text-center" { "Source: " (source) }
                                           }
                                        } @else {
                                            p class="text-center" { "Source: Unknown" }
                                        }
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
                                        "grid grid-flow-col border-gray-700 col-span-6 py-1 md:border-y md:grid-cols-3 md:row-span-1 print:border-none"
                                        @if recipe_details.nutrition.is_none() { " print:hidden" }
                                    } {
                                    div class="flex justify-self-center items-center gap-1 cursor-default" title="Prep time" {
                                        (icon_cutting_board())
                                        time datetime=(data.view.formatted_times.prep_datetime) { (data.view.formatted_times.prep) }
                                    }
                                    div class="flex justify-self-center items-center gap-1 cursor-default" title="Cooking time" {
                                        (icon_cooking_pot())
                                        time datetime=(data.view.formatted_times.cook_datetime) { (data.view.formatted_times.cook) }
                                    }
                                    div class="flex justify-self-center items-center gap-1 cursor-default" title="Total time" {
                                        (icon_clock())
                                        time datetime=(data.view.formatted_times.total_datetime) { (data.view.formatted_times.total) }
                                    }
                                }
                                (view_recipe_nutrition(&recipe_details))
                                @if let Some(description) = &recipe.description {
                                    div class="col-span-3 min-h-40 md:h-full md:row-span-1 print:hidden" {
                                        label {
                                            textarea class="textarea w-full h-full resize-none" readonly {
                                                (description)
                                            }
                                        }
                                    }
                                } @else {
                                    div class="col-span-3 min-h-40 md:h-full md:row-span-1 hidden print:hidden" {}
                                }
                            }
                        }
                        @if let Some(description) = &recipe.description {
                            div class="hidden print:block print:mx-2" {
                                h1 class="text-sm print:mb-1" {
                                    b { "Description" }
                                }
                                (description)
                            }
                        }
                        div class="border-gray-700 md:border-t" {
                            (ingredients_instructions(&recipe_details))
                            div class="hidden print:grid col-span-6 ml-2 my-1" {
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
                                                label {
                                                    input type="checkbox";
                                                }
                                                span class="pl-2" {
                                                    (t.quantity.to_string()) (t.name)
                                                }
                                            }
                                        }
                                    }
                                }
                                h1 class="text-sm print:mb-1" {
                                    b { "Ingredients" }
                                }
                                ol class="col-span-6 w-full print:mb-2"
                                    style=(if recipe_details.ingredients.len() > 10 {
                                        "column-count: 2"
                                    } else {
                                        "column-count: 1"
                                    }) {
                                    @for (section, ingredients) in recipe_details.ingredients.iter() {
                                        @for ing in ingredients.iter() {
                                            li class="text-sm" {
                                                label {
                                                    input type="checkbox";
                                                }
                                                span class="pl-2" { (ing) }
                                            }
                                        }
                                    }
                                }
                            }
                            div class="hidden col-span-5 overflow-visible print:inline" {
                                h1 class="text-sm print:ml-2 print:mb-1" {
                                    b { "Instructions" }
                                }
                                ol class="col-span-6 list-decimal w-full ml-6" {
                                    @for (section, instructions) in recipe_details.instructions.iter() {
                                        @for ins in instructions.iter() {
                                            li class="print:mr-4" {
                                                span class="text-sm whitespace-pre-line" {
                                                    (ins)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div class="hidden print:block print:mx-2 print:mb-2 print:text-sm" {
                            h1 class="print:mb-1" {
                                b { "Source" }
                            }
                            @if let Some(source) = &recipe.source {
                                 @if Url::parse(source).is_ok() {
                                    p class="print:overflow-hidden" {
                                        (source)
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
            }
        }

        script defer src="/public/js/wakelock.min.js" {}
    }
}

fn view_recipe_header(recipe_id: i64, data: &Data) -> Markup {
    html! {
        h2 class="card-title bg-base-200 px-2 pt-2 place-content-center rounded-t-2xl print:border-b print:border-black" style="justify-content: space-between" {
            span class="grid grid-flow-col place-items-center pb-2 print:hidden" {
                button title="Toggle screen lock"
                    _="on load if not navigator.wakeLock hide me end
                           on click
                           if wakeLock wakeLock.release() then
                                add @d='M 12.276 18.55 v -0.748 a 4.79 4.79 0 0 1 1.463 -3.458 a 5.763 5.763 0 0 0 1.804 -4.21 a 5.821 5.821 0 0 0 -6.475 -5.778 c -2.779 0.307 -4.99 2.65 -5.146 5.448 a 5.82 5.82 0 0 0 1.757 4.503 a 4.906 4.906 0 0 1 1.5 3.495 v 0.747 a 1.44 1.44 0 0 0 1.44 1.439 h 2.218 a 1.44 1.44 0 0 0 1.44 -1.439 z m -1.058 0 c 0 0.209 -0.17 0.38 -0.38 0.38 h -2.22 c -0.21 0 -0.38 -0.171 -0.38 -0.38 v -0.748 c 0 -1.58 -0.664 -3.13 -1.822 -4.254 A 4.762 4.762 0 0 1 4.98 9.863 c 0.127 -2.289 1.935 -4.204 4.205 -4.455 a 4.762 4.762 0 0 1 5.3 4.727 a 4.714 4.714 0 0 1 -1.474 3.443 a 5.853 5.853 0 0 0 -1.791 4.225 v 0.746 z M 11.45 20.51 H 8.006 a 0.397 0.397 0 1 0 0 0.795 h 3.444 a 0.397 0.397 0 1 0 0 -0.794 z M 11.847 22.162 a 0.397 0.397 0 0 0 -0.397 -0.397 H 8.006 a 0.397 0.397 0 1 0 0 0.794 h 3.444 c 0.22 0 0.397 -0.178 0.397 -0.397 z z z z z z z z M 10.986 23.416 H 8.867 a 0.397 0.397 0 1 0 0 0.794 h 1.722 c 0.22 0 0.397 -0.178 0.397 -0.397 z' to #icon-bulb
                           else
                                call initWakeLock() then
                                add @d='M12.276 18.55v-.748a4.79 4.79 0 0 1 1.463-3.458 5.763 5.763 0 0 0 1.804-4.21 5.821 5.821 0 0 0-6.475-5.778c-2.779.307-4.99 2.65-5.146 5.448a5.82 5.82 0 0 0 1.757 4.503 4.906 4.906 0 0 1 1.5 3.495v.747a1.44 1.44 0 0 0 1.44 1.439h2.218a1.44 1.44 0 0 0 1.44-1.439zm-1.058 0c0 .209-.17.38-.38.38h-2.22c-.21 0-.38-.171-.38-.38v-.748c0-1.58-.664-3.13-1.822-4.254A4.762 4.762 0 0 1 4.98 9.863c.127-2.289 1.935-4.204 4.205-4.455a4.762 4.762 0 0 1 5.3 4.727 4.714 4.714 0 0 1-1.474 3.443 5.853 5.853 0 0 0-1.791 4.225v.746zM11.45 20.51H8.006a.397.397 0 1 0 0 .795h3.444a.397.397 0 1 0 0-.794zM11.847 22.162a.397.397 0 0 0-.397-.397H8.006a.397.397 0 1 0 0 .794h3.444c.22 0 .397-.178.397-.397zM.397 10.125h2.287a.397.397 0 1 0 0-.794H.397a.397.397 0 1 0 0 .794zM19.456 9.728a.397.397 0 0 0-.397-.397h-2.287a.397.397 0 1 0 0 .794h2.287c.22 0 .397-.178.397-.397zM9.331.397v2.287a.397.397 0 1 0 .794 0V.397a.397.397 0 1 0-.794 0zM16.045 2.85 14.43 4.465a.397.397 0 1 0 .561.561l1.617-1.617a.397.397 0 1 0-.562-.56zM5.027 14.429a.397.397 0 0 0-.56 0l-1.618 1.616a.397.397 0 1 0 .562.562l1.617-1.617a.397.397 0 0 0 0-.561zM4.466 5.027a.396.396 0 0 0 .562 0 .397.397 0 0 0 0-.56L3.41 2.848a.397.397 0 1 0-.561.561zM16.045 16.607a.396.396 0 0 0 .562 0 .397.397 0 0 0 0-.562L14.99 14.43a.397.397 0 1 0-.561.56zM10.986 23.416a.397.397 0 0 0-.397-.397H8.867a.397.397 0 1 0 0 .794h1.722c.22 0 .397-.178.397-.397z' to #icon-bulb
                           end" {
                    (icon_bulb_on())
                }

                @if data.is_authenticated && data.share.is_from_host {
                    button class="ml-2 hidden sm:block"
                        title="Edit recipe"
                        hx-get=(format!("/recipes/{recipe_id}/edit"))
                        hx-push-url="true"
                        hx-target="#content"
                        hx-swap="innerHTML transition:true" {
                        (icon_pencil())
                    }
                }
            }
            span class="text-center pb-2 print:w-full" itemprop="name" {
                    (data.view.recipe_details.recipe.name)
            }
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
                            a title="Edit recipe"
                                hx-get=(format!("/recipes/{recipe_id}/edit"))
                                hx-push-url="true"
                                hx-target="#content"
                                hx-swap="innerHTML transition:true" {
                                (icon_pencil())
                                "Edit"
                            }
                        }
                        @if !data.share.is_shared {
                            li {
                                a title="Share recipe"
                                    hx-post=(format!("/recipes/{recipe_id}/share"))
                                    hx-target="#share-dialog-result"
                                    _="on htmx:afterRequest from me
                                            if event.detail.successful
                                                if navigator.canShare
                                                    set name to document.querySelector('[itemprop=name]').textContent then
                                                    set data to {title: name, text: name, url: document.querySelector('#share-dialog-result input').value} then
                                                    call navigator.share(data)
                                                else
                                                    call share_dialog.showModal()
                                            end" {
                                    (icon_share())
                                    "Share"
                                }
                            }
                        }
                        li {
                            a title="Duplicate recipe"
                                hx-push-url="/recipes/add/manual"
                                hx-get=(format!("/recipes/{recipe_id}/duplicate"))
                                hx-target="#content" {
                                (icon_document_duplicate())
                                "Duplicate"
                            }
                        }
                        li title="Print recipe" _="on click print()" {
                            a {
                                (icon_printer())
                                "Print"
                            }
                        }
                        @if data.share.is_from_host {
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
                @if data.share.is_shared {
                    @if !data.share.is_from_host {
                        button class="mr-2"
                            title="Add recipe to collection"
                            hx-get=(format!("/recipes/{recipe_id}/share/add"))
                            hx-push-url="true" {
                            (icon_plus_circle())
                        }
                    }
                } @else {
                    button title="Share recipe" class="mr-2 hidden sm:block"
                        hx-post=(format!("/recipes/{recipe_id}/share"))
                        hx-target="#share-dialog-result"
                        _="on htmx:afterRequest from me
                            if event.detail.successful
                                if navigator.canShare
                                    set name to document.querySelector('[itemprop=name]').textContent then
                                    set data to {title: name, text: name, url: document.querySelector('#share-dialog-result input').value} then
                                    call navigator.share(data)
                                else
                                    call share_dialog.showModal()
                            end" {
                        (icon_share())
                    }
                }
                button class="mr-2 hidden sm:block" title="Duplicate recipe" hx-push-url="/recipes/add/manual" hx-get=(format!("/recipes/{recipe_id}/duplicate")) hx-target="#content" {
                    (icon_document_duplicate())
                }
                button class="mr-2 hidden sm:block" title="Print recipe" _="on click print()" {
                    (icon_printer())
                }
                @if data.share.is_from_host {
                    button title="Delete recipe"
                        class="mr-2 hidden sm:block"
                        hx-delete=(format!("/recipes/{recipe_id}"))
                        hx-swap="none"
                        hx-confirm="Are you sure you wish to delete this recipe?"
                        hx-indicator="#fullscreen-loader" {
                        (icon_trash())
                    }
                }
            }
        }
    }
}

fn view_recipe_media(recipe_details: &RecipeDetails, data_dir: &DataDir) -> Markup {
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
                        } @else if is_file_exists(video.video, &data_dir.videos) {
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
                                @if is_file_exists(img, &data_dir.images) {
                                     img style="object-fit: cover"
                                    alt="Image of the recipe"
                                    class="w-full max-h-80 md:max-h-[34rem]"
                                    src=(format!("/data/images/{img}.webp"));
                                } @else {
                                      img style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]"
                                          src="/data/images/Placeholders/placeholder.recipe.webp";
                                }
                                div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2" {
                                    a class="btn btn-circle"
                                        href=(if idx == 0 {
                                            format!("#media-{}", recipe_details.num_media() - 1)
                                        } else {
                                            format!("#media-{}", idx-1)
                                        }) {
                                        "❮"
                                    }
                                    a class="btn btn-circle"
                                      href=(if idx == recipe_details.num_media() - 1 {
                                            String::from("#media-0")
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
                                } @else if is_file_exists(v.video, &data_dir.videos) {
                                    video controls preload="metadata" src=(format!("/data/videos/{}.webm", v.video)) type="video/webm" {}
                                } @else {
                                    p class="grid place-self-center" {
                                        (format!("Video #{} is currently being processed.", idx+1))
                                        br;
                                        "Please refresh the page later."
                                    }
                                }
                                div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2" {
                                    a class="btn btn-circle" href=(format!("#media-{}", idx.checked_add(recipe_details.num_images()).and_then(|val| val.checked_sub(1)).unwrap_or(1))) { "❮" }
                                    a class="btn btn-circle"
                                      href=(if idx == recipe_details.num_videos() - 1 {
                                            String::from("#media-0")
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

fn view_recipe_nutrition(recipe_details: &RecipeDetails) -> Markup {
    html! {
        div class={
                "grid grid-flow-col border-gray-700 border-y col-span-6 md:border-t-0 md:row-span-2 print:border-none"
                @if recipe_details.nutrition.is_none() { " print:hidden" }
            } {
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
                        tr {
                            td { "Calories:" }
                            td { (nutrition.calories_kcal.map(|v| format!("{v} kcal")).unwrap_or(String::from("-"))) }
                        }
                        tr {
                            td { "Total carbs:" }
                            td { (nutrition.total_carbohydrates.map(|v| format!("{v} g")).unwrap_or(String::from("-"))) }
                        }
                        tr {
                            td { "Sugars:" }
                            td { (nutrition.sugars_g.map(|v| format!("{v} g")).unwrap_or(String::from("-"))) }
                        }
                        tr {
                            td { "Protein:" }
                            td { (nutrition.protein_g.map(|v| format!("{v} g")).unwrap_or(String::from("-"))) }
                        }
                        tr {
                            td { "Total fat:" }
                            td { (nutrition.total_fat_g.map(|v| format!("{v} g")).unwrap_or(String::from("-"))) }
                        }
                        tr {
                            td { "Saturated fat:" }
                            td { (nutrition.saturated_fat_g.map(|v| format!("{v} g")).unwrap_or(String::from("-"))) }
                        }
                        tr {
                            td { "Unsaturated fat:" }
                            td { (nutrition.unsaturated_fat_g.map(|v| format!("{v} g")).unwrap_or(String::from("-"))) }
                        }
                        tr {
                            td { "Trans fat:" }
                            td { (nutrition.trans_fat_g.map(|v| format!("{v} g")).unwrap_or(String::from("-"))) }
                        }
                        tr {
                            td { "Cholesterol:" }
                            td { (nutrition.cholesterol_mg.map(|v| format!("{v} mg")).unwrap_or(String::from("-"))) }
                        }
                        tr {
                            td { "Sodium:" }
                            td { (nutrition.sodium_mg.map(|v| format!("{v} mg")).unwrap_or(String::from("-"))) }
                        }
                        tr {
                            td { "Fiber:" }
                            td { (nutrition.fiber_g.map(|v| format!("{v} g")).unwrap_or(String::from("-"))) }
                        }
                    } @else {
                        tr {
                            td { "Calories:" }
                            td { "-" }
                        }
                        tr {
                            td { "Total carbs:" }
                            td { "-" }
                        }
                        tr {
                            td { "Sugars:" }
                            td { "-" }
                        }
                        tr {
                            td { "Protein:" }
                            td { "-" }
                        }
                        tr {
                            td { "Total fat:" }
                            td { "-" }
                        }
                        tr {
                            td { "Saturated fat:" }
                            td { "-" }
                        }
                        tr {
                            td { "Unsaturated fat:" }
                            td { "-" }
                        }
                        tr {
                            td { "Trans fat:" }
                            td { "-" }
                        }
                        tr {
                            td { "Cholesterol:" }
                            td { "-" }
                        }
                        tr {
                            td { "Sodium:" }
                            td { "-" }
                        }
                        tr {
                            td { "Fiber:" }
                            td { "-" }
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
}

fn ingredients_instructions(recipe: &RecipeDetails) -> Markup {
    html! {
        div id="ingredients-instructions-container" class="grid text-sm md:grid-flow-col md:col-span-6" {
            div class="col-span-6 border-gray-700 px-4 py-2 border-y md:col-span-2 md:border-r md:border-y-0 print:hidden" {
                @if !recipe.tools.is_empty() {
                    h2 class="font-semibold text-center underline pb-1" { "Tools" }
                    ul class="md:pb-2" {
                        @for tool in recipe.tools.iter() {
                            li class="hover:bg-gray-100 dark:hover:bg-gray-700" {
                                label class="label justify-start" {
                                    input type="checkbox" class="checkbox";
                                    span class="pl-2" { (tool.quantity.to_string()) (tool.name) }
                                }
                            }
                        }
                    }
                }
                h2 class="font-semibold text-center underline pb-1" { "Ingredients" }
                ul {
                    @for (_section, ingredients) in recipe.ingredients.iter() {
                        @for ingredient in ingredients.iter() {
                             li class="hover:bg-gray-100 dark:hover:bg-gray-700" {
                                label class="label justify-start" {
                                    input type="checkbox" class="checkbox";
                                    span class="pl-2" { (ingredient) }
                                }
                            }
                        }
                    }
                }
            }
            div class="col-span-6 px-8 py-2 border-gray-700 md:rounded-bl-none md:col-span-4 print:hidden" {
                h2 class="font-semibold text-center underline pb-1" { "Instructions" }
                ol class="grid list-decimal" {
                    @for (_section, instruction) in recipe.instructions.iter() {
                        @for instruction in instruction.iter() {
                            li class="min-w-full py-2 select-none hover:bg-gray-100 dark:hover:bg-gray-700" _="on mousedown toggle .line-through" {
                                span class="whitespace-pre-line" { (instruction) }
                            }
                        }
                    }
                }
            }
        }
    }
}
