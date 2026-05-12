use axum::Router;
use axum::routing::get;

use app::state::AppState;

use crate::handlers::shared::{share_recipe_handler, share_shopping_list_handler};

/// Defines the routes for shared resources.
#[allow(clippy::literal_string_with_formatting_args)]
pub fn shared_routes() -> Router<AppState> {
    Router::new()
        .route("/r/{:link}", get(share_recipe_handler))
        .route("/sl/{:list_id}", get(share_shopping_list_handler))
}

#[cfg(test)]
mod tests {
    use models::user::User;
    use models::{Recipe, recipe::structs::recipe::RecipeForCreate};
    use test_db::TestDb;
    use test_models::a_complete_recipe_for_create;
    use uuid::Uuid;

    use axum_test::TestResponse;
    use models::share::ShareRecipe;
    use test_fixtures::{RecipeImages, assert_html, assert_not_in_html};
    use test_utils::{
        build_server_anonymous, build_server_logged_in, create_app_state, insert_other_user,
    };

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_recipe {
        use super::*;

        fn base_uri(link: Uuid) -> String {
            format!("/shared/r/{link}")
        }

        #[tokio::test]
        async fn test_recipe_not_shared_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config).await?;

            let res = server.get(&base_uri(Uuid::new_v4())).await;

            res.assert_status_not_found();
            Ok(())
        }

        #[tokio::test]
        async fn test_shared_logged_in_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let users = User::all(&state.mm).await?;
            let (recipe1, images) = a_complete_recipe_for_create();
            let recipe_id = Recipe::create(&state.mm, users[0].id, &recipe1).await?;
            let share = ShareRecipe::new(&state.mm, recipe_id, users[0].id, None).await?;

            let res = server.get(&base_uri(share.link)).await;

            res.assert_status_ok();
            let (recipe, _) = a_complete_recipe_for_create();
            assert_complete_recipe(&res, &recipe, &images);
            assert_html(
                &res,
                &[
                    r##"<fieldset class="fieldset"><legend>Servings</legend><input id="yield" type="number" min="1" name="yield" value="4" class="input max-w-18 md:max-w-24" hx-get="/recipes/1/scale" hx-trigger="input" hx-target="#ingredients-instructions-container"></fieldset>"##,
                ],
            );
            assert_not_in_html(
                &res,
                &[
                    r#"<button class="mr-2" title="Add recipe to collection" hx-get="/recipes/1/share" hx-push-url="true">"#,
                    r##"<a id="duplicate-recipe" title="Duplicate recipe" hx-push-url="/recipes/add/manual" hx-get="/recipes/1/duplicate" hx-target="#content">"##,
                    r##"#<button class="ml-2 hidden sm:block" title="Edit recipe" hx-get="/recipes/1/edit" hx-push-url="true" hx-target="#content" hx-swap="innerHTML transition:true">"##,
                    r##"<button title="Share recipe" class="mr-2 hidden sm:block" hx-post="/recipes/1/share" hx-target="#share-dialog-result""##,
                    r##"<button title="Delete recipe" class="mr-2 hidden sm:block" hx-delete="/recipes/1" hx-swap="none" hx-confirm="Are you sure you wish to delete this recipe?" hx-indicator="#fullscreen-loader">"##,
                ],
            )?;
            Ok(())
        }

        #[tokio::test]
        async fn test_shared_logged_in_other_user_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let user = insert_other_user(config.clone(), "slava@ukraini.ua").await?;
            let state = create_app_state(config).await;
            let (recipe, _) = a_complete_recipe_for_create();
            let recipe_id = Recipe::create(&state.mm, user.id, &recipe).await?;
            let share = ShareRecipe::new(&state.mm, recipe_id, user.id, None).await?;

            let res = server.get(&base_uri(share.link)).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    r#"<button class="mr-2" title="Add recipe to collection" hx-get="/recipes/1/share" hx-push-url="true">"#,
                    r##"<fieldset class="fieldset"><legend>Servings</legend><input id="yield" type="number" min="1" name="yield" value="4" class="input max-w-18 md:max-w-24" hx-get="/recipes/1/scale" hx-trigger="input" hx-target="#ingredients-instructions-container"></fieldset>"##,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_shared_anonymous_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config.clone()).await?;
            let state = create_app_state(config).await;
            let users = User::all(&state.mm).await?;
            let (recipe, images) = a_complete_recipe_for_create();
            let recipe_id = Recipe::create(&state.mm, users[0].id, &recipe).await?;
            let share = ShareRecipe::new(&state.mm, recipe_id, users[0].id, None).await?;

            let res = server.get(&base_uri(share.link)).await;

            res.assert_status_ok();
            let (recipe, _) = a_complete_recipe_for_create();
            assert_complete_recipe(&res, &recipe, &images);
            assert_html(
                &res,
                &[
                    r#"<button class="mr-2" title="Add recipe to collection" hx-get="/recipes/1/share" hx-push-url="true">"#,
                    "<p class=\"text-xs\">Nutrition Facts\nPer 100g: calories 300 kcal; total carbohydrates 55g; sugar 43g; protein 7g; total fat 6g; saturated fat 1g; unsaturated fat 2g; trans fat 3g; cholesterol 5mg; sodium 12mg; fiber 10g</p>",
                ],
            );
            assert_not_in_html(
                &res,
                &[
                    r##"<a id="duplicate-recipe" title="Duplicate recipe" hx-push-url="/recipes/add/manual" hx-get="/recipes/1/duplicate" hx-target="#content">"##,
                    r##"<button class="ml-2 hidden sm:block" title="Edit recipe" hx-get="/recipes/1/edit" hx-push-url="true" hx-target="#content" hx-swap="innerHTML transition:true">"##,
                    r##"<button title="Share recipe" class="mr-2 hidden sm:block" hx-post="/recipes/1/share" hx-target="#share-dialog-result""##,
                    r##"<button title="Delete recipe" class="mr-2 hidden sm:block" hx-delete="/recipes/1" hx-swap="none" hx-confirm="Are you sure you wish to delete this recipe?" hx-indicator="#fullscreen-loader">"##,
                ],
            )?;
            Ok(())
        }

        fn assert_complete_recipe(
            res: &TestResponse,
            recipe: &RecipeForCreate,
            images: &RecipeImages,
        ) {
            assert_html(
                res,
                &[
                    &format!(
                        "<title hx-swap-oob=\"true\">{} | Recipya</title>",
                        recipe.name
                    ),
                    r#"<button title="Toggle screen lock"#,
                    &format!(
                        "<span class=\"text-center pb-2 print:w-full\" itemprop=\"name\">{}</span>",
                        recipe.name
                    ),
                    r#"<li title="Print recipe" _="on click print()">"#,
                    r#"<iframe src="https://example.com/embed/j43yfe3.mp4" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen="" style="height: 100%;width: 100%;"></iframe>"#,
                    &format!(
                        r#"<div id="media-0" class="carousel-item relative w-full"><img style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src="/data/images/{}.webp">"#,
                        images.main
                    ),
                    &format!(
                        r#"<img style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src="/data/images/{}.webp">"#,
                        images.main
                    ),
                    r#"<div class="badge badge-primary badge-outline">dinner</div>"#,
                    r#"<div class="badge badge-sm badge-neutral m-1 flex-auto">tofu</div><div class="badge badge-sm badge-neutral m-1 flex-auto">vegetarian</div>"#,
                    r#"<a class="btn btn-sm btn-outline no-underline print:hidden" href="https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/" target="_blank"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 21a9.004 9.004 0 0 0 8.716-6.747M12 21a9.004 9.004 0 0 1-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 0 1 7.843 4.582M12 3a8.997 8.997 0 0 0-7.843 4.582m15.686 0A11.953 11.953 0 0 1 12 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0 1 21 12c0 .778-.099 1.533-.284 2.253m0 0A17.919 17.919 0 0 1 12 16.5c-3.162 0-6.133-.815-8.716-2.247m0 0A9.015 9.015 0 0 1 3 12c0-1.605.42-3.113 1.157-4.418"></path></svg>Source</a>"#,
                    r#"<textarea readonly class="textarea textarea-ghost w-full h-full resize-none rounded-none focus:outline-none">This is the most delicious recipe!</textarea>"#,
                    "<p class=\"text-xs\">Nutrition Facts\nPer 100g: calories 300 kcal; total carbohydrates 55g; sugar 43g; protein 7g; total fat 6g; saturated fat 1g; unsaturated fat 2g; trans fat 3g; cholesterol 5mg; sodium 12mg; fiber 10g</p>",
                    r#"<div class="grid grid-flow-col border-gray-700 col-span-6 py-1 md:border-b md:row-span-1 print:border-none md:grid-cols-4"><div class="contents grid grid-flow-col md:col-span-6"><div class="flex justify-self-center items-center gap-1 cursor-default" title="Prep time">"#,
                    r#"<table class="table table-zebra table-xs print:hidden"><thead><tr><select class="select select-sm" onchange="filterNutritionRows(this, this.value)"><option value="per-100g">Nutrition (per 100g)</option><option value="per-serving">Nutrition (per serving)</option></select></tr></thead><tbody><tr data-nutrition-type="per-100g"><td>Calories:</td><td>300 kcal</td></tr><tr data-nutrition-type="per-100g"><td>Total carbs:</td><td>55g</td></tr><tr data-nutrition-type="per-100g"><td>Sugars:</td><td>43g</td></tr><tr data-nutrition-type="per-100g"><td>Protein:</td><td>7g</td></tr><tr data-nutrition-type="per-100g"><td>Total fat:</td><td>6g</td></tr><tr data-nutrition-type="per-100g"><td>Saturated fat:</td><td>1g</td></tr><tr data-nutrition-type="per-100g"><td>Unsaturated fat:</td><td>2g</td></tr><tr data-nutrition-type="per-100g"><td>Trans fat:</td><td>3g</td></tr><tr data-nutrition-type="per-100g"><td>Cholesterol:</td><td>5mg</td></tr><tr data-nutrition-type="per-100g"><td>Sodium:</td><td>12mg</td></tr><tr data-nutrition-type="per-100g"><td>Fiber:</td><td>10g</td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Serving size:</td><td></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Calories:</td><td>240 kcal</td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Total carbs:</td><td>30g</td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Sugars:</td><td>24g</td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Protein:</td><td>4g</td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Total fat:</td><td>6g</td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Saturated fat:</td><td>2g</td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Unsaturated fat:</td><td>7g</td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Trans fat:</td><td>2g</td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Cholesterol:</td><td>12mg</td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Sodium:</td><td>100mg</td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Fiber:</td><td>18g</td></tr></tbody></table>"#,
                    r#"h1 class="text-sm print:mb-1"><b>Tools</b></h1><ol class="col-span-6 w-full mb-4" style="column-count: 1"><li class="text-sm"><label class="flex items-center w-full"><input type="checkbox"></label><span class="pl-2">1wok</span></li><li class="text-sm"><label class="flex items-center w-full"><input type="checkbox"></label><span class="pl-2">1frying pan</span></li></ol>"#,
                    r#"<div class="col-span-6 px-8 py-2 border-gray-700 md:rounded-bl-none md:col-span-4 print:hidden"><h2 class="font-semibold text-center underline pb-1">Instructions</h2><div><h3 class="font-bold py-2">Sauce</h3><ol class="grid list-decimal"><li class="min-w-full py-2 select-none hover:bg-base-300" _="on mousedown toggle .line-through toggle .opacity-40 then if I match .line-through then add .invisible to .timer in me else remove .invisible from .timer in me"><div class="flex"><div class="whitespace-pre-line w-full transition-all">Mix all these ingredients</div></div></li></ol></div><div><h3 class="font-bold py-2">Chicken</h3><ol class="grid list-decimal"><li class="min-w-full py-2 select-none hover:bg-base-300" _="on mousedown toggle .line-through toggle .opacity-40 then if I match .line-through then add .invisible to .timer in me else remove .invisible from .timer in me"><div class="flex"><div class="whitespace-pre-line w-full transition-all">Turn the oven at 300 F</div></div></li><li class="min-w-full py-2 select-none hover:bg-base-300" _="on mousedown toggle .line-through toggle .opacity-40 then if I match .line-through then add .invisible to .timer in me else remove .invisible from .timer in me"><div class="flex"><div class="whitespace-pre-line w-full transition-all">Soak the chicken in the lemon juice</div></div></li><li class="min-w-full py-2 select-none hover:bg-base-300" _="on mousedown toggle .line-through toggle .opacity-40 then if I match .line-through then add .invisible to .timer in me else remove .invisible from .timer in me"><div class="flex"><div class="whitespace-pre-line w-full transition-all">Bake for 35 minutes</div><div id="timer-container-2" class="timer-container" _="on mousedown halt the event"><button class="timer btn btn-sm btn-wide btn-ghost" title="Start 35m timer" _="on click add .hidden to me"#,
                    r#"<h1 class="text-sm print:mb-1"><b>Ingredients</b></h1><div><h3 class="font-bold py-2">Sauce</h3><ul class="col-span-6 w-full print:mb-2" style="column-count: 1"><li class="text-sm"><label><input type="checkbox"></label><span class="pl-2">1 cup blue spinach</span></li><li class="text-sm"><label><input type="checkbox"></label><span class="pl-2">1/2 tbsp cinnamon</span></li></ul><h3 class="font-bold py-2">Main</h3><ul class="col-span-6 w-full print:mb-2" style="column-count: 1"><li class="text-sm"><label><input type="checkbox"></label><span class="pl-2">4 pounds top quality chicken filet</span></li><li class="text-sm"><label><input type="checkbox"></label><span class="pl-2">1/8 cup lemon juice</span></li></ul></div></div>"#,
                ],
            );
        }
    }

    mod tests_shopping_list {
        use models::shopping::{ShareShoppingList, ShoppingList, ShoppingListItemForCreate};

        use super::*;

        fn base_uri(link: Uuid) -> String {
            format!("/shared/sl/{link}")
        }

        fn an_item_c() -> ShoppingListItemForCreate {
            ShoppingListItemForCreate {
                ingredient: "Spaghetti".into(),
                quantity: None,
                label: None,
                recipe_id: None,
            }
        }

        #[tokio::test]
        async fn test_list_not_shared_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config).await?;

            let res = server.get(&base_uri(Uuid::new_v4())).await;

            res.assert_status_not_found();
            Ok(())
        }

        #[tokio::test]
        async fn test_sl_shared_no_items_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Costco", user_id).await?;
            let share = ShareShoppingList::new(&state.mm, list_id, user_id, None).await?;

            let res = server.get(&base_uri(share.link)).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    r#"<div class="p-2"><h1 class="text-center text-2xl font-bold underline p-2">Costco</h1><div class="grid"><div class="place-self-center"><p class="text-center">Shopping list has no items.</p>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_sl_shared_logged_in_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Costco", user_id).await?;
            let share = ShareShoppingList::new(&state.mm, list_id, user_id, None).await?;
            let _ = ShoppingList::add_item(&state.mm, list_id, an_item_c(), user_id).await?;

            let res = server.get(&base_uri(share.link)).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    r#"<div id="content" class="flex-1 pb-0"><div class="p-2"><h1 class="text-center text-2xl font-bold underline p-2">Costco</h1>"#,
                    &format!(
                        r#"<div class="grid"><div class="min-w-full sm:min-w-[33vw] place-self-center"><details open><summary class="text-left cursor-default">No label</summary><ol class="list bg-base-100 rounded-box shadow-md"><li class="list-row grid grid-cols-[1fr_auto]" data-item-id="1" data-drag-row><div class="grid grid-flow-col" data-drageable draggable="true"><div class="grid gap-1 min-w-0"><label class="label text-base-content"><input type="checkbox" class="checkbox peer" hx-post="/shopping/lists/{list_id}/items/1/toggle"><span class="[input:checked~&amp;]:line-through [input:checked~&amp;]:opacity-50 transition-all">Spaghetti</span></label></div></div></li></ol></details></div>"#
                    ),
                ],
            );
            assert_not_in_html(
                &res,
                &[
                    r#"<button class="btn join-item">Toggle Recipes</button>"#,
                    r#"<button class="btn join-item">View (default)</button>"#,
                    r#"<button class="btn join-item">Copy</button>"#,
                    r#"<button class="btn join-item">Export</button>"#,
                    r#"<button class="btn join-item">Upload to app</button>"#,
                    &format!(
                        r##"<button title="Share list" class="btn join-item" hx-post="/shopping/lists/{list_id}/share" hx-target="#share-dialog-result" hx-push-url="false""##
                    ),
                    &format!(
                        r#"<button title="Delete list" class="btn join-item" hx-delete="/shopping/lists/{list_id}" hx-confirm="Are you sure you wish to delete this list?"><svg xmlns="http://www.w3.org/2000/svg" class="size-6 hover:text-red-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"></path></svg></button>"#
                    ),
                ],
            )?;
            Ok(())
        }

        #[tokio::test]
        async fn test_sl_shared_anonymous_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Costco", user_id).await?;
            let share = ShareShoppingList::new(&state.mm, list_id, user_id, None).await?;
            let _ = ShoppingList::add_item(&state.mm, list_id, an_item_c(), user_id).await?;

            let res = server.get(&base_uri(share.link)).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    r#"<div id="content" class="flex-1 pb-0"><div class="p-2"><h1 class="text-center text-2xl font-bold underline p-2">Costco</h1>"#,
                    &format!(
                        r#"<div class="grid"><div class="min-w-full sm:min-w-[33vw] place-self-center"><details open><summary class="text-left cursor-default">No label</summary><ol class="list bg-base-100 rounded-box shadow-md"><li class="list-row grid grid-cols-[1fr_auto]" data-item-id="1" data-drag-row><div class="grid grid-flow-col" data-drageable draggable="true"><div class="grid gap-1 min-w-0"><label class="label text-base-content"><input type="checkbox" class="checkbox peer" hx-post="/shopping/lists/{list_id}/items/1/toggle"><span class="[input:checked~&amp;]:line-through [input:checked~&amp;]:opacity-50 transition-all">Spaghetti</span></label></div></div></li></ol></details></div>"#
                    ),
                ],
            );
            assert_not_in_html(
                &res,
                &[
                    r#"<button class="btn join-item">Toggle Recipes</button>"#,
                    r#"<button class="btn join-item">View (default)</button>"#,
                    r#"<button class="btn join-item">Copy</button>"#,
                    r#"<button class="btn join-item">Export</button>"#,
                    r#"<button class="btn join-item">Upload to app</button>"#,
                    &format!(
                        r##"<button title="Share list" class="btn join-item" hx-post="/shopping/lists/{list_id}/share" hx-target="#share-dialog-result" hx-push-url="false""##
                    ),
                    &format!(
                        r#"<button title="Delete list" class="btn join-item" hx-delete="/shopping/lists/{list_id}" hx-confirm="Are you sure you wish to delete this list?"><svg xmlns="http://www.w3.org/2000/svg" class="size-6 hover:text-red-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"></path></svg></button>"#
                    ),
                ],
            )?;
            Ok(())
        }
    }
}
