use axum::routing::get;
use axum::{middleware, Router};

use crate::server::router::handlers::recipes::{
    delete_recipe_handler, recipe_view_handler, recipes_add_handler, recipes_handler,
};
use crate::server::router::middleware::mw_auth;
use crate::server::AppState;

/// Defines the routes for endpoints related to recipes.
pub(super) fn recipes_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(recipes_handler))
        .route(
            "/{:recipe_id}",
            get(recipe_view_handler).delete(delete_recipe_handler),
        )
        .route("/add", get(recipes_add_handler))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mw_auth::mw_ctx_require,
        ))
}

#[cfg(test)]
mod tests {
    use crate::core::config::Config;
    use crate::server::test_utils::{build_server_logged_in, TestDb};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_add {
        use super::*;
        use axum_test::TestResponse;

        use crate::server::test_utils::{assert_html, assert_must_be_logged_in};

        const BASE_URI: &str = "/recipes/add";

        #[tokio::test]
        async fn test_add_recipe_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(BASE_URI).await
        }

        #[tokio::test]
        async fn test_add_recipe_is_htmx_request_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let mut server = build_server_logged_in(config).await?;
            server.add_header(axum_htmx::headers::HX_REQUEST, "true");

            let res = server.get(BASE_URI).await;

            assert_content(res);
            Ok(())
        }

        #[tokio::test]
        async fn test_add_recipe_is_not_htmx_request_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(BASE_URI).await;

            assert_content(res);
            Ok(())
        }

        fn assert_content(res: TestResponse) {
            assert_html(
                res,
                vec![
                    r#"<title hx-swap-oob="true">Add Recipe | Recipya</title>"#,
                    r#"<img class="object-cover w-full h-40 rounded-t-xl" src="/public/img/recipes/new/manual.webp" alt="Writing on a piece of paper with a traditional pen.">"#,
                    r#"<img class="object-cover w-full h-40 rounded-t-xl" src="/public/img/recipes/new/import.webp" alt="Earth connected from end-to-end by telecommunications.">"#,
                    r#"<img class="object-cover w-full h-40 rounded-t-xl" src="/public/img/recipes/new/camera.webp" alt="A cellphone used as a camera.">"#,
                    r#"<img class="object-cover w-full h-40 rounded-t-xl" src="/public/img/recipes/new/schema.webp" alt="A bunch of shipping containers on a cargo boat.">"#,
                    r##"<button class="underline" hx-get="/recipes/supported-websites" hx-target="#search-results" onclick="document.querySelector('#supported-websites-dialog').showModal()">supported</button>"##,
                    r##"<dialog id="websites-dialog" class="modal"><div class="modal-box"><form method="dialog"><button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button></form><h3 class="font-bold text-lg">Fetch recipes from websites</h3><form class="py-4" hx-post="/recipes/add/website" hx-swap="none" _="on submit call #websites-dialog.close() then set me.querySelector('textarea').value to ''"><div class="grid mb-4"><label class="floating-label"><span>Enter one or more URLs, each on a new line.</span><textarea class="textarea whitespace-pre-line" name="urls" rows="5" placeholder="URL 1"##,
                    r#"</textarea></label></div><button class="btn btn-block btn-primary btn-sm">Submit</button></form></div></dialog>"#,
                    r##"<dialog id="supported-websites-dialog" class="modal"><div class="modal-box h-2/3"><form method="dialog"><button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button></form><h3 class="mb-1"><label class="floating-label"><input type="search" placeholder="Search a website" class="input input-sm w-11/12" _="on input show <tbody>tr/> in next <table/> when its textContent.toLowerCase() contains my value.toLowerCase()"></label></h3><div class="overflow-x-auto"><table class="table table-zebra table-sm"><thead><tr class="text-center"><th class="py-1">Number</th><th class="py-1">Website</th></tr></thead><tbody id="search-results"></tbody></table></div></div></dialog>"##,
                    r##"<dialog class="modal" id="supported-apps-import-dialog"><div class="modal-box h-2/3"><form method="dialog"><button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button></form><h3 class="mb-1"><label class="floating-label"><input type="search" placeholder="Search an application" class="input input-bordered input-sm w-11/12" _="on input show <tbody>tr/> in next <table/> when its textContent.toLowerCase() contains my value.toLowerCase()"></label></h3><div class="overflow-x-auto"><table class="table table-zebra table-sm"><thead><tr class="text-center"><th class="py-1">Number</th><th class="py-1">Application</th></tr></thead><tbody id="application-results"></tbody></table></div></div></dialog>"##,
                    r##"<dialog class="modal" id="add-ocr-dialog"><div class="modal-box"><form method="dialog"><button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button></form><h3 class="font-bold text-lg">Scan Recipe</h3><form class="py-4" hx-post="/recipes/add/ocr" hx-encoding="multipart/form-data" hx-indicator="#fullscreen-loader" hx-swap="none" _="on submit call document.querySelector('#add-ocr-dialog').close()"><div class="grid mb-4"><label for="add-ocr-files-input" class="floating-label text-sm font-medium mb-1">Select your recipe's images ordered by page or a recipe document in the PDF format.</label><input id="add-ocr-files-input" type="file" name="files" accept=".jpg, .jpeg, .png, .bmp, .tiff, .heif, .pdf" multiple class="p-2 border border-gray-300 rounded-lg shadow focus:ring-2 focus:ring-purple-600 dark:bg-gray-900 dark:border-none"></div><button class="btn btn-block btn-primary btn-sm">Submit</button></form></div></dialog>"##,
                    r##"<dialog class="modal" id="import-recipes-dialog"><div class="modal-box"><form method="dialog"><button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button></form><h3 class="font-bold text-lg">Import Recipes</h3><form class="py-4" hx-post="/recipes/add/import" enctype="multipart/form-data" hx-indicator="#fullscreen-loader" hx-swap="none"><div class="grid mb-4"><label for="import-dialog-file" class="floating-label text-sm font-semibold mb-1">Choose files in the .json, .txt, .zip or other application format.</label><input id="import-dialog-file" type="file" name="files" accept=".cml,.crumb,.json,.mxp,.paprikarecipes,.txt,.zip" multiple class="p-2 border border-gray-300 rounded-lg shadow focus:ring-2 focus:ring-purple-600 dark:bg-gray-900 dark:border-none"></div><button type="submit" class="btn btn-block btn-primary btn-sm" onclick="document.querySelector('#import-recipes-dialog').close()">Submit</button></form></div></dialog>"##,
                ],
            )
        }
    }

    mod tests_recipes {
        use super::*;
        use crate::core::model::Recipe;
        use crate::server::test_utils::{a_complete_recipe, assert_html, assert_must_be_logged_in};
        use crate::server::AppState;

        const BASE_URI: &str = "/recipes";

        #[tokio::test]
        async fn test_get_recipes_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(BASE_URI).await
        }

        #[tokio::test]
        async fn test_get_recipes_user_has_no_recipes_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(BASE_URI).await;

            assert_html(
                res,
                vec![
                    r#"<title hx-swap-oob="true">Recipes | Recipya</title>"#,
                    r##"<p class="pb-2">Your recipe collection looks a bit empty at the moment.</p><p>Why not start adding recipes by clicking the <a class="underline font-semibold cursor-pointer" hx-get="/recipes/add" hx-target="#content" hx-push-url="true">Add recipe</a> button at the top?</p>"##,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_get_recipes_user_has_recipes_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = AppState::new(config).await?;
            for i in 0..3 {
                let mut recipe = a_complete_recipe();
                recipe.name.push_str(i.to_string().as_str());
                let _ = Recipe::create(&state.mm, 1, &recipe).await?;
            }

            let res = server.get(BASE_URI).await;

            assert_html(
                res,
                vec![
                    r#"<title hx-swap-oob="true">Recipes | Recipya</title>"#,
                    r##"<form class="w-72 flex md:w-96" hx-get="/recipes/search" hx-vals="{"page": 1}" hx-target="#list-recipes" hx-push-url="true" hx-trigger="submit, change target:.sort-option"><div class="w-full"><label class="input input-sm flex justify-between px-0 gap-2 z-20"><button id="search-shortcut" type="button" class="pl-2" popovertarget="search-help" _="on click toggle .hidden on #search-help"><svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 self-center" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg></button><input id="search_recipes" class="w-full" type="search" name="q" placeholder="Search for recipes..." value="""##,
                    r#"<button type="submit" class="px-2 btn btn-sm btn-primary"><svg class="w-4 h-4" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 20"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z"></path></svg><span class="sr-only">Search</span></button></label></div><div class="dropdown dropdown-left ml-1"><div tabindex="0" role="button" class="btn btn-sm p-1"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25H12"></path></svg></div><div tabindex="0" class="dropdown-content z-10 menu menu-sm p-2 shadow bg-base-200 w-52 sm:menu-md prose"><h4>Sort</h4><fieldset class="fieldset"><label class="label cursor-pointer" for="sort-opt-default">Default</label><input id="sort-opt-default" type="radio" name="sort" class="radio radio-sm sort-option" value="default"></fieldset><fieldset class="fieldset"><label class="label cursor-pointer" for="sort-opt-a-z">Name:<br>A to Z</label><input id="sort-opt-a-z" type="radio" name="sort" class="radio radio-sm sort-option" value="a-z"></fieldset><fieldset class="fieldset"><label class="label cursor-pointer" for="sort-opt-z-a">Name:<br>Z to A</label><input id="sort-opt-z-a" type="radio" name="sort" class="radio radio-sm sort-option" value="z-a"></fieldset><fieldset class="fieldset"><label class="label cursor-pointer" for="sort-opt-new-old">Date created:<br>Newest to oldest</label><input id="sort-opt-new-old" type="radio" name="sort" class="radio radio-sm sort-option" value="new-old"></fieldset><fieldset class="fieldset"><label class="label cursor-pointer" for="sort-opt-old-new">Date created:<br>Oldest to newest</label><input id="sort-opt-old-new" type="radio" name="sort" class="radio radio-sm sort-option" value="old-new"></fieldset><fieldset class="fieldset"><label class="label cursor-pointer" for="sort-opt-random">Random</label><input id="sort-opt-random" type="radio" name="sort" class="radio radio-sm sort-option" value="random"></fieldset></div></div></form>"#,
                    r#"<div id="list-recipes" class="min-h-[79vh]"><article class="grid gap-4 p-4 text-sm place-items-center grid-cols-1 sm:grid-cols-2 md:m-auto md:max-w-7xl md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 md:text-base">"#,
                    r#"<div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex">"#,
                    r##"<section class="card-side sm:card card-compact card-bordered bg-base-100 shadow-lg indicator w-full"><span class="hidden sm:block"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral indicator-item indicator-center" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "cat:dinner"}" _="on click put &quot;cat:dinner&quot; into #search_recipes.value">dinner</span></span><figure class="relative cursor-pointer" hx-get="/recipes/1" hx-target="#content" hx-push-url="true" hx-trigger="mousedown" hx-swap="innerHTML show:window:top transition:true"><img class="h-28 w-24 object-cover rounded-t-lg sm:h-40 sm:min-w-full sm:w-full" src="/data/images/Placeholders/placeholder.recipe.webp" alt="Image for the Best Chinese Kale0 recipe"><div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex"><p class="p-2 text-sm">This is the most delicious recipe!</p></div></figure><div class="card-body justify-between"><h2 class="sm:font-semibold sm:w-[25ch] sm:break-words sm:min-h-14">Best Chinese Kale0</h2><div class="sm:max-h-14 sm:overflow-y-auto sm:content-end sm:min-h-14"><div class="flex flex-col flex-wrap overflow-x-auto max-h-12 pb-2 sm:pb-0 sm:max-h-none sm:flex-auto sm:flex-row"><span class="sm:hidden"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "cat:dinner"}" _="on click put &quot;cat:dinner&quot; into #search_recipes.value">dinner</span></span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "tag":vegetarian}" _="on click put &quot;tag:vegetarian&quot; into #search-recipes.value">vegetarian</span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "tag":tofu}" _="on click put &quot;tag:tofu&quot; into #search-recipes.value">tofu</span></div></div><div class="card-actions flex-col-reverse h-fit"><button class="btn btn-block btn-xs btn-outline sm:btn-sm" hx-get="/recipes/1" hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true">View</button></div></div></section>"##,
                    r##"<section class="card-side sm:card card-compact card-bordered bg-base-100 shadow-lg indicator w-full"><span class="hidden sm:block"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral indicator-item indicator-center" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "cat:dinner"}" _="on click put &quot;cat:dinner&quot; into #search_recipes.value">dinner</span></span><figure class="relative cursor-pointer" hx-get="/recipes/2" hx-target="#content" hx-push-url="true" hx-trigger="mousedown" hx-swap="innerHTML show:window:top transition:true"><img class="h-28 w-24 object-cover rounded-t-lg sm:h-40 sm:min-w-full sm:w-full" src="/data/images/Placeholders/placeholder.recipe.webp" alt="Image for the Best Chinese Kale1 recipe"><div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex"><p class="p-2 text-sm">This is the most delicious recipe!</p></div></figure><div class="card-body justify-between"><h2 class="sm:font-semibold sm:w-[25ch] sm:break-words sm:min-h-14">Best Chinese Kale1</h2><div class="sm:max-h-14 sm:overflow-y-auto sm:content-end sm:min-h-14"><div class="flex flex-col flex-wrap overflow-x-auto max-h-12 pb-2 sm:pb-0 sm:max-h-none sm:flex-auto sm:flex-row"><span class="sm:hidden"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "cat:dinner"}" _="on click put &quot;cat:dinner&quot; into #search_recipes.value">dinner</span></span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "tag":vegetarian}" _="on click put &quot;tag:vegetarian&quot; into #search-recipes.value">vegetarian</span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "tag":tofu}" _="on click put &quot;tag:tofu&quot; into #search-recipes.value">tofu</span></div></div><div class="card-actions flex-col-reverse h-fit"><button class="btn btn-block btn-xs btn-outline sm:btn-sm" hx-get="/recipes/2" hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true">View</button></div></div></section>"##,
                    r##"<section class="card-side sm:card card-compact card-bordered bg-base-100 shadow-lg indicator w-full"><span class="hidden sm:block"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral indicator-item indicator-center" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "cat:dinner"}" _="on click put &quot;cat:dinner&quot; into #search_recipes.value">dinner</span></span><figure class="relative cursor-pointer" hx-get="/recipes/3" hx-target="#content" hx-push-url="true" hx-trigger="mousedown" hx-swap="innerHTML show:window:top transition:true"><img class="h-28 w-24 object-cover rounded-t-lg sm:h-40 sm:min-w-full sm:w-full" src="/data/images/Placeholders/placeholder.recipe.webp" alt="Image for the Best Chinese Kale2 recipe"><div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex"><p class="p-2 text-sm">This is the most delicious recipe!</p></div></figure><div class="card-body justify-between"><h2 class="sm:font-semibold sm:w-[25ch] sm:break-words sm:min-h-14">Best Chinese Kale2</h2><div class="sm:max-h-14 sm:overflow-y-auto sm:content-end sm:min-h-14"><div class="flex flex-col flex-wrap overflow-x-auto max-h-12 pb-2 sm:pb-0 sm:max-h-none sm:flex-auto sm:flex-row"><span class="sm:hidden"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "cat:dinner"}" _="on click put &quot;cat:dinner&quot; into #search_recipes.value">dinner</span></span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "tag":vegetarian}" _="on click put &quot;tag:vegetarian&quot; into #search-recipes.value">vegetarian</span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "tag":tofu}" _="on click put &quot;tag:tofu&quot; into #search-recipes.value">tofu</span></div></div><div class="card-actions flex-col-reverse h-fit"><button class="btn btn-block btn-xs btn-outline sm:btn-sm" hx-get="/recipes/3" hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true">View</button></div></div></section>"##,
                    r#"<footer id="pagination" class="footer footer-center bg-base-200 pb-12 p-2 md:pb-2 text-base-content gap-2" onload="updateAddCookbookUrl(1)"></footer><div class="join gap-0"><button class="join-item btn btn-disabled">«</button><button aria-current="page" class="join-item btn btn-active">1</button><button class="join-item btn btn-disabled">»</button></div><div class="text-center"><p class="text-sm">Showing<span class="font-medium">1</span>to<span class="font-medium">3</span>of<span id="search-count" class="font-medium">3</span>results</p></div></div>"#,
                ],
            );
            Ok(())
        }
    }

    mod tests_recipe {
        use super::*;

        use axum::http::{HeaderValue, StatusCode};
        use axum_test::TestResponse;
        use uuid::Uuid;

        use crate::core::model::recipe::{RecipeForCreate, VideoForCreate};
        use crate::core::model::Recipe;
        use crate::server::test_utils::{a_complete_recipe, assert_html, assert_must_be_logged_in, build_server_ws};
        use crate::server::AppState;

        fn base_uri(recipe_id: i64) -> String {
            format!("/recipes/{recipe_id}")
        }

        // region GET /recipes/{id}
        #[tokio::test]
        async fn test_get_recipe_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(&base_uri(1)).await
        }

        #[tokio::test]
        async fn test_get_recipe_exists_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = AppState::new(config).await?;
            let recipe = a_complete_recipe();
            let _recipe_id = Recipe::create(&state.mm, 1, &recipe).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_ok();
            assert_complete_recipe(res, recipe);
            Ok(())
        }

        #[tokio::test]
        async fn test_get_recipe_exists_from_htmx_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let mut server = build_server_logged_in(config.clone()).await?;
            server.add_header(
                axum_htmx::headers::HX_REQUEST,
                HeaderValue::from_static("true"),
            );
            let state = AppState::new(config).await?;
            let recipe = a_complete_recipe();
            let _recipe_id = Recipe::create(&state.mm, 1, &recipe).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_ok();
            assert_complete_recipe(res, recipe);
            Ok(())
        }

        #[tokio::test]
        async fn test_get_recipe_not_exist_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&base_uri(999)).await;

            res.assert_status_ok();
            assert_html(
                res,
                vec![
                    r##"<title hx-swap-oob="true">Recipe Not Found | Recipya</title>"##,
                    "Recipe Not Found",
                    "The recipe you requested to view is not found.",
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_get_recipe_no_media_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = AppState::new(config).await?;
            let mut recipe = a_complete_recipe();
            recipe.videos.clear();
            recipe.images = None;
            let _recipe_id = Recipe::create(&state.mm, 1, &recipe).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_ok();
            assert_html(
                res,
                vec![
                    r#"<img style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src="/data/images/Placeholders/placeholder.recipe.webp">"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_get_recipe_1_image_0_video_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = AppState::new(config).await?;
            let mut recipe = a_complete_recipe();
            recipe.videos.clear();
            let img1 = Uuid::new_v4();
            recipe.images = Some(vec![img1]);
            let _recipe_id = Recipe::create(&state.mm, 1, &recipe).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_ok();
            assert_html(
                res,
                vec![&format!(
                    "<img id=\"output\" style=\"object-fit: cover\" alt=\"Image of the recipe\" class=\"w-full max-h-80 md:max-h-[34rem]\" src=\"/data/images/{img1}.webp\">"
                )],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_get_recipe_2_images_0_video_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = AppState::new(config).await?;
            let mut recipe = a_complete_recipe();
            recipe.videos.clear();
            recipe.images = Some(vec![Uuid::new_v4(), Uuid::new_v4()]);
            let _recipe_id = Recipe::create(&state.mm, 1, &recipe).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_ok();
            assert_html(
                res,
                vec![
                    r##"<img style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src="/data/images/Placeholders/placeholder.recipe.webp"><div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2"><a class="btn btn-circle" href="#media-1">❮</a><a class="btn btn-circle" href="#media-1">❯</a></div></div><div id="media-1" class="carousel-item relative w-full">"##,
                    r##"<img style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src="/data/images/Placeholders/placeholder.recipe.webp"><div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2"><a class="btn btn-circle" href="#media-0">❮</a><a class="btn btn-circle" href="#media-0">❯</a></div></div></div></div>"##,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_get_recipe_0_images_1_video_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = AppState::new(config).await?;
            let mut recipe = a_complete_recipe();
            recipe.videos = vec![VideoForCreate {
                video: Uuid::new_v4(),
                duration: None,
                content_url: Some(String::from("https://example.com/embed/yg8FG4")),
                embed_url: None,
            }];
            recipe.images = None;
            let _recipe_id = Recipe::create(&state.mm, 1, &recipe).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_ok();
            assert_html(
                res,
                vec![
                    r#"<video controls preload="metadata" src="https://example.com/embed/yg8FG4"></video>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_get_recipe_0_images_2_videos_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = AppState::new(config).await?;
            let mut recipe = a_complete_recipe();
            recipe.videos = vec![
                VideoForCreate {
                    video: Uuid::new_v4(),
                    duration: None,
                    content_url: Some(String::from("https://example.com/embed/yg8FG4")),
                    embed_url: None,
                },
                VideoForCreate {
                    video: Uuid::new_v4(),
                    duration: None,
                    content_url: None,
                    embed_url: Some(String::from("https://example.com/embed/yg8FG4")),
                },
            ];
            recipe.images = None;
            let _recipe_id = Recipe::create(&state.mm, 1, &recipe).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_ok();
            assert_html(
                res,
                vec![
                    r##"<video controls preload="metadata" src="https://example.com/embed/yg8FG4"></video><div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2"><a class="btn btn-circle" href="#media-1">❮</a><a class="btn btn-circle" href="#media-1">❯</a></div></div>"##,
                    r##"<div id="media-1" class="carousel-item relative w-full"><iframe src="https://example.com/embed/yg8FG4" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen="" style="height: 100%;width: 100%;"></iframe>"##,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_get_recipe_2_images_2_videos_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = AppState::new(config).await?;
            let mut recipe = a_complete_recipe();
            recipe.videos = vec![
                VideoForCreate {
                    video: Uuid::new_v4(),
                    duration: None,
                    content_url: Some(String::from("https://example.com/embed/yg8FG4")),
                    embed_url: None,
                },
                VideoForCreate {
                    video: Uuid::new_v4(),
                    duration: None,
                    content_url: None,
                    embed_url: Some(String::from("https://example.com/embed/yg8FG4")),
                },
            ];
            recipe.images = Some(vec![Uuid::new_v4(), Uuid::new_v4()]);
            let _recipe_id = Recipe::create(&state.mm, 1, &recipe).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_ok();
            assert_html(
                res,
                vec![
                    r##"<div class="carousel w-full"><div id="media-0" class="carousel-item relative w-full"><img style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src="/data/images/Placeholders/placeholder.recipe.webp"><div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2"><a class="btn btn-circle" href="#media-3">❮</a><a class="btn btn-circle" href="#media-1">❯</a></div></div>"##,
                    r##"<div id="media-1" class="carousel-item relative w-full"><img style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src="/data/images/Placeholders/placeholder.recipe.webp"><div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2"><a class="btn btn-circle" href="#media-0">❮</a><a class="btn btn-circle" href="#media-2">❯</a></div></div>"##,
                    r##"<div id="media-2" class="carousel-item relative w-full"><video controls preload="metadata" src="https://example.com/embed/yg8FG4"></video><div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2"><a class="btn btn-circle" href="#media-1">❮</a><a class="btn btn-circle" href="#media-3">❯</a></div></div>"##,
                    r##"<div id="media-3" class="carousel-item relative w-full"><iframe src="https://example.com/embed/yg8FG4" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen="" style="height: 100%;width: 100%;"></iframe><div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2"><a class="btn btn-circle" href="#media-2">❮</a><a class="btn btn-circle" href="#media-0">❯</a></div></div></div></div>"##,
                ],
            );
            Ok(())
        }
        //endregion

        // region DELETE /recipes/{id}
        #[tokio::test]
        async fn test_delete_recipe_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(&base_uri(1)).await
        }

        #[tokio::test]
        async fn test_delete_recipe_does_not_exist() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let (server, mut ws) = build_server_ws(config.clone()).await?;

            let res = server.delete(&base_uri(1)).await;

            res.assert_status_internal_server_error();
            let _ = ws.receive_message().await;
            ws
                .assert_receive_text_contains(r#"{"showMessageHtmx":{"type":"toast","message":"Recipe could not be deleted.","status":"alert-error","title":"Operation Failed"}}"#)
                .await;
            Ok(())
        }

        #[tokio::test]
        async fn test_delete_recipe_exists_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = AppState::new(config).await?;
            let _recipe_id = Recipe::create(&state.mm, 1, &a_complete_recipe()).await?;

            let res = server.delete(&base_uri(1)).await;

            res.assert_status(StatusCode::NO_CONTENT);
            res.assert_header(axum_htmx::headers::HX_REDIRECT, "/");
            Ok(())
        }
        // endregion

        fn assert_complete_recipe(res: TestResponse, recipe: RecipeForCreate) {
            assert_html(
                res,
                vec![
                    &format!(
                        "<title hx-swap-oob=\"true\">{} | Recipya</title>",
                        recipe.name
                    ),
                    r#"<button title="Toggle screen lock"#,
                    r##"<button class="ml-2 hidden sm:block" title="Edit recipe" hx-get="/recipes/1/edit" hx-push-url="true" hx-target="#content" hx-swap="innerHTML transition:true">"##,
                    &format!(
                        "<span class=\"text-center pb-2 print:w-full\" itemprop=\"name\">{}</span>",
                        recipe.name
                    ),
                    r##"<button title="Share recipe" class="mr-2 hidden sm:block" hx-post="/recipes/1/share" hx-target="#share-dialog-result""##,
                    r#"<button class="mr-2 hidden sm:block" title="Print recipe" _="on click print()">"#,
                    r##"<button title="Delete recipe" class="mr-2 hidden sm:block" hx-delete="/recipes/1" hx-swap="none" hx-confirm="Are you sure you wish to delete this recipe?" hx-indicator="#fullscreen-loader">"##,
                    r#"<iframe src="https://example.com/embed/j43yfe3.mp4" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen="" style="height: 100%;width: 100%;"></iframe>"#,
                    r#"<div id="media-0" class="carousel-item relative w-full"><img style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src="/data/images/Placeholders/placeholder.recipe.webp">"#,
                    r#"<img style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src="/data/images/Placeholders/placeholder.recipe.webp">"#,
                    r#"<div class="badge badge-primary badge-outline">dinner</div>"#,
                    r#"<div class="badge badge-sm badge-neutral m-1 flex-auto">vegetarian</div><div class="badge badge-sm badge-neutral m-1 flex-auto">tofu</div>"#,
                    r##"<fieldset class="fieldset"><legend>Servings</legend><label class="label" for="yield">Servings</label><input id="yield" type="number" min="1" name="yield" value="4" class="input" hx-get="/recipes/1/scale" hx-trigger="input" hx-target="#ingredients-instructions-container"></fieldset>"##,
                    r#"<a class="btn btn-sm btn-outline no-underline print:hidden" href="https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/" target="_blank">Source</a>"#,
                    r#"<textarea class="textarea w-full h-full resize-none" readonly>This is the most delicious recipe!</textarea>"#,
                    r#"<p class="text-xs">Per 100g: calories 300 kcal; total carbohydrates 55g; sugar 43g; protein 7g; total fat 6g; saturated fat 1g; unsaturated fat 2g; trans fat 3g; cholesterol 5mg; sodium 12mg; fiber 10g</p>"#,
                    r#"<div class="grid grid-flow-col border-gray-700 col-span-6 py-1 md:border-y md:grid-cols-3 md:row-span-1 print:border-none"><div class="flex justify-self-center items-center gap-1 cursor-default" title="Prep time">"#,
                    r#"<table class="table table-zebra table-xs print:hidden"><thead><tr><th>Nutrition (per 100g)</th><th>Amount</th></tr></thead><tbody><tr><td>Calories:</td><td>300 kcal</td></tr><tr><td>Total carbs:</td><td>55 g</td></tr><tr><td>Sugars:</td><td>43 g</td></tr><tr><td>Protein:</td><td>7 g</td></tr><tr><td>Total fat:</td><td>6 g</td></tr><tr><td>Saturated fat:</td><td>1 g</td></tr><tr><td>Unsaturated fat:</td><td>2 g</td></tr><tr><td>Trans fat:</td><td>3 g</td></tr><tr><td>Cholesterol:</td><td>5 mg</td></tr><tr><td>Sodium:</td><td>12 mg</td></tr><tr><td>Fiber:</td><td>10 g</td></tr></tbody></table>"#,
                    r#"<h1 class="text-sm print:mb-1"><b>Tools</b></h1><ol class="col-span-6 w-full mb-4" style="column-count: 1"><li class="text-sm"><label><input type="checkbox"></label><span class="pl-2">1wok</span></li><li class="text-sm"><label><input type="checkbox"></label><span class="pl-2">1frying pan</span></li></ol>"#,
                    r#"<div class="col-span-6 px-8 py-2 border-gray-700 md:rounded-bl-none md:col-span-4 print:hidden"><h2 class="font-semibold text-center underline pb-1">Instructions</h2><ol class="grid list-decimal"><li class="min-w-full py-2 select-none hover:bg-gray-100 dark:hover:bg-gray-700" _="on mousedown toggle .line-through"><span class="whitespace-pre-line">Mix all these ingredients</span></li><li class="min-w-full py-2 select-none hover:bg-gray-100 dark:hover:bg-gray-700" _="on mousedown toggle .line-through"><span class="whitespace-pre-line">Turn the oven at 300 F</span></li><li class="min-w-full py-2 select-none hover:bg-gray-100 dark:hover:bg-gray-700" _="on mousedown toggle .line-through"><span class="whitespace-pre-line">Soak the chicken in the lemon juice</span></li><li class="min-w-full py-2 select-none hover:bg-gray-100 dark:hover:bg-gray-700" _="on mousedown toggle .line-through"><span class="whitespace-pre-line">Bake for 35 minutes</span></li></ol></div></div>"#,
                    r#"<h1 class="text-sm print:mb-1"><b>Ingredients</b></h1><ol class="col-span-6 w-full print:mb-2" style="column-count: 1"><li class="text-sm"><label><input type="checkbox"></label><span class="pl-2">"#,
                ],
            )
        }
    }
}
