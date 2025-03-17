use axum::routing::get;
use axum::{middleware, Router};

use crate::server::router::handlers::recipes::{
    delete_recipe_handler, recipe_view_handler, recipes_add_handler, recipes_handler,
    supported_applications_handler, supported_websites_handler,
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
        .route(
            "/supported-applications",
            get(supported_applications_handler),
        )
        .route("/supported-websites", get(supported_websites_handler))
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
        use crate::server::test_utils::{
            a_complete_recipe_for_create, assert_html, assert_must_be_logged_in,
        };
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
                let mut recipe = a_complete_recipe_for_create();
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
        use crate::server::test_utils::{
            a_complete_recipe_for_create, assert_html, assert_must_be_logged_in, build_server_ws,
        };
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
            let recipe = a_complete_recipe_for_create();
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
            let recipe = a_complete_recipe_for_create();
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
            let mut recipe = a_complete_recipe_for_create();
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
            let mut recipe = a_complete_recipe_for_create();
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
            let mut recipe = a_complete_recipe_for_create();
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
            let mut recipe = a_complete_recipe_for_create();
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
            let mut recipe = a_complete_recipe_for_create();
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
            let mut recipe = a_complete_recipe_for_create();
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
            let _recipe_id = Recipe::create(&state.mm, 1, &a_complete_recipe_for_create()).await?;

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

    mod tests_supported_applications {
        use super::*;
        use axum::http::header::CONTENT_TYPE;

        use crate::server::test_utils::assert_must_be_logged_in;

        const BASE_URI: &str = "/recipes/supported-applications";

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(BASE_URI).await
        }

        #[tokio::test]
        async fn test_returns_list_of_apps_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            res.assert_header(CONTENT_TYPE, "text/html; charset=utf-8");
            res.assert_text_contains(r#"<tr class="text-center"><td>1</td><td><a class="underline" href="https://www.accuchef.com" target="_blank">AccuChef</a></td></tr><tr class="text-center"><td>2</td><td><a class="underline" href="https://cheftap.com" target="_blank">ChefTap</a></td></tr><tr class="text-center"><td>3</td><td><a class="underline" href="https://crouton.app" target="_blank">Crouton</a></td></tr><tr class="text-center"><td>4</td><td><a class="underline" href="https://easy-recipe-deluxe.software.informer.com" target="_blank">Easy Recipe Deluxe</a></td></tr><tr class="text-center"><td>5</td><td><a class="underline" href="https://www.kalorio.de" target="_blank">Kalorio</a></td></tr><tr class="text-center"><td>6</td><td><a class="underline" href="https://www.mastercook.com" target="_blank">MasterCook</a></td></tr><tr class="text-center"><td>7</td><td><a class="underline" href="https://www.paprikaapp.com" target="_blank">Paprika</a></td></tr><tr class="text-center"><td>8</td><td><a class="underline" href="https://recipekeeperonline.com" target="_blank">Recipe Keeper</a></td></tr><tr class="text-center"><td>9</td><td><a class="underline" href="https://recipesage.com" target="_blank">RecipeSage</a></td></tr><tr class="text-center"><td>10</td><td><a class="underline" href="https://www.mysaffronapp.com" target="_blank">Saffron</a></td></tr>"#);
            Ok(())
        }
    }

    mod tests_supported_websites {
        use super::*;

        use axum_test::http::header::CONTENT_TYPE;

        use crate::server::test_utils::assert_must_be_logged_in;

        const BASE_URI: &str = "/recipes/supported-websites";

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(BASE_URI).await
        }

        #[tokio::test]
        async fn test_fetch_websites_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            res.assert_header(CONTENT_TYPE, "text/html; charset=utf-8");
            res.assert_text_contains(r#"<tr class="text-center"><td>1</td><td><a class="underline" href="https://15gram.be/recepten" target="_blank">15gram.be</a></td></tr><tr class="text-center"><td>2</td><td><a class="underline" href="https://www.750g.com" target="_blank">750g.com</a></td></tr><tr class="text-center"><td>3</td><td><a class="underline" href="https://101cookbooks.com" target="_blank">101cookbooks.com</a></td></tr><tr class="text-center"><td>4</td><td><a class="underline" href="https://www.claudia.abril.com.br/receitas" target="_blank">claudia.abril.com</a></td></tr><tr class="text-center"><td>5</td><td><a class="underline" href="https://aberlehome.com" target="_blank">aberlehome.com</a></td></tr><tr class="text-center"><td>6</td><td><a class="underline" href="https://abuelascounter.com" target="_blank">abuelascounter.com</a></td></tr><tr class="text-center"><td>7</td><td><a class="underline" href="https://www.acouplecooks.com" target="_blank">acouplecooks.com</a></td></tr><tr class="text-center"><td>8</td><td><a class="underline" href="https://addapinch.com" target="_blank">addapinch.com</a></td></tr><tr class="text-center"><td>9</td><td><a class="underline" href="http://www.afghankitchenrecipes.com" target="_blank">afghankitchenrecipes.com</a></td></tr><tr class="text-center"><td>10</td><td><a class="underline" href="https://www.ah.nl/allerhande" target="_blank">ah.nl</a></td></tr><tr class="text-center"><td>11</td><td><a class="underline" href="https://akispetretzikis.com" target="_blank">akispetretzikis.com</a></td></tr><tr class="text-center"><td>12</td><td><a class="underline" href="https://www.allrecipes.com" target="_blank">allrecipes.com</a></td></tr><tr class="text-center"><td>13</td><td><a class="underline" href="https://altonbrown.com" target="_blank">altonbrown.com</a></td></tr><tr class="text-center"><td>14</td><td><a class="underline" href="https://amazingribs.com" target="_blank">amazingribs.com</a></td></tr><tr class="text-center"><td>15</td><td><a class="underline" href="https://www.ambitiouskitchen.com" target="_blank">ambitiouskitchen.com</a></td></tr><tr class="text-center"><td>16</td><td><a class="underline" href="https://www.archanaskitchen.com" target="_blank">archanaskitchen.com</a></td></tr><tr class="text-center"><td>17</td><td><a class="underline" href="https://www.argiro.gr" target="_blank">argiro.gr</a></td></tr><tr class="text-center"><td>18</td><td><a class="underline" href="https://www.arla.se/recept" target="_blank">arla.se</a></td></tr><tr class="text-center"><td>19</td><td><a class="underline" href="https://www.atelierdeschefs.fr" target="_blank">atelierdeschefs.fr</a></td></tr><tr class="text-center"><td>20</td><td><a class="underline" href="https://www.averiecooks.com" target="_blank">averiecooks.com</a></td></tr><tr class="text-center"><td>21</td><td><a class="underline" href="https://bakingmischief.com" target="_blank">bakingmischief.com</a></td></tr><tr class="text-center"><td>22</td><td><a class="underline" href="https://www.baking-sense.com" target="_blank">baking-sense.com</a></td></tr><tr class="text-center"><td>23</td><td><a class="underline" href="https://www.bbc.co.uk/food/recipes" target="_blank">bbc.co.uk</a></td></tr><tr class="text-center"><td>24</td><td><a class="underline" href="https://www.bbcgoodfood.com/recipes" target="_blank">bbcgoodfood.com</a></td></tr><tr class="text-center"><td>25</td><td><a class="underline" href="https://www.bergamot.app" target="_blank">bergamot.app</a></td></tr><tr class="text-center"><td>26</td><td><a class="underline" href="https://www.bettycrocker.com/recipes" target="_blank">bettycrocker.com</a></td></tr><tr class="text-center"><td>27</td><td><a class="underline" href="https://biancazapatka.com" target="_blank">biancazapatka</a></td></tr><tr class="text-center"><td>28</td><td><a class="underline" href="https://www.bigoven.com" target="_blank">bigoven.com</a></td></tr><tr class="text-center"><td>29</td><td><a class="underline" href="https://www.blueapron.com/cookbook" target="_blank">blueapron.com</a></td></tr><tr class="text-center"><td>30</td><td><a class="underline" href="https://bluejeanchef.com/" target="_blank">bluejeanchef.com</a></td></tr><tr class="text-center"><td>31</td><td><a class="underline" href="https://www.brianlagerstrom.com" target="_blank">brianlagerstrom.com</a></td></tr><tr class="text-center"><td>32</td><td><a class="underline" href="https://briceletbaklava.ch" target="_blank">briceletbaklava.ch</a></td></tr><tr class="text-center"><td>33</td><td><a class="underline" href="https://www.bodybuilding.com/recipes" target="_blank">bodybuilding.com</a></td></tr><tr class="text-center"><td>34</td><td><a class="underline" href="https://www.bonappetit.com" target="_blank">bonappetit.com</a></td></tr><tr class="text-center"><td>35</td><td><a class="underline" href="https://www.bongeats.com/" target="_blank">bongeats.com</a></td></tr><tr class="text-center"><td>36</td><td><a class="underline" href="https://www.bowlofdelicious.com" target="_blank">bowlofdelicious.com</a></td></tr><tr class="text-center"><td>37</td><td><a class="underline" href="https://www.budgetbytes.com" target="_blank">budgetbytes.com</a></td></tr><tr class="text-center"><td>38</td><td><a class="underline" href="https://cafedelites.com" target="_blank">cafedelites.com</a></td></tr><tr class="text-center"><td>39</td><td><a class="underline" href="https://www.castironketo.net" target="_blank">castironketo.net</a></td></tr><tr class="text-center"><td>40</td><td><a class="underline" href="https://www.cdkitchen.com" target="_blank">cdkitchen.com</a></td></tr><tr class="text-center"><td>41</td><td><a class="underline" href="https://www.chefkoch.de" target="_blank">chefkoch.de</a></td></tr><tr class="text-center"><td>42</td><td><a class="underline" href="https://www.chefnini.com" target="_blank">chefnini.com</a></td></tr><tr class="text-center"><td>43</td><td><a class="underline" href="https://chefsavvy.com" target="_blank">chefsavvy.com</a></td></tr><tr class="text-center"><td>44</td><td><a class="underline" href="https://www.closetcooking.com" target="_blank">closetcooking.com</a></td></tr><tr class="text-center"><td>45</td><td><a class="underline" href="https://comidinhasdochef.com" target="_blank">comidinhasdochef.com</a></td></tr><tr class="text-center"><td>46</td><td><a class="underline" href="https://cookeatshare.com" target="_blank">cookeatshare.com</a></td></tr><tr class="text-center"><td>47</td><td><a class="underline" href="https://cookieandkate.com" target="_blank">cookieandkate.com</a></td></tr><tr class="text-center"><td>48</td><td><a class="underline" href="https://cookpad.com" target="_blank">cookpad.com</a></td></tr><tr class="text-center"><td>49</td><td><a class="underline" href="https://cook-talk.com" target="_blank">cook-talk.com</a></td></tr><tr class="text-center"><td>50</td><td><a class="underline" href="https://www.coop.se/recept" target="_blank">coop.se</a></td></tr><tr class="text-center"><td>51</td><td><a class="underline" href="https://copykat.com" target="_blank">copykat.com</a></td></tr><tr class="text-center"><td>52</td><td><a class="underline" href="https://www.costco.com" target="_blank">costco.com</a></td></tr><tr class="text-center"><td>53</td><td><a class="underline" href="https://www.countryliving.com" target="_blank">countryliving.com</a></td></tr><tr class="text-center"><td>54</td><td><a class="underline" href="https://creativecanning.com" target="_blank">creativecanning.com</a></td></tr><tr class="text-center"><td>55</td><td><a class="underline" href="https://www.cucchiaio.it/" target="_blank">cucchiaio.it</a></td></tr><tr class="text-center"><td>56</td><td><a class="underline" href="https://www.cuisineaz.com" target="_blank">cuisineaz.com</a></td></tr><tr class="text-center"><td>57</td><td><a class="underline" href="https://cybercook.com.br" target="_blank">cybercook.com.br</a></td></tr><tr class="text-center"><td>58</td><td><a class="underline" href="https://www.davidlebovitz.com" target="_blank">davidlebovitz.com</a></td></tr><tr class="text-center"><td>59</td><td><a class="underline" href="https://www.delish.com" target="_blank">delish.com</a></td></tr><tr class="text-center"><td>60</td><td><a class="underline" href="https://www.ditchthecarbs.com" target="_blank">ditchthecarbs.com</a></td></tr><tr class="text-center"><td>61</td><td><a class="underline" href="https://domesticate-me.com" target="_blank">domesticate-me.com</a></td></tr><tr class="text-center"><td>62</td><td><a class="underline" href="https://downshiftology.com" target="_blank">downshiftology.com</a></td></tr><tr class="text-center"><td>63</td><td><a class="underline" href="https://www.dr.dk/mad/opskrift" target="_blank">dr.dk</a></td></tr><tr class="text-center"><td>64</td><td><a class="underline" href="https://www.eatingbirdfood.com" target="_blank">eatingbirdfood.com</a></td></tr><tr class="text-center"><td>65</td><td><a class="underline" href="https://www.eatingwell.com" target="_blank">eatingwell.com</a></td></tr><tr class="text-center"><td>66</td><td><a class="underline" href="https://eatsmarter.com" target="_blank">eatsmarter.com</a></td></tr><tr class="text-center"><td>67</td><td><a class="underline" href="https://www.eatwell101.com" target="_blank">eatwell101.com</a></td></tr><tr class="text-center"><td>68</td><td><a class="underline" href="https://eatwhattonight.com" target="_blank">eatwhattonight</a></td></tr><tr class="text-center"><td>69</td><td><a class="underline" href="https://elavegan.com" target="_blank">elavegan.com</a></td></tr><tr class="text-center"><td>70</td><td><a class="underline" href="https://emmikochteinfach.de" target="_blank">emmikochteinfach.de</a></td></tr><tr class="text-center"><td>71</td><td><a class="underline" href="https://www.epicurious.com" target="_blank">epicurious.com</a></td></tr><tr class="text-center"><td>72</td><td><a class="underline" href="https://www.errenskitchen.com" target="_blank">errenskitchen.com</a></td></tr><tr class="text-center"><td>73</td><td><a class="underline" href="https://www.expressen.se/alltommat/recept" target="_blank">expressen.se</a></td></tr><tr class="text-center"><td>74</td><td><a class="underline" href="https://recipes.farmhousedelivery.com" target="_blank">farmhousedelivery.com</a></td></tr><tr class="text-center"><td>75</td><td><a class="underline" href="https://www.farmhouseonboone.com" target="_blank">farmhouseonboone.com</a></td></tr><tr class="text-center"><td>76</td><td><a class="underline" href="https://www.fattoincasadabenedetta.it" target="_blank">fattoincasadabenedetta.it</a></td></tr><tr class="text-center"><td>77</td><td><a class="underline" href="https://www.fifteenspatulas.com" target="_blank">fifteenspatulas.com</a></td></tr><tr class="text-center"><td>78</td><td><a class="underline" href="https://www.finedininglovers.com" target="_blank">finedininglovers.com</a></td></tr><tr class="text-center"><td>79</td><td><a class="underline" href="https://fitmencook.com" target="_blank">fitmencook.com</a></td></tr><tr class="text-center"><td>80</td><td><a class="underline" href="https://www.food.com" target="_blank">food.com</a></td></tr><tr class="text-center"><td>81</td><td><a class="underline" href="https://food52.com/recipes" target="_blank">food52.com</a></td></tr><tr class="text-center"><td>82</td><td><a class="underline" href="https://www.foodandwine.com" target="_blank">foodandwine.com</a></td></tr><tr class="text-center"><td>83</td><td><a class="underline" href="https://www.foodbag.be/nl/home" target="_blank">foodbag.be</a></td></tr><tr class="text-center"><td>84</td><td><a class="underline" href="https://foodnetwork.co.uk/recipes" target="_blank">foodnetwork.co.uk</a></td></tr><tr class="text-center"><td>85</td><td><a class="underline" href="https://www.foodrepublic.com" target="_blank">foodrepublic.com</a></td></tr><tr class="text-center"><td>86</td><td><a class="underline" href="https://www.forksoverknives.com" target="_blank">forksoverknives.com</a></td></tr><tr class="text-center"><td>87</td><td><a class="underline" href="https://forktospoon.com" target="_blank">forktospoon.com</a></td></tr><tr class="text-center"><td>88</td><td><a class="underline" href="https://www.franzoesischkochen.de" target="_blank">franzoesischkochen.de</a></td></tr><tr class="text-center"><td>89</td><td><a class="underline" href="https://fredriksfika.allas.se" target="_blank">fredriksfika.allas.se</a></td></tr><tr class="text-center"><td>90</td><td><a class="underline" href="https://www.gesund-aktiv.com/rezepte" target="_blank">gesund-aktiv.com</a></td></tr><tr class="text-center"><td>91</td><td><a class="underline" href="https://www.giallozafferano.com" target="_blank">giallozafferano.com</a></td></tr><tr class="text-center"><td>92</td><td><a class="underline" href="https://www.gimmesomeoven.com" target="_blank">gimmesomeoven.com</a></td></tr><tr class="text-center"><td>93</td><td><a class="underline" href="https://receitas.globo.com" target="_blank">globo.com</a></td></tr><tr class="text-center"><td>94</td><td><a class="underline" href="https://www.gonnawantseconds.com" target="_blank">gonnawantseconds.com</a></td></tr><tr class="text-center"><td>95</td><td><a class="underline" href="https://goodfooddiscoveries.com" target="_blank">goodfooddiscoveries.com</a></td></tr><tr class="text-center"><td>96</td><td><a class="underline" href="https://www.goodhousekeeping.com" target="_blank">goodhousekeeping.com</a></td></tr><tr class="text-center"><td>97</td><td><a class="underline" href="https://www.grandfrais.com/recettes" target="_blank">grandfrais.com</a></td></tr><tr class="text-center"><td>98</td><td><a class="underline" href="https://www.greatbritishchefs.com" target="_blank">greatbritishchefs.com</a></td></tr><tr class="text-center"><td>99</td><td><a class="underline" href="https://grimgrains.com" target="_blank">grimgrains.com</a></td></tr><tr class="text-center"><td>100</td><td><a class="underline" href="http://www.grouprecipes.com" target="_blank">grouprecipes.com</a></td></tr><tr class="text-center"><td>101</td><td><a class="underline" href="https://www.halfbakedharvest.com" target="_blank">halfbakedharvest.com</a></td></tr><tr class="text-center"><td>102</td><td><a class="underline" href="https://handletheheat.com" target="_blank">handletheheat.com</a></td></tr><tr class="text-center"><td>103</td><td><a class="underline" href="https://www.hassanchef.com" target="_blank">hassanchef.com</a></td></tr><tr class="text-center"><td>104</td><td><a class="underline" href="https://headbangerskitchen.com" target="_blank">headbangerskitchen.com</a></td></tr><tr class="text-center"><td>105</td><td><a class="underline" href="https://heatherchristo.com" target="_blank">heatherchristo.com</a></td></tr><tr class="text-center"><td>106</td><td><a class="underline" href="https://www.hellofresh.com/recipes" target="_blank">hellofresh.com</a></td></tr><tr class="text-center"><td>107</td><td><a class="underline" href="https://www.homechef.com/our-menu" target="_blank">homechef.com</a></td></tr><tr class="text-center"><td>108</td><td><a class="underline" href="https://hostthetoast.com" target="_blank">hostthetoast.com</a></td></tr><tr class="text-center"><td>109</td><td><a class="underline" href="https://www.ica.se/recept" target="_blank">ica.se</a></td></tr><tr class="text-center"><td>110</td><td><a class="underline" href="https://im-worthy.com" target="_blank">im-worthy.com</a></td></tr><tr class="text-center"><td>111</td><td><a class="underline" href="https://www.indianhealthyrecipes.com" target="_blank">indianhealthyrecipes.com</a></td></tr><tr class="text-center"><td>112</td><td><a class="underline" href="https://www.innit.com/meal" target="_blank">innit.com</a></td></tr><tr class="text-center"><td>113</td><td><a class="underline" href="https://insanelygoodrecipes.com" target="_blank">insanelygoodrecipes.com</a></td></tr><tr class="text-center"><td>114</td><td><a class="underline" href="https://inspiralized.com" target="_blank">inspiralized.com</a></td></tr><tr class="text-center"><td>115</td><td><a class="underline" href="https://www.jamieoliver.com" target="_blank">jamieoliver.com</a></td></tr><tr class="text-center"><td>116</td><td><a class="underline" href="https://jimcooksfoodgood.com" target="_blank">jimcooksfoodgood.com</a></td></tr><tr class="text-center"><td>117</td><td><a class="underline" href="https://joyfoodsunshine.com" target="_blank">joyfoodsunshine.com</a></td></tr><tr class="text-center"><td>118</td><td><a class="underline" href="https://juliegoodwin.com.au" target="_blank">juliegoodwin.com.au</a></td></tr><tr class="text-center"><td>119</td><td><a class="underline" href="https://www.justataste.com" target="_blank">justataste.com</a></td></tr><tr class="text-center"><td>120</td><td><a class="underline" href="https://justbento.com" target="_blank">justbento.com</a></td></tr><tr class="text-center"><td>121</td><td><a class="underline" href="https://www.justonecookbook.com" target="_blank">justonecookbook.com</a></td></tr><tr class="text-center"><td>122</td><td><a class="underline" href="https://kennymcgovern.com" target="_blank">kennymcgovern.com</a></td></tr><tr class="text-center"><td>123</td><td><a class="underline" href="https://www.kingarthurbaking.com" target="_blank">kingarthurbaking.com</a></td></tr><tr class="text-center"><td>124</td><td><a class="underline" href="https://www.kitchenstories.com" target="_blank">kitchenstories.com</a></td></tr><tr class="text-center"><td>125</td><td><a class="underline" href="https://www.kochbar.de/rezept" target="_blank">kochbar.de</a></td></tr><tr class="text-center"><td>126</td><td><a class="underline" href="https://kochbucher.com" target="_blank">kochbucher.com</a></td></tr><tr class="text-center"><td>127</td><td><a class="underline" href="https://www.koket.se" target="_blank">koket.se</a></td></tr><tr class="text-center"><td>128</td><td><a class="underline" href="https://www.kptncook.com" target="_blank">kptncook.com</a></td></tr><tr class="text-center"><td>129</td><td><a class="underline" href="https://www.kuchnia-domowa.pl" target="_blank">kuchnia-domowa.pl</a></td></tr><tr class="text-center"><td>130</td><td><a class="underline" href="https://www.kwestiasmaku.com" target="_blank">kwestiasmaku.com</a></td></tr><tr class="text-center"><td>131</td><td><a class="underline" href="https://www.latelierderoxane.com" target="_blank">latelierderoxane.com</a></td></tr><tr class="text-center"><td>132</td><td><a class="underline" href="https://leanandgreenrecipes.net" target="_blank">leanandgreenrecipes.net</a></td></tr><tr class="text-center"><td>133</td><td><a class="underline" href="https://www.lecker.de" target="_blank">lecker.de</a></td></tr><tr class="text-center"><td>134</td><td><a class="underline" href="https://www.lecremedelacrumb.com" target="_blank">lecremedelacrumb.com</a></td></tr><tr class="text-center"><td>135</td><td><a class="underline" href="https://lifestyleofafoodie.com" target="_blank">lifestyleofafoodie.com</a></td></tr><tr class="text-center"><td>136</td><td><a class="underline" href="https://littlespicejar.com" target="_blank">littlespicejar.com</a></td></tr><tr class="text-center"><td>137</td><td><a class="underline" href="https://livelytable.com" target="_blank">livelytable.com</a></td></tr><tr class="text-center"><td>138</td><td><a class="underline" href="https://livingthegreenlife.com" target="_blank">livingthegreenlife.com</a></td></tr><tr class="text-center"><td>139</td><td><a class="underline" href="https://lovingitvegan.com" target="_blank">lovingitvegan.com</a></td></tr><tr class="text-center"><td>140</td><td><a class="underline" href="https://www.maangchi.com" target="_blank">maangchi.com</a></td></tr><tr class="text-center"><td>141</td><td><a class="underline" href="https://madensverden.dk" target="_blank">madensverden.dk</a></td></tr><tr class="text-center"><td>142</td><td><a class="underline" href="https://madsvin.com" target="_blank">madsvin.com</a></td></tr><tr class="text-center"><td>143</td><td><a class="underline" href="https://www.marthastewart.com" target="_blank">marthastewart.com</a></td></tr><tr class="text-center"><td>144</td><td><a class="underline" href="https://www.matprat.no" target="_blank">matprat.no</a></td></tr><tr class="text-center"><td>145</td><td><a class="underline" href="https://meljoulwan.com" target="_blank">meljoulwan.com</a></td></tr><tr class="text-center"><td>146</td><td><a class="underline" href="https://www.melskitchencafe.com" target="_blank">melskitchencafe.com</a></td></tr><tr class="text-center"><td>147</td><td><a class="underline" href="https://www.mindmegette.hu" target="_blank">mindmegette.hu</a></td></tr><tr class="text-center"><td>148</td><td><a class="underline" href="https://minimalistbaker.com" target="_blank">minimalistbaker.com</a></td></tr><tr class="text-center"><td>149</td><td><a class="underline" href="https://ministryofcurry.com" target="_blank">ministryofcurry.com</a></td></tr><tr class="text-center"><td>150</td><td><a class="underline" href="https://www.misya.info" target="_blank">misya.info</a></td></tr><tr class="text-center"><td>151</td><td><a class="underline" href="https://momsdish.com" target="_blank">momsdish.com</a></td></tr><tr class="text-center"><td>152</td><td><a class="underline" href="https://momswithcrockpots.com" target="_blank">momswithcrockpots.com</a></td></tr><tr class="text-center"><td>153</td><td><a class="underline" href="https://www.monsieur-cuisine.com" target="_blank">monsieur-cuisine.com</a></td></tr><tr class="text-center"><td>154</td><td><a class="underline" href="https://www.motherthyme.com" target="_blank">motherthyme.com</a></td></tr><tr class="text-center"><td>155</td><td><a class="underline" href="https://www.moulinex.fr/recette" target="_blank">moulinex.fr</a></td></tr><tr class="text-center"><td>156</td><td><a class="underline" href="https://www.mundodereceitasbimby.com.pt" target="_blank">mundodereceitasbimby.com.pt</a></td></tr><tr class="text-center"><td>157</td><td><a class="underline" href="https://www.mybakingaddiction.com" target="_blank">mybakingaddiction.com</a></td></tr><tr class="text-center"><td>158</td><td><a class="underline" href="https://mykitchen101.com" target="_blank">mykitchen101.com</a></td></tr><tr class="text-center"><td>159</td><td><a class="underline" href="https://mykitchen101en.com" target="_blank">mykitchen101en.com</a></td></tr><tr class="text-center"><td>160</td><td><a class="underline" href="https://www.myplate.gov/recipes" target="_blank">myplate.gov</a></td></tr><tr class="text-center"><td>161</td><td><a class="underline" href="https://www.myrecipes.com" target="_blank">myrecipes.com</a></td></tr><tr class="text-center"><td>162</td><td><a class="underline" href="https://ninjatestkitchen.eu" target="_blank">ninjatestkitchen.eu</a></td></tr><tr class="text-center"><td>163</td><td><a class="underline" href="https://nourishedbynutrition.com" target="_blank">nourishedbynutrition.com</a></td></tr><tr class="text-center"><td>164</td><td><a class="underline" href="https://www.nosalty.hu" target="_blank">nosalty.hu</a></td></tr><tr class="text-center"><td>165</td><td><a class="underline" href="https://www.nrk.no/mat" target="_blank">nrk.no</a></td></tr><tr class="text-center"><td>166</td><td><a class="underline" href="https://www.number-2-pencil.com" target="_blank">number-2-pencil.com</a></td></tr><tr class="text-center"><td>167</td><td><a class="underline" href="https://nutritionfacts.org/recipes" target="_blank">nutritionfacts.org</a></td></tr><tr class="text-center"><td>168</td><td><a class="underline" href="https://cooking.nytimes.com/recipes" target="_blank">nytimes.com</a></td></tr><tr class="text-center"><td>169</td><td><a class="underline" href="https://ohsheglows.com" target="_blank">ohsheglows.com</a></td></tr><tr class="text-center"><td>170</td><td><a class="underline" href="https://omnivorescookbook.com" target="_blank">omnivorescookbook.com</a></td></tr><tr class="text-center"><td>171</td><td><a class="underline" href="https://www.onceuponachef.com" target="_blank">onceuponachef.com</a></td></tr><tr class="text-center"><td>172</td><td><a class="underline" href="https://www.owen-han.com" target="_blank">owen-han.com</a></td></tr><tr class="text-center"><td>173</td><td><a class="underline" href="https://www.paleorunningmomma.com" target="_blank">paleorunningmomma.com</a></td></tr><tr class="text-center"><td>174</td><td><a class="underline" href="https://www.panelinha.com.br" target="_blank">panelinha.com.br</a></td></tr><tr class="text-center"><td>175</td><td><a class="underline" href="https://paninihappy.com" target="_blank">paninihappy.com</a></td></tr><tr class="text-center"><td>176</td><td><a class="underline" href="https://www.persnicketyplates.com" target="_blank">persnicketyplates.com</a></td></tr><tr class="text-center"><td>177</td><td><a class="underline" href="https://www.pickuplimes.com" target="_blank">pickuplimes.com</a></td></tr><tr class="text-center"><td>178</td><td><a class="underline" href="https://www.pingodoce.pt" target="_blank">pingodoce.pt</a></td></tr><tr class="text-center"><td>179</td><td><a class="underline" href="https://pinkowlkitchen.com" target="_blank">pinkowlkitchen.com</a></td></tr><tr class="text-center"><td>180</td><td><a class="underline" href="https://www.platingpixels.com" target="_blank">platingpixels.com</a></td></tr><tr class="text-center"><td>181</td><td><a class="underline" href="https://www.ploetzblog.de/rezepte" target="_blank">ploetzblog.de</a></td></tr><tr class="text-center"><td>182</td><td><a class="underline" href="https://plowingthroughlife.com" target="_blank">plowingthroughlife.com</a></td></tr><tr class="text-center"><td>183</td><td><a class="underline" href="https://www.popsugar.co.uk/food" target="_blank">popsugar.co.uk</a></td></tr><tr class="text-center"><td>184</td><td><a class="underline" href="https://practicalselfreliance.com" target="_blank">practicalselfreliance.com</a></td></tr><tr class="text-center"><td>185</td><td><a class="underline" href="https://pressureluckcooking.com" target="_blank">pressureluckcooking.com</a></td></tr><tr class="text-center"><td>186</td><td><a class="underline" href="https://www.primaledgehealth.com" target="_blank">primaledgehealth.com</a></td></tr><tr class="text-center"><td>187</td><td><a class="underline" href="https://www.projectgezond.nl" target="_blank">projectgezond.nl</a></td></tr><tr class="text-center"><td>188</td><td><a class="underline" href="https://www.przepisy.pl" target="_blank">przepisy.pl</a></td></tr><tr class="text-center"><td>189</td><td><a class="underline" href="https://purelypope.com" target="_blank">purelypope.com</a></td></tr><tr class="text-center"><td>190</td><td><a class="underline" href="https://www.purplecarrot.com" target="_blank">purplecarrot.com</a></td></tr><tr class="text-center"><td>191</td><td><a class="underline" href="https://rachlmansfield.com" target="_blank">rachlmansfield.com</a></td></tr><tr class="text-center"><td>192</td><td><a class="underline" href="https://rainbowplantlife.com" target="_blank">rainbowplantlife.com</a></td></tr><tr class="text-center"><td>193</td><td><a class="underline" href="https://www.realsimple.com" target="_blank">realsimple.com</a></td></tr><tr class="text-center"><td>194</td><td><a class="underline" href="https://www.recettes.qc.ca" target="_blank">recettes.qc.ca</a></td></tr><tr class="text-center"><td>195</td><td><a class="underline" href="https://www.receitasnestle.com.br" target="_blank">receitasnestle.com.br</a></td></tr><tr class="text-center"><td>196</td><td><a class="underline" href="https://www.recipecommunity.com.au" target="_blank">recipecommunity.com.au</a></td></tr><tr class="text-center"><td>197</td><td><a class="underline" href="https://reciperunner.com" target="_blank">reciperunner.com</a></td></tr><tr class="text-center"><td>198</td><td><a class="underline" href="https://www.recipetineats.com" target="_blank">recipetineats.com</a></td></tr><tr class="text-center"><td>199</td><td><a class="underline" href="https://redhousespice.com" target="_blank">redhousespice.com</a></td></tr><tr class="text-center"><td>200</td><td><a class="underline" href="https://www.reishunger.de" target="_blank">reishunger.de</a></td></tr><tr class="text-center"><td>201</td><td><a class="underline" href="https://www.rezeptwelt.de" target="_blank">rezeptwelt.de</a></td></tr><tr class="text-center"><td>202</td><td><a class="underline" href="https://ricetta.it" target="_blank">ricetta.it</a></td></tr><tr class="text-center"><td>203</td><td><a class="underline" href="https://rosannapansino.com/blogs/recipes" target="_blank">rosannapansino.com</a></td></tr><tr class="text-center"><td>204</td><td><a class="underline" href="https://rutgerbakt.nl/alle-recepten" target="_blank">rutgerbakt.nl</a></td></tr><tr class="text-center"><td>205</td><td><a class="underline" href="https://www.saboresajinomoto.com.br" target="_blank">saboresajinomoto.com.br</a></td></tr><tr class="text-center"><td>206</td><td><a class="underline" href="https://sallysbakingaddiction.com" target="_blank">sallysbakingaddiction.com</a></td></tr><tr class="text-center"><td>207</td><td><a class="underline" href="https://sallys-blog.de/rezepte" target="_blank">sallys-blog.de</a></td></tr><tr class="text-center"><td>208</td><td><a class="underline" href="https://saltpepperskillet.com" target="_blank">saltpepperskillet.com</a></td></tr><tr class="text-center"><td>209</td><td><a class="underline" href="https://www.saveur.com" target="_blank">saveur.com</a></td></tr><tr class="text-center"><td>210</td><td><a class="underline" href="https://www.seriouseats.com" target="_blank">seriouseats.com</a></td></tr><tr class="text-center"><td>211</td><td><a class="underline" href="https://simple-veganista.com" target="_blank">simple-veganista.com</a></td></tr><tr class="text-center"><td>212</td><td><a class="underline" href="https://www.simply-cookit.com" target="_blank">simply-cookit.com</a></td></tr><tr class="text-center"><td>213</td><td><a class="underline" href="https://www.simplyquinoa.com" target="_blank">simplyquinoa.com</a></td></tr><tr class="text-center"><td>214</td><td><a class="underline" href="https://www.simplyrecipes.com" target="_blank">simplyrecipes.com</a></td></tr><tr class="text-center"><td>215</td><td><a class="underline" href="https://www.simplywhisked.com" target="_blank">simplywhisked.com</a></td></tr><tr class="text-center"><td>216</td><td><a class="underline" href="https://www.skinnytaste.com" target="_blank">skinnytaste.com</a></td></tr><tr class="text-center"><td>217</td><td><a class="underline" href="https://sobors.hu/receptek" target="_blank">sobors.hu</a></td></tr><tr class="text-center"><td>218</td><td><a class="underline" href="https://southerncastiron.com/category/recipes" target="_blank">southerncastiron.com</a></td></tr><tr class="text-center"><td>219</td><td><a class="underline" href="https://www.southernliving.com" target="_blank">southernliving.com</a></td></tr><tr class="text-center"><td>220</td><td><a class="underline" href="https://www.spendwithpennies.com" target="_blank">spendwithpennies.com</a></td></tr><tr class="text-center"><td>221</td><td><a class="underline" href="https://www.staysnatched.com" target="_blank">staysnatched.com</a></td></tr><tr class="text-center"><td>222</td><td><a class="underline" href="https://steamykitchen.com" target="_blank">steamykitchen.com</a></td></tr><tr class="text-center"><td>223</td><td><a class="underline" href="https://streetkitchen.co" target="_blank">streetkitchen.co</a></td></tr><tr class="text-center"><td>224</td><td><a class="underline" href="https://sunbasket.com" target="_blank">sunbasket.com</a></td></tr><tr class="text-center"><td>225</td><td><a class="underline" href="https://sundpaabudget.dk" target="_blank">sundpaabudget.dk</a></td></tr><tr class="text-center"><td>226</td><td><a class="underline" href="https://www.sunset.com" target="_blank">sunset.com</a></td></tr><tr class="text-center"><td>227</td><td><a class="underline" href="https://sweetcsdesigns.com" target="_blank">sweetcsdesigns.com</a></td></tr><tr class="text-center"><td>228</td><td><a class="underline" href="https://sweetpeasandsaffron.com" target="_blank">sweetpeasandsaffron.com</a></td></tr><tr class="text-center"><td>229</td><td><a class="underline" href="https://www.tasteofhome.com" target="_blank">tasteofhome.com</a></td></tr><tr class="text-center"><td>230</td><td><a class="underline" href="https://tastesbetterfromscratch.com" target="_blank">tastesbetterfromscratch.com</a></td></tr><tr class="text-center"><td>231</td><td><a class="underline" href="https://www.tastesoflizzyt.com" target="_blank">tastesoflizzyt.com</a></td></tr><tr class="text-center"><td>232</td><td><a class="underline" href="https://tasty.co" target="_blank">tasty.co</a></td></tr><tr class="text-center"><td>233</td><td><a class="underline" href="https://tastykitchen.com" target="_blank">tastykitchen.com</a></td></tr><tr class="text-center"><td>234</td><td><a class="underline" href="https://realfood.tesco.com" target="_blank">tesco.com</a></td></tr><tr class="text-center"><td>235</td><td><a class="underline" href="https://www.theclevercarrot.com" target="_blank">theclevercarrot.com</a></td></tr><tr class="text-center"><td>236</td><td><a class="underline" href="https://www.thecookingguy.com" target="_blank">thecookingguy.com</a></td></tr><tr class="text-center"><td>237</td><td><a class="underline" href="https://theexpertguides.com" target="_blank">theexpertguides.com</a></td></tr><tr class="text-center"><td>238</td><td><a class="underline" href="https://thehappyfoodie.co.uk" target="_blank">thehappyfoodie.co.uk</a></td></tr><tr class="text-center"><td>239</td><td><a class="underline" href="https://thekitchencommunity.org/" target="_blank">thekitchencommunity.org</a></td></tr><tr class="text-center"><td>240</td><td><a class="underline" href="https://www.thekitchenmagpie.com" target="_blank">thekitchenmagpie.com</a></td></tr><tr class="text-center"><td>241</td><td><a class="underline" href="https://www.thekitchn.com" target="_blank">thekitchn.com</a></td></tr><tr class="text-center"><td>242</td><td><a class="underline" href="https://www.themagicalslowcooker.com/" target="_blank">themagicalslowcooker.com</a></td></tr><tr class="text-center"><td>243</td><td><a class="underline" href="https://themodernproper.com" target="_blank">themodernproper.com</a></td></tr><tr class="text-center"><td>244</td><td><a class="underline" href="https:///thenutritiouskitchen.co" target="_blank">thenutritiouskitchen.co</a></td></tr><tr class="text-center"><td>245</td><td><a class="underline" href="https://www.thepioneerwoman.com" target="_blank">thepioneerwoman.com</a></td></tr><tr class="text-center"><td>246</td><td><a class="underline" href="https://therecipecritic.com" target="_blank">therecipecritic.com</a></td></tr><tr class="text-center"><td>247</td><td><a class="underline" href="https://www.thespruceeats.com" target="_blank">thespruceeats.com</a></td></tr><tr class="text-center"><td>248</td><td><a class="underline" href="https://www.thevintagemixer.com" target="_blank">thevintagemixer.com</a></td></tr><tr class="text-center"><td>249</td><td><a class="underline" href="https://thewoksoflife.com" target="_blank">thewoksoflife.com</a></td></tr><tr class="text-center"><td>250</td><td><a class="underline" href="https://thinlicious.com/" target="_blank">thinlicious.com</a></td></tr><tr class="text-center"><td>251</td><td><a class="underline" href="https://tidymom.net" target="_blank">tidymom.net</a></td></tr><tr class="text-center"><td>252</td><td><a class="underline" href="https://recipes.timesofindia.com/recipes" target="_blank">timesofindia.com</a></td></tr><tr class="text-center"><td>253</td><td><a class="underline" href="https://www.tine.no/oppskrifter" target="_blank">tine.no</a></td></tr><tr class="text-center"><td>254</td><td><a class="underline" href="https://www.tudogostoso.com.br" target="_blank">tudogostoso.com</a></td></tr><tr class="text-center"><td>255</td><td><a class="underline" href="https://www.twopeasandtheirpod.com" target="_blank">twopeasandtheirpod.com</a></td></tr><tr class="text-center"><td>256</td><td><a class="underline" href="https://uitpaulineskeuken.nl" target="_blank">uitpaulineskeuken.nl</a></td></tr><tr class="text-center"><td>257</td><td><a class="underline" href="https://usapears.org" target="_blank">usapears.org</a></td></tr><tr class="text-center"><td>258</td><td><a class="underline" href="https://www.valdemarsro.dk" target="_blank">valdemarsro.dk</a></td></tr><tr class="text-center"><td>259</td><td><a class="underline" href="https://vanillaandbean.com" target="_blank">vanillaandbean.com</a></td></tr><tr class="text-center"><td>260</td><td><a class="underline" href="https://www.vegetarbloggen.no" target="_blank">vegetarbloggen.no</a></td></tr><tr class="text-center"><td>261</td><td><a class="underline" href="https://www.vegolosi.it" target="_blank">vegolosi.it</a></td></tr><tr class="text-center"><td>262</td><td><a class="underline" href="https://www.vegrecipesofindia.com" target="_blank">vegrecipesofindia.com</a></td></tr><tr class="text-center"><td>263</td><td><a class="underline" href="https://www.waitrose.com/" target="_blank">waitrose.com</a></td></tr><tr class="text-center"><td>264</td><td><a class="underline" href="https://www.watchwhatueat.com" target="_blank">watchwhatueat.com</a></td></tr><tr class="text-center"><td>265</td><td><a class="underline" href="https://wearenotmartha.com" target="_blank">wearenotmartha.com</a></td></tr><tr class="text-center"><td>266</td><td><a class="underline" href="https://www.weightwatchers.com" target="_blank">weightwatchers.com</a></td></tr><tr class="text-center"><td>267</td><td><a class="underline" href="https://www.wellplated.com" target="_blank">wellplated.com</a></td></tr><tr class="text-center"><td>268</td><td><a class="underline" href="https://whatsgabycooking.com" target="_blank">whatsgabycooking.com</a></td></tr><tr class="text-center"><td>269</td><td><a class="underline" href="https://www.wholefoodsmarket.co.uk/" target="_blank">wholefoodsmarket.co.uk</a></td></tr><tr class="text-center"><td>270</td><td><a class="underline" href="https://en.wikibooks.org" target="_blank">wikibooks.org</a></td></tr><tr class="text-center"><td>271</td><td><a class="underline" href="https://en.m.wikibooks.org" target="_blank">m.wikibooks.org</a></td></tr><tr class="text-center"><td>272</td><td><a class="underline" href="https://woop.co.nz" target="_blank">woop.co.nz</a></td></tr><tr class="text-center"><td>273</td><td><a class="underline" href="https://ye-mek.net" target="_blank">ye-mek.net</a></td></tr><tr class="text-center"><td>274</td><td><a class="underline" href="https://www.zeit.de" target="_blank">zeit.de</a></td></tr><tr class="text-center"><td>275</td><td><a class="underline" href="https://www.zenbelly.com" target="_blank">zenbelly.com</a></td></tr><tr class="text-center"><td>276</td><td><a class="underline" href="https://www.puurgezond.nl" target="_blank">puurgezond.nl</a></td></tr><tr class="text-center"><td>277</td><td><a class="underline" href="https://jaimyskitchen.nl" target="_blank">jaimyskitchen.nl</a></td></tr><tr class="text-center"><td>278</td><td><a class="underline" href="https://www.leukerecepten.nl" target="_blank">leukerecepten.nl</a></td></tr><tr class="text-center"><td>279</td><td><a class="underline" href="https://www.bettybossi.ch" target="_blank">bettybossi.ch</a></td></tr><tr class="text-center"><td>280</td><td><a class="underline" href="https://www.reddit.com" target="_blank">reddit.com</a></td></tr><tr class="text-center"><td>281</td><td><a class="underline" href="https://www.marmiton.org" target="_blank">marmiton.org</a></td></tr><tr class="text-center"><td>282</td><td><a class="underline" href="https://www.yumelise.fr" target="_blank">yumelise.fr</a></td></tr><tr class="text-center"><td>283</td><td><a class="underline" href="https://www.lidl-kochen.de/rezeptwelt" target="_blank">lidl-kochen.de</a></td></tr><tr class="text-center"><td>284</td><td><a class="underline" href="https://www.all-clad.com" target="_blank">all-clad.com</a></td></tr><tr class="text-center"><td>285</td><td><a class="underline" href="https://www.francescakookt.nl" target="_blank">francescakookt.nl</a></td></tr><tr class="text-center"><td>286</td><td><a class="underline" href="https://www.quitoque.fr" target="_blank">quitoque.fr</a></td></tr><tr class="text-center"><td>287</td><td><a class="underline" href="https://kuchynalidla.sk" target="_blank">kuchynalidla.sk</a></td></tr><tr class="text-center"><td>288</td><td><a class="underline" href="https://www.myjewishlearning.com" target="_blank">myjewishlearning.com</a></td></tr><tr class="text-center"><td>289</td><td><a class="underline" href="https://drinkoteket.se" target="_blank">drinkoteket.se</a></td></tr><tr class="text-center"><td>290</td><td><a class="underline" href="https://www.24kitchen.nl/recepten" target="_blank">24kitchen.nl</a></td></tr><tr class="text-center"><td>291</td><td><a class="underline" href="https://www.ah.be" target="_blank">ah.be</a></td></tr><tr class="text-center"><td>292</td><td><a class="underline" href="https://aflavorjournal.com" target="_blank">aflavorjournal.com</a></td></tr><tr class="text-center"><td>293</td><td><a class="underline" href="https://www.aldi.com.au/recipes" target="_blank">aldi.com.au</a></td></tr><tr class="text-center"><td>294</td><td><a class="underline" href="https://alexandracooks.com" target="_blank">alexandracooks.com</a></td></tr><tr class="text-center"><td>295</td><td><a class="underline" href="https://alittlebityummy.com" target="_blank">alittlebityummy.com</a></td></tr><tr class="text-center"><td>296</td><td><a class="underline" href="https://allthehealthythings.com" target="_blank">allthehealthythings.com</a></td></tr><tr class="text-center"><td>297</td><td><a class="underline" href="https://aniagotuje.pl" target="_blank">aniagotuje.pl</a></td></tr><tr class="text-center"><td>298</td><td><a class="underline" href="https://www.americastestkitchen.com" target="_blank">americastestkitchen.com</a></td></tr><tr class="text-center"><td>299</td><td><a class="underline" href="https://www.angielaeats.com" target="_blank">angielaeats.com</a></td></tr><tr class="text-center"><td>300</td><td><a class="underline" href="https://www.antilliaans-eten.nl" target="_blank">antilliaans-eten.nl</a></td></tr><tr class="text-center"><td>301</td><td><a class="underline" href="https://avocadoskillet.com" target="_blank">avocadoskillet.com</a></td></tr><tr class="text-center"><td>302</td><td><a class="underline" href="https://www.bakels.com.au" target="_blank">bakels.com.au</a></td></tr><tr class="text-center"><td>303</td><td><a class="underline" href="https://barefeetinthekitchen.com" target="_blank">barefeetinthekitchen.com</a></td></tr><tr class="text-center"><td>304</td><td><a class="underline" href="https://beyondkimchee.com" target="_blank">beyondkimchee.com</a></td></tr><tr class="text-center"><td>305</td><td><a class="underline" href="https://bottomlessgreens.com" target="_blank">bottomlessgreens.com</a></td></tr><tr class="text-center"><td>306</td><td><a class="underline" href="https://breadtopia.com" target="_blank">breadtopia.com</a></td></tr><tr class="text-center"><td>307</td><td><a class="underline" href="https://www.britishbakels.co.uk/recipes" target="_blank">britishbakels.co.uk</a></td></tr><tr class="text-center"><td>308</td><td><a class="underline" href="https://chatelaine.com" target="_blank">chatelaine.com</a></td></tr><tr class="text-center"><td>309</td><td><a class="underline" href="https://chejorge.com" target="_blank">chejorge.com</a></td></tr><tr class="text-center"><td>310</td><td><a class="underline" href="https://chetnamakan.co.uk" target="_blank">chetnamakan.co.uk</a></td></tr><tr class="text-center"><td>311</td><td><a class="underline" href="https://chinesecookingdemystified.substack.com" target="_blank">chinesecookingdemystified.substack.com</a></td></tr><tr class="text-center"><td>312</td><td><a class="underline" href="https://www.colruyt.be" target="_blank">colruyt.be</a></td></tr><tr class="text-center"><td>313</td><td><a class="underline" href="https://www.culy.nl" target="_blank">culy.nl</a></td></tr><tr class="text-center"><td>314</td><td><a class="underline" href="https://cuisineandtravel.com" target="_blank">cuisineandtravel.com</a></td></tr><tr class="text-center"><td>315</td><td><a class="underline" href="https://daringgourmet.com" target="_blank">daringgourmet.com</a></td></tr><tr class="text-center"><td>316</td><td><a class="underline" href="https://www.dherbs.com" target="_blank">dherbs.com</a></td></tr><tr class="text-center"><td>317</td><td><a class="underline" href="https://damndelicious.net" target="_blank">damndelicious.net</a></td></tr><tr class="text-center"><td>318</td><td><a class="underline" href="https://dinnerthendessert.com" target="_blank">dinnerthendessert.com</a></td></tr><tr class="text-center"><td>319</td><td><a class="underline" href="https://www.dinneratthezoo.com" target="_blank">dinneratthezoo.com</a></td></tr><tr class="text-center"><td>320</td><td><a class="underline" href="https://dish.co.nz" target="_blank">dish.co.nz</a></td></tr><tr class="text-center"><td>321</td><td><a class="underline" href="https://www.donnahay.com.au" target="_blank">donnahay.com.au</a></td></tr><tr class="text-center"><td>322</td><td><a class="underline" href="https://dreenaburton.com" target="_blank">dreenaburton.com</a></td></tr><tr class="text-center"><td>323</td><td><a class="underline" href="https://elephantasticvegan.com" target="_blank">elephantasticvegan.com</a></td></tr><tr class="text-center"><td>324</td><td><a class="underline" href="https://entertainingwithbeth.com" target="_blank">entertainingwithbeth.com</a></td></tr><tr class="text-center"><td>325</td><td><a class="underline" href="http://www.etenvaneefke.nl" target="_blank">etenvaneefke.nl</a></td></tr><tr class="text-center"><td>326</td><td><a class="underline" href="https://www.evolvingtable.com" target="_blank">evolvingtable.com</a></td></tr><tr class="text-center"><td>327</td><td><a class="underline" href="https://www.familyfoodonthetable.com" target="_blank">familyfoodonthetable</a></td></tr><tr class="text-center"><td>328</td><td><a class="underline" href="https://feastingathome.com" target="_blank">feastingathome.com</a></td></tr><tr class="text-center"><td>329</td><td><a class="underline" href="https://felix.kitchen" target="_blank">felix.kitchen</a></td></tr><tr class="text-center"><td>330</td><td><a class="underline" href="https://findingtimeforcooking.com" target="_blank">findingtimeforcooking.com</a></td></tr><tr class="text-center"><td>331</td><td><a class="underline" href="https://foodal.com" target="_blank">foodal.com</a></td></tr><tr class="text-center"><td>332</td><td><a class="underline" href="https://foodbymaria.com" target="_blank">foodbymaria.com</a></td></tr><tr class="text-center"><td>333</td><td><a class="underline" href="https://foodiecrush.com" target="_blank">foodiecrush.com</a></td></tr><tr class="text-center"><td>334</td><td><a class="underline" href="https://food-guide.canada.ca/en/recipes" target="_blank">food-guide.canada.ca</a></td></tr><tr class="text-center"><td>335</td><td><a class="underline" href="https://foolproofliving.com" target="_blank">foolproofliving.com</a></td></tr><tr class="text-center"><td>336</td><td><a class="underline" href="https://gastroplant.com" target="_blank">gastroplant.com</a></td></tr><tr class="text-center"><td>337</td><td><a class="underline" href="https://www.gazoakleychef.com" target="_blank">gazoakleychef.com</a></td></tr><tr class="text-center"><td>338</td><td><a class="underline" href="https://glutenfreetables.com" target="_blank">glutenfreetables.com</a></td></tr><tr class="text-center"><td>339</td><td><a class="underline" href="https://goodeatings.com" target="_blank">goodeatings.com</a></td></tr><tr class="text-center"><td>340</td><td><a class="underline" href="https://goodto.com" target="_blank">goodto.com</a></td></tr><tr class="text-center"><td>341</td><td><a class="underline" href="https://www.gourmettraveller.com.au" target="_blank">gourmettraveller.com.au</a></td></tr><tr class="text-center"><td>342</td><td><a class="underline" href="https://www.gousto.co.uk" target="_blank">gousto.co.uk</a></td></tr><tr class="text-center"><td>343</td><td><a class="underline" href="https://greenevi.com" target="_blank">greenevi.com</a></td></tr><tr class="text-center"><td>344</td><td><a class="underline" href="https://gurki.no" target="_blank">gurki.no</a></td></tr><tr class="text-center"><td>345</td><td><a class="underline" href="https://www.healthylittlefoodies.com" target="_blank">healthylittlefoodies.com</a></td></tr><tr class="text-center"><td>346</td><td><a class="underline" href="https://www.hellofresh.se/recipes" target="_blank">hellofresh.se</a></td></tr><tr class="text-center"><td>347</td><td><a class="underline" href="https://homebrewanswers.com" target="_blank">homebrewanswers.com</a></td></tr><tr class="text-center"><td>348</td><td><a class="underline" href="https://inbloombakery.com" target="_blank">inbloombakery.com</a></td></tr><tr class="text-center"><td>349</td><td><a class="underline" href="https://instantpot.com" target="_blank">instantpot.com</a></td></tr><tr class="text-center"><td>350</td><td><a class="underline" href="https://jaroflemons.com" target="_blank">jaroflemons.com</a></td></tr><tr class="text-center"><td>351</td><td><a class="underline" href="https://www.jocooks.com" target="_blank">jocooks.com</a></td></tr><tr class="text-center"><td>352</td><td><a class="underline" href="https://joythebaker.com" target="_blank">joythebaker.com</a></td></tr><tr class="text-center"><td>353</td><td><a class="underline" href="https://www.jumbo.com/recepten" target="_blank">jumbo.com</a></td></tr><tr class="text-center"><td>354</td><td><a class="underline" href="https://keepinitkind.com" target="_blank">keepinitkind.com</a></td></tr><tr class="text-center"><td>355</td><td><a class="underline" href="https://kitchenaid.com.au" target="_blank">kitchenaid.com</a></td></tr><tr class="text-center"><td>356</td><td><a class="underline" href="https://www.kitchensanctuary.com" target="_blank">kitchensanctuary.com</a></td></tr><tr class="text-center"><td>357</td><td><a class="underline" href="https://www.kookjij.nl" target="_blank">kookjij.nl</a></td></tr><tr class="text-center"><td>358</td><td><a class="underline" href="https://kristineskitchenblog.com" target="_blank">kristineskitchenblog.com</a></td></tr><tr class="text-center"><td>359</td><td><a class="underline" href="https://www.lahbco.com" target="_blank">lahbco.com</a></td></tr><tr class="text-center"><td>360</td><td><a class="underline" href="https://www.lekkerensimpel.com/gougeres" target="_blank">lekkerensimpel.com</a></td></tr><tr class="text-center"><td>361</td><td><a class="underline" href="https://recepten.lidl.nl/recept" target="_blank">lidl.nl</a></td></tr><tr class="text-center"><td>362</td><td><a class="underline" href="https://lithuanianintheusa.com" target="_blank">lithuanianintheusa.com</a></td></tr><tr class="text-center"><td>363</td><td><a class="underline" href="https://www.loveandlemons.com/" target="_blank">loveandlemons.com</a></td></tr><tr class="text-center"><td>364</td><td><a class="underline" href="https://www.madewithlau.com" target="_blank">madewithlau.com</a></td></tr><tr class="text-center"><td>365</td><td><a class="underline" href="https://www.mccormick.com" target="_blank">mccormick.com</a></td></tr><tr class="text-center"><td>366</td><td><a class="underline" href="https://mexicanmademeatless.com" target="_blank">mexicanmademeatless.com</a></td></tr><tr class="text-center"><td>367</td><td><a class="underline" href="https://www.modernhoney.com" target="_blank">modernhoney.com</a></td></tr><tr class="text-center"><td>368</td><td><a class="underline" href="https://www.momontimeout.com" target="_blank">momontimeout.com</a></td></tr><tr class="text-center"><td>369</td><td><a class="underline" href="https://www.mygingergarlickitchen.com" target="_blank">mygingergarlickitchen.com</a></td></tr><tr class="text-center"><td>370</td><td><a class="underline" href="https://mykoreankitchen.com" target="_blank">mykoreankitchen.com</a></td></tr><tr class="text-center"><td>371</td><td><a class="underline" href="https://natashaskitchen.com" target="_blank">natashaskitchen.com</a></td></tr><tr class="text-center"><td>372</td><td><a class="underline" href="https://nigella.com" target="_blank">nigella.com</a></td></tr><tr class="text-center"><td>373</td><td><a class="underline" href="https://www.notenoughcinnamon.com" target="_blank">notenoughcinnamon.com</a></td></tr><tr class="text-center"><td>374</td><td><a class="underline" href="https://ohmyveggies.com" target="_blank">ohmyveggies.com</a></td></tr><tr class="text-center"><td>375</td><td><a class="underline" href="https://www.okokorecepten.nl" target="_blank">okokorecepten.nl</a></td></tr><tr class="text-center"><td>376</td><td><a class="underline" href="https://onesweetappetite.com" target="_blank">onesweetappetite.com</a></td></tr><tr class="text-center"><td>377</td><td><a class="underline" href="https://parsleyandparm.com" target="_blank">parsleyandparm.com</a></td></tr><tr class="text-center"><td>378</td><td><a class="underline" href="https://plentyvegan.com" target="_blank">plentyvegan.com</a></td></tr><tr class="text-center"><td>379</td><td><a class="underline" href="https://potatorolls.com" target="_blank">potatorolls.com</a></td></tr><tr class="text-center"><td>380</td><td><a class="underline" href="https://www.purewow.com" target="_blank">purewow.com</a></td></tr><tr class="text-center"><td>381</td><td><a class="underline" href="https://www.radiofrance.fr" target="_blank">radiofrance.fr</a></td></tr><tr class="text-center"><td>382</td><td><a class="underline" href="https://www.recipegirl.com" target="_blank">recipegirl.com</a></td></tr><tr class="text-center"><td>383</td><td><a class="underline" href="https://robinasbell.com" target="_blank">robinasbell.com</a></td></tr><tr class="text-center"><td>384</td><td><a class="underline" href="https://www.saltandlavender.com" target="_blank">saltandlavender.com</a></td></tr><tr class="text-center"><td>385</td><td><a class="underline" href="https://sarahsveganguide.com" target="_blank">sarahsveganguide.com</a></td></tr><tr class="text-center"><td>386</td><td><a class="underline" href="https://www.savorynothings.com" target="_blank">savorynothings.com</a></td></tr><tr class="text-center"><td>387</td><td><a class="underline" href="https://smittenkitchen.com" target="_blank">smittenkitchen.com</a></td></tr><tr class="text-center"><td>388</td><td><a class="underline" href="https://spiceboxtravels.com" target="_blank">spiceboxtravels.com</a></td></tr><tr class="text-center"><td>389</td><td><a class="underline" href="https://www.tasteatlas.com" target="_blank">tasteatlas.com</a></td></tr><tr class="text-center"><td>390</td><td><a class="underline" href="https://www.thatvegandad.net" target="_blank">thatvegandad.com</a></td></tr><tr class="text-center"><td>391</td><td><a class="underline" href="https://www.thecookierookie.com" target="_blank">thecookierookie.com</a></td></tr><tr class="text-center"><td>392</td><td><a class="underline" href="https://thefoodflamingo.com" target="_blank">thefoodflamingo.com</a></td></tr><tr class="text-center"><td>393</td><td><a class="underline" href="https://www.theguccha.com" target="_blank">theguccha.com</a></td></tr><tr class="text-center"><td>394</td><td><a class="underline" href="https://theheartysoul.com" target="_blank">theheartysoul.com</a></td></tr><tr class="text-center"><td>395</td><td><a class="underline" href="https://thesaltymarshmallow.com" target="_blank">thesaltymarshmallow.com</a></td></tr><tr class="text-center"><td>396</td><td><a class="underline" href="https://twosleevers.com" target="_blank">twosleevers.com</a></td></tr><tr class="text-center"><td>397</td><td><a class="underline" href="https://unsophisticook.com" target="_blank">unsophisticook.com</a></td></tr><tr class="text-center"><td>398</td><td><a class="underline" href="https://vegan-pratique.fr/recettes/banana-bread/" target="_blank">vegan-pratique.fr</a></td></tr>"#);
            Ok(())
        }
    }
}
