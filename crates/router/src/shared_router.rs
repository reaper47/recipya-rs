use axum::Router;
use axum::routing::get;

use crate::handlers::shared::share_recipe_handler;
use app::state::AppState;

/// Defines the routes for shared resources.
pub(super) fn shared_routes() -> Router<AppState> {
    Router::new().route("/r/{:link}", get(share_recipe_handler))
}

#[cfg(test)]
mod tests {
    use models::recipe::structs::recipe::RecipeForCreate;
    use uuid::Uuid;

    use axum_test::TestResponse;
    use models::share::ShareRecipe;
    use models::{Recipe, recipe::structs::test_utils::a_complete_recipe_for_create};
    use testing::utils::{
        TestDb, assert_html, assert_not_in_html, build_server_anonymous, build_server_logged_in,
        create_app_state, insert_other_user,
    };

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

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
        let recipe_id = Recipe::create(&state.mm, 1, &a_complete_recipe_for_create()).await?;
        let share = ShareRecipe::new(&state.mm, recipe_id, 1, None).await?;

        let res = server.get(&base_uri(share.link)).await;

        res.assert_status_ok();
        assert_complete_recipe(res.clone(), a_complete_recipe_for_create());
        assert_html(
            res.clone(),
            vec![
                r##"<fieldset class="fieldset"><legend>Servings</legend><input id="yield" type="number" min="1" name="yield" value="4" class="input md:max-w-24" hx-get="/recipes/1/scale" hx-trigger="input" hx-target="#ingredients-instructions-container"></fieldset>"##,
            ],
        );
        assert_not_in_html(
            res,
            vec![
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
        let recipe_id = Recipe::create(&state.mm, user.id, &a_complete_recipe_for_create()).await?;
        let share = ShareRecipe::new(&state.mm, recipe_id, user.id, None).await?;

        let res = server.get(&base_uri(share.link)).await;

        res.assert_status_ok();
        assert_html(
            res,
            vec![
                r#"<button class="mr-2" title="Add recipe to collection" hx-get="/recipes/1/share" hx-push-url="true">"#,
                r##"<fieldset class="fieldset"><legend>Servings</legend><input id="yield" type="number" min="1" name="yield" value="4" class="input md:max-w-24" hx-get="/recipes/1/scale" hx-trigger="input" hx-target="#ingredients-instructions-container"></fieldset>"##,
            ],
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_shared_anonymous_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let server = build_server_anonymous(config.clone()).await?;
        let state = create_app_state(config).await;
        let recipe_id = Recipe::create(&state.mm, 1, &a_complete_recipe_for_create()).await?;
        let share = ShareRecipe::new(&state.mm, recipe_id, 1, None).await?;

        let res = server.get(&base_uri(share.link)).await;

        res.assert_status_ok();
        assert_complete_recipe(res.clone(), a_complete_recipe_for_create());
        assert_html(
            res.clone(),
            vec![
                r#"<button class="mr-2" title="Add recipe to collection" hx-get="/recipes/1/share" hx-push-url="true">"#,
                "<p class=\"text-xs\">Nutrition Facts\nPer 100g: calories 300 kcal; total carbohydrates 55g; sugar 43g; protein 7g; total fat 6g; saturated fat 1g; unsaturated fat 2g; trans fat 3g; cholesterol 5mg; sodium 12mg; fiber 10g</p>",
            ],
        );
        assert_not_in_html(
            res,
            vec![
                r##"<a id="duplicate-recipe" title="Duplicate recipe" hx-push-url="/recipes/add/manual" hx-get="/recipes/1/duplicate" hx-target="#content">"##,
                r##"<button class="ml-2 hidden sm:block" title="Edit recipe" hx-get="/recipes/1/edit" hx-push-url="true" hx-target="#content" hx-swap="innerHTML transition:true">"##,
                r##"<button title="Share recipe" class="mr-2 hidden sm:block" hx-post="/recipes/1/share" hx-target="#share-dialog-result""##,
                r##"<button title="Delete recipe" class="mr-2 hidden sm:block" hx-delete="/recipes/1" hx-swap="none" hx-confirm="Are you sure you wish to delete this recipe?" hx-indicator="#fullscreen-loader">"##,
            ],
        )?;
        Ok(())
    }

    fn assert_complete_recipe(res: TestResponse, recipe: RecipeForCreate) {
        assert_html(
            res,
            vec![
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
                r#"<div id="media-0" class="carousel-item relative w-full"><img style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src="/data/images/Placeholders/placeholder.recipe.webp">"#,
                r#"<img style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src="/data/images/Placeholders/placeholder.recipe.webp">"#,
                r#"<div class="badge badge-primary badge-outline">dinner</div>"#,
                r#"<div class="badge badge-sm badge-neutral m-1 flex-auto">tofu</div><div class="badge badge-sm badge-neutral m-1 flex-auto">vegetarian</div>"#,
                r#"<a class="btn btn-sm btn-outline no-underline print:hidden" href="https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/" target="_blank">Source</a>"#,
                r#"<textarea readonly class="textarea textarea-ghost w-full h-full resize-none rounded-none focus:outline-none">This is the most delicious recipe!</textarea>"#,
                "<p class=\"text-xs\">Nutrition Facts\nPer 100g: calories 300 kcal; total carbohydrates 55g; sugar 43g; protein 7g; total fat 6g; saturated fat 1g; unsaturated fat 2g; trans fat 3g; cholesterol 5mg; sodium 12mg; fiber 10g</p>",
                r#"<div class="grid grid-flow-col border-gray-700 col-span-6 py-1 md:border-b md:row-span-1 print:border-none md:grid-cols-4"><div class="contents grid grid-flow-col md:col-span-6"><div class="flex justify-self-center items-center gap-1 cursor-default" title="Prep time">"#,
                r#"<table class="table table-zebra table-xs print:hidden"><thead><tr><select class="select select-sm" onchange="filterNutritionRows(this.value)"><option value="per-100g">Nutrition (per 100g)</option><option value="per-serving">Nutrition (per serving)</option></select><th>Amount</th></tr></thead><tbody><tr data-nutrition-type="per-100g"><td>Calories:</td><td>300 kcal</td></tr><tr data-nutrition-type="per-100g"><td>Total carbs:</td><td>55 g</td></tr><tr data-nutrition-type="per-100g"><td>Sugars:</td><td>43 g</td></tr><tr data-nutrition-type="per-100g"><td>Protein:</td><td>7 g</td></tr><tr data-nutrition-type="per-100g"><td>Total fat:</td><td>6 g</td></tr><tr data-nutrition-type="per-100g"><td>Saturated fat:</td><td>1 g</td></tr><tr data-nutrition-type="per-100g"><td>Unsaturated fat:</td><td>2 g</td></tr><tr data-nutrition-type="per-100g"><td>Trans fat:</td><td>3 g</td></tr><tr data-nutrition-type="per-100g"><td>Cholesterol:</td><td>5 mg</td></tr><tr data-nutrition-type="per-100g"><td>Sodium:</td><td>12 mg</td></tr><tr data-nutrition-type="per-100g"><td>Fiber:</td><td>10 g</td></tr><tr data-nutrition-type="per-serving"><td>Serving size:</td><td>2 buns</td></tr><tr data-nutrition-type="per-serving"><td>Calories:</td><td>240 kcal</td></tr><tr data-nutrition-type="per-serving"><td>Total carbs:</td><td>30 g</td></tr><tr data-nutrition-type="per-serving"><td>Sugars:</td><td>24 g</td></tr><tr data-nutrition-type="per-serving"><td>Protein:</td><td>4 g</td></tr><tr data-nutrition-type="per-serving"><td>Total fat:</td><td>6 g</td></tr><tr data-nutrition-type="per-serving"><td>Saturated fat:</td><td>2 g</td></tr><tr data-nutrition-type="per-serving"><td>Unsaturated fat:</td><td>7 g</td></tr><tr data-nutrition-type="per-serving"><td>Trans fat:</td><td>2 g</td></tr><tr data-nutrition-type="per-serving"><td>Cholesterol:</td><td>12 mg</td></tr><tr data-nutrition-type="per-serving"><td>Sodium:</td><td>100 mg</td></tr><tr data-nutrition-type="per-serving"><td>Fiber:</td><td>18 g</td></tr></tbody></table>"#,
                r#"h1 class="text-sm print:mb-1"><b>Tools</b></h1><ol class="col-span-6 w-full mb-4" style="column-count: 1"><li class="text-sm"><label class="flex items-center w-full"><input type="checkbox"></label><span class="pl-2">1wok</span></li><li class="text-sm"><label class="flex items-center w-full"><input type="checkbox"></label><span class="pl-2">1frying pan</span></li></ol>"#,
                r#"<div class="col-span-6 px-8 py-2 border-gray-700 md:rounded-bl-none md:col-span-4 print:hidden"><h2 class="font-semibold text-center underline pb-1">Instructions</h2><ol class="grid list-decimal"><li class="min-w-full py-2 select-none hover:bg-base-300"><div class="flex"><div class="whitespace-pre-line w-full" _="on mousedown toggle .line-through">Mix all these ingredients</div></div></li><li class="min-w-full py-2 select-none hover:bg-base-300"><div class="flex"><div class="whitespace-pre-line w-full" _="on mousedown toggle .line-through">Turn the oven at 300 F</div></div></li><li class="min-w-full py-2 select-none hover:bg-base-300"><div class="flex"><div class="whitespace-pre-line w-full" _="on mousedown toggle .line-through">Soak the chicken in the lemon juice</div></div></li><li class="min-w-full py-2 select-none hover:bg-base-300"><div class="flex"><div class="whitespace-pre-line w-full" _="on mousedown toggle .line-through">Bake for 35 minutes</div><div id="timer-container-2" class="timer-container"><button class="timer btn btn-sm btn-circle btn-ghost" title="Start 35m timer" _="on click add .hidden to me"#,
                r#"<h1 class="text-sm print:mb-1"><b>Ingredients</b></h1><ol class="col-span-6 w-full print:mb-2" style="column-count: 1"><li class="text-sm"><label><input type="checkbox"></label><span class="pl-2">"#,
            ],
        )
    }
}
