use std::io::Cursor;
use std::str::FromStr;

use axum::extract::multipart::{InvalidBoundary, MultipartRejection};
use axum::extract::{DefaultBodyLimit, FromRequest, Multipart, Request};
use axum::routing::{get, post};
use axum::{Router, middleware};

use integrations::{App, FileFormat, parse_recipe};
use recipe_schema::RecipeSchema;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

use crate::handlers::recipes::{
    add_manual_recipe_handler, add_manual_recipe_post_handler, add_recipe_import_handler,
    add_recipes_handler, add_website_post_handler, delete_recipe_categories_handler,
    delete_recipe_handler, duplicate_recipe_handler, edit_recipe_handler, edit_recipe_put_handler,
    post_recipe_categories_handler, recipes_handler, scale_recipe_handler, search_recipes_handler,
    share_recipe_post_handler, supported_applications_handler, supported_websites_handler,
    view_recipe_handler,
};
use crate::middleware::mw_auth;
use crate::{AppState, Result as ServerResult};

/// Represents the content of the "Add Recipe -> Import from an app" form.
#[derive(Default)]
pub struct ImportFromAppForm {
    pub file_data: Vec<u8>,
    pub file_name: String,
    pub app: App,
    pub file_format: FileFormat,
}

impl<S> FromRequest<S> for ImportFromAppForm
where
    S: Send + Sync,
{
    type Rejection = MultipartRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let mut multipart = Multipart::from_request(req, state).await?;

        let mut form = ImportFromAppForm::default();

        while let Some(field) = multipart.next_field().await.map_err(|err| {
            error!("Failed to read multipart field in import recipes from app: {err}");
            InvalidBoundary::default()
        })? {
            let name = field.name().unwrap_or("");
            match name {
                "app" => {
                    let app_name = field.text().await.map_err(|_| InvalidBoundary::default())?;
                    let app = App::from_str(&app_name.to_lowercase()).unwrap_or_default();
                    if matches!(app, App::Unknown) {
                        error!("Import recipes from app form field 'app' is invalid: {app_name}");
                        return Err(InvalidBoundary::default())?;
                    }
                    form.app = app;
                }
                "file" => {
                    let filename = field
                        .file_name()
                        .map(|s| s.to_string())
                        .ok_or(InvalidBoundary::default())?;
                    form.file_name = filename.clone();

                    form.file_data = field
                        .bytes()
                        .await
                        .map_err(|err| {
                            error!("Failed to read file bytes for '{filename}': {err}");
                            InvalidBoundary::default()
                        })?
                        .to_vec();

                    form.file_format = FileFormat::from_filename(&filename);
                }
                _ => {
                    warn!("Import recipes from app form field '{name}' is not processed")
                }
            }
        }

        info!(
            "Importing recipes from '{}' using {} parser (format: {})",
            form.file_name, form.app, form.file_format
        );
        Ok(form)
    }
}

impl ImportFromAppForm {
    /// Parses the recipe file contained in the form.
    pub fn parse_recipes(&mut self) -> ServerResult<Vec<RecipeSchema>> {
        let mut data = Cursor::new(&self.file_data);
        let app = &self.app;
        let file_name = &self.file_name;
        let file_format = &self.file_format;
        Ok(parse_recipe(&mut data, app, file_name, file_format)?)
    }
}

/// Represents the content of the share recipe form.
#[derive(Deserialize, Serialize)]
pub struct ShareRecipeForm {
    pub datetime: Option<String>,
}

/// Represents the content of a recipe category form.
#[derive(Deserialize, Serialize)]
pub struct RecipeCategoryForm {
    pub category: String,
}

/// Represents the content of a fetch recipe from URLs form.
#[derive(Deserialize, Serialize)]
pub struct RecipeScrapeForm {
    pub urls: String,
}

/// Defines the routes for endpoints related to recipes.
pub(super) fn recipes_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(recipes_handler))
        .route(
            "/{:recipe_id}",
            get(view_recipe_handler).delete(delete_recipe_handler),
        )
        .route("/{:recipe_id}/duplicate", get(duplicate_recipe_handler))
        .route(
            "/{:recipe_id}/edit",
            get(edit_recipe_handler)
                .put(edit_recipe_put_handler)
                .layer(DefaultBodyLimit::max(1024 * 512)),
        )
        .route("/{:recipe_id}/scale", get(scale_recipe_handler))
        .route("/{:recipe_id}/share", post(share_recipe_post_handler))
        .route("/add", get(add_recipes_handler))
        .route(
            "/add/import",
            post(add_recipe_import_handler).layer(DefaultBodyLimit::max(50 * 1024 * 1024)),
        )
        .route(
            "/add/manual",
            get(add_manual_recipe_handler)
                .post(add_manual_recipe_post_handler)
                .layer(DefaultBodyLimit::max(1024 * 512)),
        )
        .route("/add/website", post(add_website_post_handler))
        .route(
            "/categories",
            post(post_recipe_categories_handler).delete(delete_recipe_categories_handler),
        )
        .route("/search", get(search_recipes_handler))
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
    use axum::http::Method;
    use axum_test::multipart::{MultipartForm, Part};

    use config::Config;
    use models::recipe::RecipeForCreate;
    use testing::utils::{
        TestDb, assert_html, assert_must_be_logged_in, assert_ws_message, build_server_logged_in,
        build_server_ws, create_app_state,
    };

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_add {
        use super::*;

        use axum_test::TestResponse;

        use testing::utils::{assert_html, assert_must_be_logged_in};

        const BASE_URI: &str = "/recipes/add";

        #[tokio::test]
        async fn test_add_recipe_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, BASE_URI).await
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
                    r##"<dialog class="modal" id="supported-apps-import-dialog"><div class="modal-box h-2/3"><form method="dialog"><button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button></form><h3 class="mb-1"><label class="floating-label"><input type="search" placeholder="Search an application" class="input input-sm w-11/12" _="on input show <tbody>tr/> in next <table/> when its textContent.toLowerCase() contains my value.toLowerCase()"></label></h3><div class="overflow-x-auto"><table class="table table-zebra table-sm"><thead><tr class="text-center"><th class="py-1">Number</th><th class="py-1">Application</th><th class="py-1">File Formats</th></tr></thead><tbody id="application-results"></tbody></table></div></div></dialog>"##,
                    r##"<dialog class="modal" id="add-ocr-dialog"><div class="modal-box"><form method="dialog"><button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button></form><h3 class="font-bold text-lg">Scan Recipe</h3><form class="py-4" hx-post="/recipes/add/ocr" hx-encoding="multipart/form-data" hx-indicator="#fullscreen-loader" hx-swap="none" _="on submit call document.querySelector('#add-ocr-dialog').close()"><div class="grid mb-4"><label for="add-ocr-files-input" class="floating-label text-sm font-medium mb-1">Select your recipe's images ordered by page or a recipe document in the PDF format.</label><input id="add-ocr-files-input" type="file" name="files" accept=".jpg, .jpeg, .png, .bmp, .tiff, .heif, .pdf" multiple class="p-2 border border-gray-300 rounded-lg shadow focus:ring-2 focus:ring-purple-600 dark:bg-gray-900 dark:border-none"></div><button class="btn btn-block btn-primary btn-sm">Submit</button></form></div></dialog>"##,
                    r##"<dialog class="modal" id="import-recipes-dialog"><div class="modal-box w-fit"><form method="dialog"><button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button></form><h3 class="font-bold text-lg">Import Recipes</h3><form class="py-4" hx-post="/recipes/add/import" enctype="multipart/form-data" hx-indicator="#fullscreen-loader" hx-swap="none" hx-on:htmx:before-request="if(!this.checkValidity()) return false; document.querySelector('#import-recipes-dialog').close()"><div><div class="grid mb-4"><label for="import-dialog-file" class="floating-label text-sm font-semibold mb-1">Select a file</label><input id="import-dialog-file" type="file" name="file" required accept=".cook,.crumb,.json,.mcb,.md,.mmf,.mx2,.mxp,.mz2,.mm,.mmf,.paprikarecipes,.rzk,.rk,.rzk,.txt,.xml" class="file-input"></div><div><label for="app-select" class="floating-label text-sm font-semibold mb-1">Select the application</label><select class="select" id="app-select" name="app"><option value="accuchef">AccuChef</option><option value="bigoven">BigOven</option><option value="cheftap">ChefTap</option><option value="cooklang">Cooklang</option><option value="cookmate">CookMate</option><option value="crouton">Crouton</option><option value="kalorio">Kalorio</option><option value="mastercook">MasterCook</option><option value="mealmaster">MealMaster</option><option value="paprika">Paprika</option><option value="recipemd">RecipeMD</option><option value="recipesage">RecipeSage</option><option value="rezkonv">Rezkonv</option><option value="saffron">Saffron</option></select></div></div><button type="submit" class="btn btn-block btn-primary btn-sm mt-4">Submit</button></form></div></dialog>"##,
                ],
            )
        }
    }

    mod tests_duplicate_recipes {
        use super::*;
        use models::recipe::test_utils::a_complete_recipe_for_create;

        use axum_test::TestResponse;

        use models::Recipe;
        use testing::utils::{
            assert_must_be_logged_in, assert_ws_message, build_server_ws, create_app_state,
        };

        fn base_uri(id: i64) -> String {
            format!("/recipes/{id}/duplicate")
        }

        #[tokio::test]
        async fn test_get_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, &base_uri(1)).await
        }

        #[tokio::test]
        async fn test_get_recipe_does_not_exist() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;

            let res = server.get(&base_uri(99)).await;

            res.assert_status_not_found();
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Recipe not found.","status":"alert-error","title":"Operation Failed"}}"#).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_get_recipe_exists() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let _ = Recipe::create(&state.mm, 1, &a_complete_recipe_for_create()).await?;

            let res = server.get(&base_uri(1)).await;

            assert_recipe_form(res);
            Ok(())
        }

        fn assert_recipe_form(res: TestResponse) {
            let expected = [
                r#"<title hx-swap-oob="true">Add Recipe Manually | Recipya</title>"#,
                r##"<form class="card-body" style="padding: 0" enctype="multipart/form-data" hx-post="/recipes/add/manual" hx-indicator="#fullscreen-loader">"##,
                r#"<input required type="text" name="title" placeholder="Title of the recipe*" autocomplete="off" class="input w-full text-center rounded-t-lg rounded-b-none bg-base-200" value="Best Chinese Kale (copy)">"#,
                r#"<img src="" alt="" class="object-cover mb-2 w-full max-h-[39rem]"><span class="grid gap-1 max-w-sm" style="margin: auto auto 0.25rem;"><div class="mr-1"><input type="file" accept="image/*,video/*" name="media" class="file-input file-input-sm file-input-bordered w-full max-w-sm""#,
                r#"<input id="servings" type="number" min="1" name="yield" value="4" class="input input-sm w-11/12">"#,
                r#"<input id="category" type="text" list="categories" name="category" class="input input-sm w-11/12" placeholder="Breakfast" autocomplete="off" value="dinner"><datalist id="categories"><option>uncategorized</option><option>appetizers</option><option>bread</option><option>breakfasts</option><option>condiments</option><option>dessert</option><option>lunch</option><option>main dish</option><option>salad</option><option>side dish</option><option>snacks</option><option>soups</option><option>stews</option><option>dinner</option></datalist>"#,
                r#"<textarea name="description" placeholder="This Thai curry chicken will make you drool." class="textarea w-full h-full resize-none">This is the most delicious recipe!</textarea>"#,
                r#"<div class="flex justify-self-center items-center gap-1 cursor-default" title="Cooking time"><span data-tip="Cook time" class="tooltip tooltip-left"><svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="24px" height="23px" viewBox="0 0 24 23" version="1.1"><g id="surface1"><path style=" stroke:none;fill-rule:nonzero;fill:rgb(62.745098%,64.705882%,65.882353%);fill-opacity:1;" d="M 4.636719 10.984375 L 4.636719 20.417969 C 4.636719 21.007812 5.078125 21.527344 5.667969 21.527344 L 18.257812 21.527344 C 18.847656 21.527344 19.363281 21.007812 19.363281 20.417969 L 19.363281 10.984375 Z M 4.636719 10.984375 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(76.862745%,76.862745%,76.862745%);fill-opacity:1;" d="M 19.289062 9.953125 C 18.992188 8.699219 17.964844 7.8125 16.710938 7.8125 L 7.214844 7.8125 C 5.964844 7.8125 4.933594 8.699219 4.710938 9.953125 Z M 19.289062 9.953125 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(54.509804%,69.411765%,81.960784%);fill-opacity:1;" d="M 16.710938 7.8125 L 13.914062 7.8125 C 14.945312 7.8125 15.828125 8.476562 16.269531 9.363281 C 16.417969 9.730469 16.785156 9.953125 17.226562 9.953125 L 19.289062 9.953125 C 18.992188 8.699219 17.964844 7.8125 16.710938 7.8125 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(0.392157%,26.666667%,38.823529%);fill-opacity:1;" d="M 22.160156 9.953125 L 20.320312 9.953125 C 20.097656 8.183594 18.550781 6.78125 16.710938 6.78125 L 15.535156 6.78125 L 15.3125 5.898438 C 15.09375 5.160156 14.503906 4.644531 13.765625 4.644531 L 11.191406 4.644531 C 10.453125 4.644531 9.792969 5.160156 9.644531 5.898438 L 9.421875 6.78125 L 7.214844 6.78125 C 5.449219 6.78125 3.902344 8.183594 3.605469 9.953125 L 1.765625 9.953125 C 1.03125 9.953125 0.441406 10.542969 0.441406 11.277344 C 0.441406 11.5 0.441406 11.722656 0.589844 11.941406 L 1.25 13.269531 C 1.546875 13.785156 2.0625 14.152344 2.648438 14.152344 L 3.605469 14.152344 L 3.605469 20.417969 C 3.605469 21.597656 4.492188 22.558594 5.667969 22.558594 L 18.257812 22.558594 C 19.4375 22.558594 20.394531 21.597656 20.394531 20.417969 L 20.394531 14.152344 L 21.351562 14.152344 C 21.9375 14.152344 22.453125 13.859375 22.75 13.269531 L 23.410156 11.867188 C 23.558594 11.648438 23.558594 11.5 23.558594 11.277344 C 23.558594 10.542969 22.894531 9.953125 22.160156 9.953125 M 3.605469 13.121094 L 2.648438 13.121094 C 2.429688 13.121094 2.28125 12.972656 2.136719 12.753906 L 1.472656 11.5 L 1.472656 11.277344 C 1.472656 11.132812 1.621094 10.984375 1.765625 10.984375 L 3.605469 10.984375 Z M 10.75 6.117188 C 10.75 5.972656 10.96875 5.75 11.265625 5.75 L 13.839844 5.75 C 14.0625 5.75 14.28125 5.898438 14.355469 6.117188 L 14.503906 6.78125 L 10.601562 6.78125 Z M 7.214844 7.8125 L 16.710938 7.8125 C 17.964844 7.8125 18.992188 8.699219 19.289062 9.953125 L 4.710938 9.953125 C 4.933594 8.699219 5.964844 7.8125 7.214844 7.8125 M 19.363281 13.636719 L 19.363281 20.417969 C 19.363281 21.007812 18.847656 21.527344 18.257812 21.527344 L 5.667969 21.527344 C 5.078125 21.527344 4.636719 21.007812 4.636719 20.417969 L 4.636719 10.984375 L 19.363281 10.984375 Z M 22.453125 11.5 L 21.71875 12.828125 C 21.644531 12.972656 21.496094 13.121094 21.277344 13.121094 L 20.394531 13.121094 L 20.394531 11.058594 L 22.160156 11.058594 C 22.308594 11.058594 22.453125 11.207031 22.453125 11.351562 L 22.453125 11.5 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(92.54902%,94.117647%,97.647059%);fill-opacity:1;" d="M 7.804688 17.324219 C 8.089844 17.324219 8.320312 17.554688 8.320312 17.839844 C 8.320312 18.125 8.089844 18.355469 7.804688 18.355469 C 7.519531 18.355469 7.289062 18.125 7.289062 17.839844 C 7.289062 17.554688 7.519531 17.324219 7.804688 17.324219 M 7.804688 16.808594 C 7.4375 16.808594 7.214844 16.585938 7.214844 16.21875 L 7.214844 13.121094 C 7.214844 12.753906 7.4375 12.53125 7.804688 12.53125 C 8.097656 12.53125 8.320312 12.753906 8.320312 13.121094 L 8.320312 16.21875 C 8.320312 16.585938 8.097656 16.808594 7.804688 16.808594 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(54.509804%,69.411765%,81.960784%);fill-opacity:1;" d="M 16.195312 12.015625 L 16.195312 19.902344 C 16.195312 20.492188 15.679688 21.007812 15.09375 21.007812 L 6.699219 21.007812 C 6.183594 21.007812 5.667969 20.492188 5.667969 19.902344 L 5.667969 12.015625 C 5.667969 11.5 5.226562 10.984375 4.636719 10.984375 L 4.636719 20.417969 C 4.636719 21.007812 5.078125 21.527344 5.667969 21.527344 L 18.257812 21.527344 C 18.847656 21.527344 19.363281 21.007812 19.363281 20.417969 L 19.363281 10.984375 L 17.226562 10.984375 C 16.636719 10.984375 16.195312 11.5 16.195312 12.015625 M 8.246094 7.8125 L 7.214844 7.8125 C 5.964844 7.8125 4.933594 8.699219 4.710938 9.953125 L 4.933594 9.953125 C 5.375 9.953125 5.742188 9.730469 5.890625 9.363281 C 6.332031 8.476562 7.214844 7.8125 8.246094 7.8125 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(0.392157%,26.666667%,38.823529%);fill-opacity:1;" d="M 18.773438 5.75 C 18.625 5.75 18.480469 5.675781 18.40625 5.527344 C 18.183594 5.308594 18.257812 4.9375 18.40625 4.792969 C 18.699219 4.644531 18.773438 4.421875 18.773438 4.128906 C 18.773438 3.90625 18.699219 3.6875 18.480469 3.539062 C 18.257812 3.316406 18.257812 3.023438 18.40625 2.800781 C 18.625 2.582031 18.992188 2.507812 19.140625 2.726562 C 19.582031 3.097656 19.878906 3.613281 19.878906 4.128906 C 19.878906 4.71875 19.582031 5.234375 19.140625 5.601562 L 18.773438 5.75 M 18.773438 1.03125 C 19.058594 1.03125 19.289062 1.261719 19.289062 1.546875 C 19.289062 1.832031 19.058594 2.0625 18.773438 2.0625 C 18.488281 2.0625 18.257812 1.832031 18.257812 1.546875 C 18.257812 1.261719 18.488281 1.03125 18.773438 1.03125 M 16.710938 5.75 L 16.269531 5.527344 C 16.050781 5.308594 16.121094 4.9375 16.34375 4.792969 C 16.5625 4.644531 16.710938 4.421875 16.710938 4.128906 C 16.710938 3.90625 16.5625 3.6875 16.417969 3.539062 C 15.902344 3.097656 15.679688 2.652344 15.679688 2.0625 C 15.679688 1.472656 15.902344 1.03125 16.417969 0.589844 C 16.5625 0.367188 16.933594 0.441406 17.152344 0.664062 C 17.300781 0.8125 17.300781 1.179688 17.078125 1.402344 C 16.785156 1.546875 16.710938 1.769531 16.710938 2.0625 C 16.710938 2.285156 16.785156 2.507812 17.007812 2.652344 C 17.519531 3.097656 17.742188 3.613281 17.742188 4.128906 C 17.742188 4.71875 17.519531 5.234375 17.007812 5.601562 L 16.710938 5.75 "></path></g></svg></span><label><input type="text" name="time-cook" value="01:00:00" class="input input-xs max-w-24 html-duration-picker"></label></div>"#,
                r#"<div class="flex justify-self-center items-center gap-1 cursor-default" title="Prep time"><span data-tip="Prep time" class="tooltip tooltip-left"><svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="24px" height="14px" viewBox="0 0 23 14" version="1.1"><defs><linearGradient id="linear0" gradientUnits="userSpaceOnUse" x1="-125.300003" y1="85.900002" x2="-64.599998" y2="85.900002" gradientTransform="matrix(0.000000000000000013,-0.225806,0.219048,0.000000000000000014,-7.447619,-14.451613)"><stop offset="0.1" style="stop-color:rgb(67.058824%,23.921569%,8.235294%);stop-opacity:1;"></stop><stop offset="0.5" style="stop-color:rgb(78.431373%,51.372549%,30.588235%);stop-opacity:1;"></stop><stop offset="0.8" style="stop-color:rgb(90.196078%,77.254902%,53.333333%);stop-opacity:1;"></stop><stop offset="1" style="stop-color:rgb(94.509804%,87.45098%,62.352941%);stop-opacity:1;"></stop></linearGradient></defs><g id="surface1"><path style=" stroke:none;fill-rule:evenodd;fill:rgb(95.294118%,82.745099%,64.705884%);fill-opacity:1;" d="M 0 8.128906 L 0.21875 6.324219 C 0.4375 5.644531 0.65625 5.195312 1.3125 4.96875 L 8.542969 2.484375 L 8.542969 1.804688 L 8.980469 0.675781 L 9.855469 0.453125 L 10.734375 0.453125 L 10.953125 0.675781 L 12.265625 1.128906 C 13.003906 1 13.761719 1.078125 14.457031 1.355469 C 15.125 1.453125 15.785156 1.601562 16.429688 1.804688 L 17.523438 1.804688 L 18.617188 0.902344 L 20.589844 0.453125 C 21.246094 0.675781 21.6875 0.902344 21.90625 1.355469 C 22.34375 1.804688 22.5625 2.710938 22.34375 4.066406 L 22.125 4.515625 C 22.5625 4.742188 22.78125 5.195312 22.78125 5.644531 L 22.78125 7.675781 L 22.125 8.804688 L 21.027344 9.484375 L 17.304688 11.289062 L 16.210938 11.742188 L 12.921875 13.324219 L 12.046875 13.773438 L 11.390625 14 L 10.078125 14 L 8.542969 13.546875 C 6.269531 12.441406 4.007812 11.308594 1.753906 10.160156 L 0.65625 9.257812 C 0.21875 9.03125 0 8.582031 0 8.128906 Z M 0 8.128906 "></path><path style=" stroke:none;fill-rule:evenodd;fill:url(#linear0);" d="M 1.3125 4.742188 L 8.542969 2.03125 L 8.542969 1.582031 L 8.980469 0.453125 C 9.199219 0.226562 9.636719 0 9.855469 0.226562 L 10.953125 0.226562 L 12.265625 0.902344 C 13.003906 0.773438 13.761719 0.851562 14.457031 1.128906 L 15.769531 1.355469 C 16.308594 1.65625 16.933594 1.738281 17.523438 1.582031 L 17.523438 1.355469 C 17.742188 1.128906 18.179688 0.675781 18.617188 0.675781 C 19.277344 0.226562 19.933594 0.226562 20.589844 0.226562 C 21.027344 0.453125 21.6875 0.675781 21.90625 1.128906 C 22.34375 1.582031 22.5625 2.484375 22.34375 3.839844 L 22.125 4.289062 C 22.5625 4.515625 22.78125 4.96875 22.78125 5.417969 L 22.78125 6.546875 L 22.5625 7.453125 L 22.125 8.582031 L 21.027344 9.257812 L 17.085938 11.289062 L 16.210938 11.515625 L 12.921875 13.097656 L 12.046875 13.546875 L 11.390625 13.773438 L 9.855469 13.773438 L 8.324219 13.324219 C 6.121094 12.21875 3.929688 11.089844 1.753906 9.933594 L 1.535156 9.933594 L 0.4375 9.03125 L 0 7.902344 C 0 7.292969 0.0742188 6.6875 0.21875 6.097656 C 0.21875 5.417969 0.65625 4.96875 1.3125 4.742188 Z M 1.3125 4.742188 "></path><path style="fill:none;stroke-width:0.3;stroke-linecap:butt;stroke-linejoin:miter;stroke:rgb(95.294118%,82.745099%,64.705884%);stroke-opacity:1;stroke-miterlimit:4;" d="M 5.991848 21.001116 L 39.999151 8.995536 L 39.00051 6.00279 L 40.997792 2.006696 L 46.008832 0 L 47.007473 0 L 49.004755 1.003348 L 50.003397 1.003348 L 55.995245 3.996094 C 59.579654 2.923549 63.413723 2.923549 66.998132 3.996094 L 73.007812 6.00279 C 75.272588 6.763951 77.733526 6.763951 79.998302 6.00279 L 84.991508 2.006696 C 88.005265 1.003348 91.001189 0 93.997113 1.003348 C 96.993037 1.003348 99.008152 2.006696 101.005435 3.996094 C 103.002717 6.00279 103.002717 9.998884 102.004076 16.001674 L 102.004076 18.008371 L 104.001359 23.007812 L 105 28.007254 L 104.001359 33.006696 L 101.005435 37.00279 L 95.994395 40.998884 L 78.99966 49.008371 L 75.005095 50.997768 L 74.006454 50.997768 L 58.991168 58.003906 L 54.996603 59.993304 L 52.000679 59.993304 L 51.002038 60.996652 L 46.008832 60.996652 L 39.00051 59.007254 C 28.60394 54.128906 18.278702 49.129464 8.006963 43.991629 L 8.006963 43.00558 L 2.995924 39.995536 L 0 33.992746 C 0 31.294085 0.338825 28.612723 0.998641 26.000558 C 1.997283 23.993862 2.995924 22.004464 5.991848 21.001116 Z M 5.991848 21.001116 " transform="matrix(0.219048,0,0,0.225806,0,0)"></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(25.882354%,9.411765%,1.568628%);fill-opacity:1;" d="M 1.3125 8.128906 L 1.09375 7.675781 L 1.3125 6.097656 L 1.753906 5.644531 L 9.855469 2.933594 L 9.636719 2.257812 C 9.710938 1.882812 9.785156 1.503906 9.855469 1.128906 L 10.078125 1.128906 L 10.515625 1.355469 L 10.734375 1.355469 L 12.265625 2.03125 C 12.914062 1.859375 13.589844 1.859375 14.238281 2.03125 C 14.910156 2.207031 15.570312 2.433594 16.210938 2.710938 L 16.648438 2.710938 L 17.960938 2.484375 L 18.398438 2.03125 L 19.058594 1.582031 L 20.371094 1.355469 L 21.246094 1.804688 L 21.246094 3.839844 L 21.027344 4.066406 L 20.371094 4.515625 L 20.589844 4.515625 L 21.464844 4.96875 L 21.90625 5.417969 L 21.90625 6.324219 L 21.6875 7 L 21.464844 7.675781 L 20.589844 8.128906 C 19.933594 8.582031 18.839844 9.257812 16.867188 9.933594 L 12.484375 11.96875 L 11.828125 12.417969 C 11.617188 12.515625 11.394531 12.589844 11.171875 12.644531 L 10.296875 12.644531 L 8.980469 12.195312 C 6.703125 11.09375 4.441406 9.964844 2.191406 8.804688 Z M 1.3125 8.128906 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(65.490198%,51.372552%,26.274511%);fill-opacity:1;" d="M 6.351562 5.195312 C 8.921875 4.820312 11.476562 4.371094 14.019531 3.839844 L 15.332031 4.289062 L 16.429688 4.515625 C 17.304688 4.515625 17.742188 4.289062 17.960938 4.066406 L 18.179688 3.613281 L 18.617188 3.160156 L 19.933594 2.484375 L 21.027344 2.03125 L 21.027344 3.839844 L 20.808594 4.066406 C 20.371094 4.289062 19.933594 4.515625 19.277344 4.289062 L 17.960938 4.742188 L 19.496094 4.96875 L 21.464844 5.644531 L 21.464844 7.226562 L 21.246094 7.453125 L 20.371094 8.128906 C 17.839844 9.550781 15.203125 10.757812 12.484375 11.742188 L 11.171875 12.417969 C 10.515625 12.417969 9.636719 12.417969 8.980469 11.96875 C 6.324219 10.59375 3.695312 9.164062 1.09375 7.675781 L 1.3125 7 L 1.535156 6.324219 Z M 6.351562 5.195312 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(56.078434%,44.313726%,22.352941%);fill-opacity:1;" d="M 4.382812 7.902344 L 3.503906 7.902344 L 3.285156 9.257812 L 3.066406 9.03125 L 3.285156 7.675781 L 2.847656 7.226562 L 2.628906 7.453125 L 2.410156 8.804688 L 2.191406 8.582031 L 1.972656 8.582031 L 2.410156 7.226562 L 1.972656 7 L 1.753906 8.355469 L 1.3125 8.128906 L 1.535156 6.546875 L 6.351562 6.324219 L 10.078125 8.128906 L 10.953125 10.839844 L 11.171875 12.417969 L 10.296875 12.417969 L 10.515625 10.839844 L 9.417969 10.613281 L 9.199219 12.195312 L 8.542969 11.96875 L 8.761719 10.386719 L 8.105469 10.160156 L 7.886719 11.515625 L 7.449219 11.289062 L 7.449219 9.710938 L 7.230469 9.03125 L 6.570312 9.257812 L 6.132812 10.613281 L 5.476562 10.386719 L 5.914062 9.03125 L 4.820312 8.128906 L 4.601562 8.355469 L 4.382812 9.710938 L 4.160156 9.484375 Z M 4.382812 7.902344 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(52.941179%,41.176471%,18.82353%);fill-opacity:1;" d="M 19.933594 2.484375 L 19.933594 2.710938 C 18.839844 3.160156 18.617188 3.839844 19.496094 4.289062 L 18.617188 4.289062 L 17.960938 4.742188 L 19.496094 4.96875 L 21.464844 5.644531 L 21.464844 7.226562 L 21.246094 7.453125 L 20.371094 8.128906 C 17.839844 9.550781 15.203125 10.757812 12.484375 11.742188 L 11.828125 12.195312 L 10.515625 12.417969 L 10.734375 12.195312 L 10.734375 9.710938 L 10.515625 8.804688 C 13.609375 6.628906 16.75 4.519531 19.933594 2.484375 Z M 19.933594 2.484375 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(47.450981%,36.470589%,16.862746%);fill-opacity:1;" d="M 21.027344 7.453125 C 21.464844 7 21.464844 6.546875 21.464844 5.644531 L 21.464844 7.226562 L 21.246094 7.453125 L 20.371094 8.128906 C 17.839844 9.550781 15.203125 10.757812 12.484375 11.742188 L 11.828125 12.195312 L 10.515625 12.417969 L 10.734375 12.195312 L 11.171875 12.195312 L 12.046875 11.96875 C 15.042969 10.46875 18.035156 8.960938 21.027344 7.453125 Z M 21.027344 7.453125 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(87.450981%,71.764708%,40.784314%);fill-opacity:1;" d="M 1.753906 6.097656 C 5.445312 4.722656 9.167969 3.441406 12.921875 2.257812 L 14.238281 2.484375 L 15.550781 2.933594 L 16.648438 3.160156 C 17.523438 3.160156 17.960938 2.933594 18.179688 2.710938 L 18.617188 2.257812 L 19.058594 2.03125 C 19.714844 1.582031 20.152344 1.582031 20.808594 1.804688 C 21.246094 2.03125 21.246094 2.484375 20.808594 2.710938 L 19.496094 3.160156 L 18.179688 3.613281 L 19.058594 4.289062 C 19.855469 4.75 20.660156 5.203125 21.464844 5.644531 C 21.6875 5.871094 21.246094 6.324219 20.589844 6.773438 C 17.578125 8.257812 14.511719 9.613281 11.390625 10.839844 C 10.734375 11.066406 9.855469 10.839844 8.980469 10.613281 C 6.472656 9.3125 3.988281 7.957031 1.535156 6.546875 L 1.535156 6.097656 Z M 1.753906 6.097656 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(79.215688%,64.313728%,33.333334%);fill-opacity:1;" d="M 2.628906 7.226562 L 2.410156 7.226562 L 4.820312 6.097656 L 6.351562 4.742188 L 8.105469 3.839844 C 9.554688 3.546875 11.015625 3.320312 12.484375 3.160156 C 12.921875 3.160156 13.582031 2.933594 14.019531 2.484375 L 14.457031 2.484375 C 12.945312 3.640625 11.078125 4.203125 9.199219 4.066406 C 8.105469 4.066406 7.230469 4.289062 6.570312 4.742188 L 5.039062 6.097656 C 4.601562 6.546875 3.722656 7 2.628906 7.226562 Z M 2.628906 7.226562 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(79.215688%,64.313728%,33.333334%);fill-opacity:1;" d="M 5.257812 7 C 4.601562 7 3.941406 7.226562 3.503906 7.675781 L 3.285156 7.675781 C 4.5625 6.878906 5.976562 6.34375 7.449219 6.097656 L 10.296875 5.417969 L 11.609375 4.742188 L 12.484375 4.066406 L 14.675781 2.484375 L 14.894531 2.710938 L 12.921875 4.289062 L 10.953125 5.644531 C 9.855469 6.324219 7.886719 6.773438 5.257812 7 Z M 1.972656 6.324219 L 3.066406 5.871094 L 4.160156 5.195312 L 6.132812 4.515625 L 5.476562 4.96875 L 3.722656 6.097656 L 2.191406 6.773438 L 1.972656 6.773438 L 1.535156 6.546875 Z M 9.855469 3.160156 L 11.609375 2.710938 L 13.363281 2.257812 L 13.582031 2.257812 C 13.144531 2.710938 12.484375 2.933594 11.828125 2.933594 Z M 18.617188 7.675781 C 19.277344 6.546875 20.152344 5.871094 21.246094 5.417969 L 21.464844 5.644531 C 20.808594 5.871094 20.152344 6.324219 19.714844 7 Z M 9.199219 7.902344 C 10.078125 7.902344 10.953125 7.675781 12.046875 7 L 14.019531 5.417969 L 15.550781 4.066406 C 16.210938 3.613281 16.648438 3.160156 17.304688 3.160156 L 18.179688 2.710938 L 20.589844 1.804688 L 20.808594 1.804688 L 20.589844 2.03125 L 18.398438 2.933594 L 16.210938 3.839844 L 14.457031 5.417969 C 13.800781 6.324219 13.144531 6.773438 12.703125 7 C 12.265625 7.453125 11.390625 7.902344 10.078125 8.128906 L 7.886719 8.582031 L 6.570312 9.257812 L 5.914062 8.804688 L 7.230469 8.355469 Z M 16.867188 6.097656 C 17.304688 5.195312 17.960938 4.742188 18.839844 4.289062 L 19.496094 4.515625 L 17.742188 5.871094 L 15.992188 7.902344 C 15.113281 8.582031 14.457031 9.03125 13.582031 9.257812 L 10.953125 9.710938 L 9.417969 10.613281 L 8.761719 10.386719 L 10.515625 9.484375 L 12.703125 9.03125 C 14.382812 8.582031 15.851562 7.542969 16.867188 6.097656 Z M 16.867188 6.097656 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(79.215688%,64.313728%,33.333334%);fill-opacity:1;" d="M 6.789062 7.675781 C 5.914062 7.675781 5.257812 7.902344 4.601562 8.355469 L 4.160156 8.128906 C 4.820312 7.675781 5.914062 7.226562 7.230469 7.226562 L 10.953125 6.097656 C 12.046875 5.644531 12.921875 4.96875 13.582031 4.289062 L 15.332031 2.710938 L 15.992188 2.933594 L 14.019531 4.515625 L 12.265625 5.871094 L 9.636719 7.226562 Z M 20.152344 4.742188 L 20.589844 4.96875 L 18.839844 6.324219 C 18.179688 6.773438 17.742188 7.453125 17.304688 8.355469 C 14.960938 9.164062 12.625 9.992188 10.296875 10.839844 L 12.703125 9.933594 L 14.894531 9.03125 C 15.992188 8.582031 17.304688 7.453125 18.617188 5.644531 Z M 13.144531 7.453125 C 14.671875 6.238281 16.203125 5.035156 17.742188 3.839844 C 17.960938 3.386719 19.058594 2.933594 21.027344 2.03125 L 21.027344 2.484375 C 20.402344 2.570312 19.804688 2.800781 19.277344 3.160156 L 18.179688 3.613281 L 18.398438 3.839844 L 15.769531 6.324219 L 13.582031 8.128906 L 10.515625 8.804688 C 9.417969 9.03125 8.761719 9.484375 8.105469 9.933594 L 7.449219 9.710938 C 9.289062 8.8125 11.191406 8.058594 13.144531 7.453125 Z M 6.570312 5.871094 L 6.570312 5.644531 L 6.789062 5.195312 L 7.230469 4.515625 L 8.761719 4.289062 L 10.515625 4.289062 C 10.734375 4.515625 10.734375 4.742188 10.296875 4.96875 L 8.761719 5.644531 L 7.449219 5.871094 L 7.449219 5.195312 L 8.324219 4.742188 L 9.199219 4.515625 L 9.417969 4.742188 L 8.761719 5.195312 L 8.980469 4.96875 L 8.324219 4.96875 L 8.105469 5.195312 C 7.886719 5.417969 8.105469 5.417969 8.324219 5.417969 L 9.199219 5.195312 L 9.855469 4.742188 L 9.855469 4.515625 L 8.761719 4.515625 L 7.449219 4.96875 L 6.789062 5.417969 Z M 6.570312 5.871094 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(41.960785%,25.098041%,14.117648%);fill-opacity:1;" d="M 9.855469 3.160156 L 11.171875 2.710938 L 12.265625 3.839844 L 14.238281 5.417969 L 15.550781 7 L 16.210938 8.582031 L 15.992188 9.484375 L 15.332031 9.710938 L 14.894531 9.710938 L 14.894531 9.257812 L 15.113281 9.03125 L 15.332031 8.582031 L 14.675781 7.675781 L 14.457031 7.453125 L 14.457031 7.226562 L 14.238281 6.773438 L 14.019531 6.773438 C 13.304688 7.003906 12.570312 7.15625 11.828125 7.226562 L 11.171875 7.226562 L 10.734375 6.097656 L 10.078125 4.289062 Z M 9.855469 3.160156 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(47.843137%,47.843137%,47.843137%);fill-opacity:1;" d="M 11.171875 7 L 11.390625 5.871094 L 12.265625 4.96875 L 13.582031 4.96875 L 14.894531 5.195312 L 15.113281 5.871094 L 14.894531 6.097656 L 12.921875 6.773438 L 11.609375 7 Z M 11.171875 7 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(59.215689%,59.215689%,59.215689%);fill-opacity:1;" d="M 12.265625 6.097656 L 12.046875 6.546875 L 12.265625 7 L 11.171875 7 L 11.390625 6.324219 Z M 12.265625 6.097656 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(92.54902%,92.54902%,92.54902%);fill-opacity:1;" d="M 9.855469 1.582031 L 10.734375 1.804688 L 12.921875 2.933594 C 13.75 3.667969 14.488281 4.5 15.113281 5.417969 L 14.894531 5.417969 L 14.019531 4.289062 L 12.484375 2.710938 L 10.734375 1.804688 Z M 9.855469 1.582031 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(78.039217%,78.039217%,78.039217%);fill-opacity:1;" d="M 9.855469 1.582031 L 10.078125 1.582031 L 10.515625 1.804688 C 10.609375 3.429688 11.140625 4.992188 12.046875 6.324219 L 11.171875 7 L 10.953125 6.546875 C 10.421875 5.246094 10.054688 3.882812 9.855469 2.484375 Z M 9.855469 1.582031 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(89.411765%,89.411765%,89.411765%);fill-opacity:1;" d="M 10.078125 3.386719 L 10.078125 2.933594 L 10.734375 2.933594 L 10.734375 3.160156 Z M 9.855469 2.03125 L 10.515625 2.03125 L 10.515625 2.710938 L 9.855469 2.710938 Z M 11.828125 6.097656 L 11.171875 6.773438 L 10.953125 6.324219 L 11.609375 5.871094 Z M 10.296875 4.515625 L 10.078125 3.839844 L 10.734375 3.613281 L 10.953125 4.066406 Z M 11.390625 5.195312 L 10.734375 5.644531 L 10.515625 4.96875 L 10.953125 4.515625 Z M 11.390625 5.195312 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(56.470591%,56.470591%,56.470591%);fill-opacity:1;" d="M 14.238281 6.546875 L 14.019531 6.324219 L 14.019531 5.871094 L 14.675781 5.871094 L 15.113281 6.097656 L 15.113281 6.324219 L 14.894531 6.546875 Z M 14.238281 6.546875 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(70.19608%,70.19608%,70.19608%);fill-opacity:1;" d="M 14.019531 5.871094 L 14.238281 5.644531 L 14.894531 5.644531 L 15.113281 5.871094 L 15.113281 6.097656 L 14.238281 6.097656 Z M 14.019531 5.871094 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(55.686277%,35.686275%,17.254902%);fill-opacity:1;" d="M 14.238281 6.773438 L 14.238281 6.097656 L 14.894531 6.097656 L 15.550781 6.773438 L 16.429688 8.128906 L 16.429688 8.582031 L 15.769531 9.484375 C 15.113281 9.710938 14.675781 9.710938 14.894531 9.257812 L 15.113281 8.582031 L 15.113281 8.128906 L 14.894531 7.675781 L 14.675781 7.453125 L 14.457031 6.773438 Z M 14.238281 6.773438 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(43.137255%,27.058825%,13.725491%);fill-opacity:1;" d="M 15.769531 8.355469 L 16.210938 7.675781 L 16.429688 8.128906 L 16.429688 8.582031 C 16.429688 9.03125 16.210938 9.484375 15.769531 9.484375 L 14.894531 9.484375 L 14.894531 9.03125 L 15.113281 8.582031 L 15.332031 8.355469 Z M 15.769531 8.355469 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(64.313728%,44.705883%,26.666668%);fill-opacity:1;" d="M 14.675781 6.324219 L 14.457031 6.097656 L 14.457031 5.871094 L 15.113281 5.871094 L 15.769531 6.097656 L 16.429688 7.226562 L 16.429688 8.582031 C 15.992188 8.804688 15.550781 9.03125 15.113281 8.804688 L 15.113281 8.355469 L 15.550781 7.902344 L 15.332031 7.453125 L 14.894531 7.226562 L 14.675781 6.773438 Z M 14.675781 6.324219 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 15.113281 8.128906 L 15.550781 7.902344 L 15.332031 8.355469 L 15.332031 8.804688 L 15.113281 8.804688 Z M 14.457031 5.871094 L 14.894531 6.546875 L 15.332031 7.453125 L 15.113281 7.453125 L 14.894531 6.773438 L 14.894531 6.546875 L 14.675781 6.546875 L 14.238281 6.097656 Z M 16.429688 7.675781 L 16.429688 7.453125 C 16.648438 7.902344 16.648438 8.355469 16.210938 8.582031 Z M 16.429688 7.675781 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 14.894531 6.097656 L 15.332031 6.546875 L 15.550781 6.773438 L 15.550781 7 L 15.769531 7.453125 L 15.769531 8.128906 L 15.550781 8.804688 L 15.550781 7 L 14.675781 5.871094 Z M 14.894531 6.097656 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 15.550781 6.324219 L 15.992188 6.546875 L 16.210938 7.226562 L 16.210938 8.128906 L 15.992188 8.804688 L 15.992188 6.546875 C 15.695312 6.324219 15.402344 6.101562 15.113281 5.871094 L 15.332031 5.871094 Z M 15.550781 6.324219 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 15.113281 5.871094 L 15.332031 6.324219 L 15.550781 6.773438 L 15.769531 7.226562 L 15.992188 7.453125 L 15.992188 8.128906 L 15.769531 8.804688 L 15.769531 7 L 15.550781 6.773438 L 15.332031 6.324219 L 14.894531 5.871094 Z M 15.332031 5.871094 L 15.769531 6.324219 L 15.992188 6.546875 L 15.550781 6.324219 Z M 15.332031 5.871094 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(43.137255%,27.058825%,13.725491%);fill-opacity:1;" d="M 16.210938 8.355469 C 15.992188 8.582031 15.769531 8.804688 15.550781 8.582031 L 15.332031 8.355469 L 15.992188 8.128906 Z M 16.210938 8.355469 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(69.411767%,69.411767%,69.411767%);fill-opacity:1;" d="M 16.210938 8.355469 L 15.992188 8.582031 L 15.332031 8.582031 L 15.769531 8.355469 Z M 16.210938 8.355469 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(49.019608%,49.019608%,49.019608%);fill-opacity:1;" d="M 16.210938 8.355469 L 15.992188 8.582031 L 15.332031 8.582031 L 15.769531 8.582031 L 15.992188 8.355469 Z M 16.210938 8.355469 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 15.992188 6.773438 L 15.769531 6.773438 L 15.992188 7 L 15.992188 6.773438 L 15.992188 7 L 15.769531 7 L 15.769531 6.773438 Z M 15.992188 6.773438 "></path></g></svg></span><label><input type="text" name="time-prep" value="00:02:00" class="input input-xs max-w-24 html-duration-picker"></label></div>"#,
                r#"<table class="table table-zebra table-xs"><thead><tr><th>Nutrition (per 100g)</th><th>Amount</th></tr></thead><tbody><tr><td>Calories</td><td><label><input type="text" name="calories" autocomplete="off" placeholder="368kcal" class="input input-xs max-w-24"></label></td></tr><tr><td>Total carbs</td><td><label><input type="text" name="total-carbohydrates" autocomplete="off" placeholder="35g" class="input input-xs max-w-24"></label></td></tr><tr><td>Sugars</td><td><label><input type="text" name="sugars" autocomplete="off" placeholder="3g" class="input input-xs max-w-24"></label></td></tr><tr><td>Protein</td><td><label><input type="text" name="protein" autocomplete="off" placeholder="21g" class="input input-xs max-w-24"></label></td></tr><tr><td>Total fat</td><td><label><input type="text" name="total-fat" autocomplete="off" placeholder="15g" class="input input-xs max-w-24"></label></td></tr><tr><td>Saturated fat</td><td><label><input type="text" name="saturated-fat" autocomplete="off" placeholder="1.8g" class="input input-xs max-w-24"></label></td></tr><tr><td>Unsaturated fat</td><td><label><input type="text" name="unsaturated-fat" autocomplete="off" placeholder="1.8g" class="input input-xs max-w-24"></label></td></tr><tr><td>Trans fat</td><td><label><input type="text" name="trans-fat" autocomplete="off" placeholder="1.8g" class="input input-xs max-w-24"></label></td></tr><tr><td>Cholesterol</td><td><label><input type="text" name="cholesterol" autocomplete="off" placeholder="1.1mg" class="input input-xs max-w-24"></label></td></tr><tr><td>Sodium</td><td><label><input type="text" name="sodium" autocomplete="off" placeholder="100mg" class="input input-xs max-w-24"></label></td></tr><tr><td>Fiber</td><td><label><input type="text" name="fiber" autocomplete="off" placeholder="8g" class="input input-xs max-w-24"></label></td></tr></tbody></table>"#,
                r#"<h2 class="font-semibold text-center pb-2"><span class="underline">Tools</span></h2>"#,
                r#"<ol id="tools-list" class="pl-4"><li class="pb-2"><div class="grid grid-flow-col items-center"><label class="flex gap-1"><div class="inline-block h-4 cursor-move handle mt-1"><svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"></path></svg></div><input type="text" name="tool" placeholder="1 frying pan" class="input input-bordered input-sm w-full" value="1 wok" _="on keydown if event.key is 'Enter' halt the event then call addItem(event)"></label><div class="ml-2 flex gap-2"><button type="button" class="btn btn-square btn-sm btn-outline btn-success" title="Shortcut: Enter" onclick="addItem(event)">+</button><button type="button" class="delete-button btn btn-square btn-sm btn-outline btn-error""#,
                r#"<input type="text" name="tool" placeholder="1 frying pan" class="input input-bordered input-sm w-full" value="1 frying pan" _="on keydown if event.key is 'Enter' halt the event then call addItem(event)">"#,
                r#"<h2 class="font-semibold text-center pb-2"><span class="underline">Ingredients</span><sup class="text-red-600">*</sup></h2>"#,
                r#"<ol id="ingredients-list" class="pl-4"><li class="pb-2"><div class="grid grid-flow-col items-center"><label class="flex gap-1"><div class="inline-block h-4 cursor-move handle mt-1"><svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"></path></svg></div><input required type="text" name="ingredient" value="1 cup blue spinach" placeholder="1 cup of chopped onions" class="input input-bordered input-sm w-full" _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))"></label><div class="ml-2 flex gap-2"><button type="button" class="btn btn-square btn-sm btn-outline btn-success" title="Shortcut: Enter" onclick="addItem(event)">+</button><button type="button" class="delete-button btn btn-square btn-sm btn-outline btn-error""#,
                r#"<input required type="text" name="ingredient" value="1/2 tbsp cinnamon" placeholder="1 cup of chopped onions" class="input input-bordered input-sm w-full" _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))">"#,
                r#"<input required type="text" name="ingredient" value="4 pounds top quality chicken filet" placeholder="1 cup of chopped onions" class="input input-bordered input-sm w-full" _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))">"#,
                r#"<input required type="text" name="ingredient" value="1/8 cup lemon juice" placeholder="1 cup of chopped onions" class="input input-bordered input-sm w-full" _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))">"#,
                r#"<h2 class="font-semibold text-center pb-2"><span class="underline">Instructions</span><sup class="text-red-600">*</sup></h2>"#,
                r##"<ol id="instructions-list" class="grid list-decimal"><li class="pt-2 md:pl-0"><div class="flex"><label class="w-11/12"><textarea required name="instruction" rows="4" class="textarea textarea-bordered w-full" placeholder="Mix all ingredients together" _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))">Mix all these ingredients</textarea></label><div class="grid gap-2 ml-2"><button type="button" class="btn btn-square btn-sm btn-outline btn-success" title="Shortcut: CTRL + Enter" onclick="addItem(event)">+</button><button type="button" class="delete-button btn btn-square btn-sm btn-outline btn-error"##,
                r#"<textarea required name="instruction" rows="4" class="textarea textarea-bordered w-full" placeholder="Mix all ingredients together" _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))">Turn the oven at 300 F</textarea>"#,
                r#"<textarea required name="instruction" rows="4" class="textarea textarea-bordered w-full" placeholder="Mix all ingredients together" _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))">Soak the chicken in the lemon juice</textarea>"#,
                r#"<textarea required name="instruction" rows="4" class="textarea textarea-bordered w-full" placeholder="Mix all ingredients together" _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))">Mix all these ingredients</textarea>"#,
                r#"<button class="btn btn-primary btn-block btn-sm">Submit</button>"#,
            ];
            for want in expected {
                res.assert_text_contains(want);
            }
        }
    }

    mod tests_edit {
        use super::*;
        use models::recipe::test_utils::a_complete_recipe_for_create;

        use axum::http::HeaderValue;
        use axum_test::TestResponse;
        use chrono::Duration;
        use uuid::Uuid;

        use models::recipe::{
            Nutrition, NutritionForCreate, RecipeForCreate, Times, TimesForCreate, ToolForCreate,
            ToolRecipe, VideoForCreate,
        };
        use models::{Recipe, RecipeDetails};
        use recipe_schema::Sections;
        use testing::utils::{assert_ws_message, build_server_ws, create_app_state};

        fn base_uri(recipe_id: i64) -> String {
            format!("/recipes/{recipe_id}/edit")
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            let _ = assert_must_be_logged_in(Method::GET, &base_uri(1)).await;
            assert_must_be_logged_in(Method::POST, &base_uri(1)).await
        }

        #[tokio::test]
        async fn test_get_recipe_not_exist_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_not_found();
            assert_ws_message(&mut ws_server,r#"{"showMessageHtmx":{"type":"toast","message":"Recipe not found.","status":"alert-error","title":"Operation Failed"}}"# ).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_get_recipe_exists_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let _ = Recipe::create(&state.mm, 1, &a_complete_recipe_for_create()).await?;

            let res = server.get(&base_uri(1)).await;

            assert_recipe_form(res);
            Ok(())
        }

        #[tokio::test]
        async fn test_get_recipe_exists_htmx_request_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let mut server = build_server_logged_in(config.clone()).await?;
            server.add_header(
                axum_htmx::headers::HX_REQUEST,
                HeaderValue::from_static("true"),
            );
            let state = create_app_state(config).await;
            let _ = Recipe::create(&state.mm, 1, &a_complete_recipe_for_create()).await?;

            let res = server.get(&base_uri(1)).await;

            assert_recipe_form(res);
            Ok(())
        }

        #[tokio::test]
        async fn test_put_missing_required_title_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;
            let recipe = RecipeForCreate {
                ingredients: Sections::from([("".into(), vec!["1 apple".to_string()])]),
                instructions: Sections::from([("".into(), vec!["Mix the apples".to_string()])]),
                ..Default::default()
            };

            let res = server
                .put(&base_uri(1))
                .multipart(create_form(&recipe))
                .await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_put_missing_required_ingredients_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;
            let recipe = RecipeForCreate {
                name: "Best Chinese Kale".to_string(),
                instructions: Sections::from([("".into(), vec!["Mix the apples".to_string()])]),
                ..Default::default()
            };

            let res = server
                .put(&base_uri(1))
                .multipart(create_form(&recipe))
                .await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_put_missing_required_instructions_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;
            let recipe = RecipeForCreate {
                name: "Best Chinese Kale".to_string(),
                ingredients: Sections::from([("".into(), vec!["8 apples".to_string()])]),
                ..Default::default()
            };

            let res = server
                .put(&base_uri(1))
                .multipart(create_form(&recipe))
                .await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_put_update_image_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config.clone()).await;
            let mut recipe = a_complete_recipe_for_create();
            Recipe::create(&state.mm, 1, &recipe).await?;
            recipe.images = Vec::new();
            recipe.videos = Vec::new();

            let res = server
                .put(&base_uri(1))
                .multipart(create_form(&recipe))
                .await;

            res.assert_status_see_other();
            let state = create_app_state(config.clone()).await;
            let got = Recipe::get(&state.mm, 1, 1).await?;
            let expected = recipe_for_create_to_details(recipe, &got);
            pretty_assertions::assert_eq!(got, expected);
            assert!(got.recipe.image.is_none());
            pretty_assertions::assert_eq!(got.additional_images.len(), 0);
            Ok(())
        }

        #[tokio::test]
        async fn test_put_missing_fields_defaults_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config.clone()).await;
            let mut recipe = a_complete_recipe_for_create();
            Recipe::create(&state.mm, 1, &a_complete_recipe_for_create()).await?;
            recipe.name = "Maple Syrup Korean Chicken".into();
            recipe.ingredients = Sections::from([("".into(), vec!["4 apples".to_string()])]);
            recipe.instructions = Sections::from([("".into(), vec!["Drink juice".to_string()])]);

            let res = server
                .put(&base_uri(1))
                .multipart(create_form(&recipe))
                .await;

            res.assert_status_see_other();
            let state = create_app_state(config.clone()).await;
            let got = Recipe::get(&state.mm, 1, 1).await?;
            let expected = recipe_for_create_to_details(recipe, &got);
            pretty_assertions::assert_eq!(got, expected);
            assert!(got.recipe.image.is_some());
            pretty_assertions::assert_eq!(
                got.additional_images.len(),
                expected.additional_images.len()
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_put_can_only_be_one_category_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config.clone()).await;
            let mut recipe = a_complete_recipe_for_create();
            Recipe::create(&state.mm, 1, &recipe).await?;
            recipe.category = Some("breakfast,dinner".into());

            let res = server
                .put(&base_uri(1))
                .multipart(create_form(&recipe))
                .await;

            res.assert_status_see_other();
            let state = create_app_state(config.clone()).await;
            let got = Recipe::get(&state.mm, 1, 1).await?;
            pretty_assertions::assert_eq!(got.category, "breakfast");
            Ok(())
        }

        #[tokio::test]
        async fn test_put_subcategories_are_possible() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config.clone()).await;
            let mut recipe = RecipeForCreate {
                name: "Best Chinese Kale".to_string(),
                instructions: Sections::from([("".into(), vec!["Mix the apples".to_string()])]),
                ingredients: Sections::from([("".into(), vec!["8 apples".to_string()])]),
                ..Default::default()
            };
            Recipe::create(&state.mm, 1, &recipe).await?;
            recipe.category = Some("drinks:vodka".into());

            let res = server
                .put(&base_uri(1))
                .multipart(create_form(&recipe))
                .await;

            res.assert_status_see_other();
            let state = create_app_state(config.clone()).await;
            let got = Recipe::get(&state.mm, 1, 1).await?;
            pretty_assertions::assert_eq!(got.category, "drinks:vodka");
            Ok(())
        }

        #[tokio::test]
        async fn test_put_submit_recipe_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config.clone()).await;
            let mut recipe = a_complete_recipe_for_create();
            Recipe::create(&state.mm, 1, &recipe).await?;
            recipe = RecipeForCreate {
                name: "Crepes".into(),
                description: Some("Trust me. They're delicious.".into()),
                images: vec![Uuid::new_v4(), Uuid::new_v4()],
                measurement_system_id: 2,
                yield_: Some(12),
                source: Some("My father's maple syrup recipes cookbook".into()),
                videos: vec![VideoForCreate {
                    video: Uuid::new_v4(),
                    duration: Some(Duration::minutes(30)),
                    content_url: Some("https://www.youtube.com/watch?v=2".into()),
                    embed_url: Some("https://www.youtube.com/embed/embeded".into()),
                }],
                category: Some("breakfast".into()),
                instructions: Sections::from([
                    (
                        "".into(),
                        vec!["Mix the blueberries".into(), "Mix the strawberries".into()],
                    ),
                    (
                        "Finalize".into(),
                        vec!["Whisk the fruits until smooth".into()],
                    ),
                ]),
                keywords: vec!["blueberries".into(), "vegan".into()],
                nutrition: Some(NutritionForCreate {
                    calories_kcal: Some(100),
                    total_carbohydrates: Some(20),
                    sugars_g: Some(30),
                    protein_g: Some(40),
                    total_fat_g: Some(50),
                    saturated_fat_g: Some(60),
                    unsaturated_fat_g: Some(70),
                    cholesterol_mg: Some(80),
                    sodium_mg: Some(90),
                    fiber_g: Some(100),
                    trans_fat_g: Some(110),
                    serving_size: Some("120g".into()),
                }),
                times: Some(TimesForCreate {
                    prep_seconds: 2000,
                    cook_seconds: 800,
                }),
                ingredients: Sections::from([(
                    "".into(),
                    vec!["8 lbs blueberries".into(), "12 lbs strawberries".into()],
                )]),
                cuisine: Some("quebec".into()),
                tools: vec![ToolForCreate {
                    name: "medium bowl".into(),
                    quantity: 1,
                }],
            };

            let res = server
                .put(&base_uri(1))
                .multipart(create_form(&recipe))
                .await;

            res.assert_status_see_other();
            let state = create_app_state(config.clone()).await;
            let got = Recipe::get(&state.mm, 1, 1).await?;
            let mut expected = recipe_for_create_to_details(recipe, &got);
            expected.ingredients = got.ingredients.clone();
            expected.instructions = got.instructions.clone();
            pretty_assertions::assert_eq!(got, expected);
            Ok(())
        }

        #[tokio::test]
        async fn test_put_duplicates_are_removed_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config.clone()).await;
            let mut recipe = a_complete_recipe_for_create();
            Recipe::create(&state.mm, 1, &recipe).await?;
            recipe.instructions = Sections::from([(
                "".into(),
                vec![
                    "Mix the apples".to_string(),
                    "Eat".to_string(),
                    "Mix the apples".to_string(),
                ],
            )]);
            recipe.ingredients = Sections::from([(
                "".into(),
                vec![
                    "8 apples".to_string(),
                    "4 oranges".to_string(),
                    "8 apples".to_string(),
                ],
            )]);
            recipe.keywords = vec!["drinks".into(), "vodka".into(), "drinks".into()];
            recipe.tools = vec![
                ToolForCreate {
                    quantity: 1,
                    name: "1 wok".into(),
                },
                ToolForCreate {
                    quantity: 1,
                    name: "1 wok".into(),
                },
            ];

            let res = server
                .put(&base_uri(1))
                .multipart(create_form(&recipe))
                .await;

            res.assert_status_see_other();
            let state = create_app_state(config.clone()).await;
            let got = Recipe::get(&state.mm, 1, 1).await?;
            pretty_assertions::assert_eq!(
                got.keywords,
                vec!["drinks".to_string(), "vodka".to_string()]
            );
            pretty_assertions::assert_eq!(
                got.ingredients,
                Sections::from([(
                    "".into(),
                    vec!["8 apples".to_string(), "4 oranges".to_string()]
                )])
            );
            pretty_assertions::assert_eq!(
                got.instructions,
                Sections::from([(
                    "".into(),
                    vec!["Mix the apples".to_string(), "Eat".to_string()]
                )])
            );
            pretty_assertions::assert_eq!(
                got.tools,
                vec![ToolRecipe {
                    name: "wok".into(),
                    tool_order: 1,
                    quantity: 1,
                }]
            );
            Ok(())
        }

        fn assert_recipe_form(res: TestResponse) {
            let expected = [
                r#"<title hx-swap-oob="true">Edit Best Chinese Kale | Recipya</title>"#,
                r##"<form class="card-body" style="padding: 0" enctype="multipart/form-data" hx-put="/recipes/1/edit" hx-indicator="#fullscreen-loader">"##,
                r#"<input required type="text" name="title" placeholder="Title of the recipe*" autocomplete="off" class="input w-full text-center rounded-t-lg rounded-b-none bg-base-200" value="Best Chinese Kale">"#,
                r#"<img src="" alt="Image #1 of the recipe" class="object-cover mb-2 w-full max-h-[39rem]"><span class="grid gap-1 max-w-sm" style="margin: auto auto 0.25rem;"><div class="mr-1"><input type="file" accept="image/*,video/*" name="media" class="file-input file-input-sm file-input-bordered w-full max-w-sm" value="""#,
                r#"<img src="" alt="Image #2 of the recipe" class="object-cover mb-2 w-full max-h-[39rem]"><span class="grid gap-1 max-w-sm" style="margin: auto auto 0.25rem;"><div class="mr-1"><input type="file" accept="image/*,video/*" name="media" class="file-input file-input-sm file-input-bordered w-full max-w-sm" value="""#,
                r#"<img src="" alt="" class="mb-2"><span class="grid gap-1 max-w-sm" style="margin: auto auto 0.25rem;"><div class="mr-1 hidden"><input type="file" accept="image/*,video/*" name="media" class="file-input file-input-sm file-input-bordered w-full max-w-sm" value="""#,
                r#"<input id="servings" type="number" min="1" name="yield" value="4" class="input input-sm w-11/12">"#,
                r#"<input id="category" type="text" list="categories" name="category" class="input input-sm w-11/12" placeholder="Breakfast" autocomplete="off" value="dinner"><datalist id="categories"><option>uncategorized</option><option>appetizers</option><option>bread</option><option>breakfasts</option><option>condiments</option><option>dessert</option><option>lunch</option><option>main dish</option><option>salad</option><option>side dish</option><option>snacks</option><option>soups</option><option>stews</option><option>dinner</option></datalist>"#,
                r#"<textarea name="description" placeholder="This Thai curry chicken will make you drool." class="textarea w-full h-full resize-none">This is the most delicious recipe!</textarea>"#,
                r#"<div class="flex justify-self-center items-center gap-1 cursor-default" title="Cooking time"><span data-tip="Cook time" class="tooltip tooltip-left"><svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="24px" height="23px" viewBox="0 0 24 23" version="1.1"><g id="surface1"><path style=" stroke:none;fill-rule:nonzero;fill:rgb(62.745098%,64.705882%,65.882353%);fill-opacity:1;" d="M 4.636719 10.984375 L 4.636719 20.417969 C 4.636719 21.007812 5.078125 21.527344 5.667969 21.527344 L 18.257812 21.527344 C 18.847656 21.527344 19.363281 21.007812 19.363281 20.417969 L 19.363281 10.984375 Z M 4.636719 10.984375 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(76.862745%,76.862745%,76.862745%);fill-opacity:1;" d="M 19.289062 9.953125 C 18.992188 8.699219 17.964844 7.8125 16.710938 7.8125 L 7.214844 7.8125 C 5.964844 7.8125 4.933594 8.699219 4.710938 9.953125 Z M 19.289062 9.953125 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(54.509804%,69.411765%,81.960784%);fill-opacity:1;" d="M 16.710938 7.8125 L 13.914062 7.8125 C 14.945312 7.8125 15.828125 8.476562 16.269531 9.363281 C 16.417969 9.730469 16.785156 9.953125 17.226562 9.953125 L 19.289062 9.953125 C 18.992188 8.699219 17.964844 7.8125 16.710938 7.8125 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(0.392157%,26.666667%,38.823529%);fill-opacity:1;" d="M 22.160156 9.953125 L 20.320312 9.953125 C 20.097656 8.183594 18.550781 6.78125 16.710938 6.78125 L 15.535156 6.78125 L 15.3125 5.898438 C 15.09375 5.160156 14.503906 4.644531 13.765625 4.644531 L 11.191406 4.644531 C 10.453125 4.644531 9.792969 5.160156 9.644531 5.898438 L 9.421875 6.78125 L 7.214844 6.78125 C 5.449219 6.78125 3.902344 8.183594 3.605469 9.953125 L 1.765625 9.953125 C 1.03125 9.953125 0.441406 10.542969 0.441406 11.277344 C 0.441406 11.5 0.441406 11.722656 0.589844 11.941406 L 1.25 13.269531 C 1.546875 13.785156 2.0625 14.152344 2.648438 14.152344 L 3.605469 14.152344 L 3.605469 20.417969 C 3.605469 21.597656 4.492188 22.558594 5.667969 22.558594 L 18.257812 22.558594 C 19.4375 22.558594 20.394531 21.597656 20.394531 20.417969 L 20.394531 14.152344 L 21.351562 14.152344 C 21.9375 14.152344 22.453125 13.859375 22.75 13.269531 L 23.410156 11.867188 C 23.558594 11.648438 23.558594 11.5 23.558594 11.277344 C 23.558594 10.542969 22.894531 9.953125 22.160156 9.953125 M 3.605469 13.121094 L 2.648438 13.121094 C 2.429688 13.121094 2.28125 12.972656 2.136719 12.753906 L 1.472656 11.5 L 1.472656 11.277344 C 1.472656 11.132812 1.621094 10.984375 1.765625 10.984375 L 3.605469 10.984375 Z M 10.75 6.117188 C 10.75 5.972656 10.96875 5.75 11.265625 5.75 L 13.839844 5.75 C 14.0625 5.75 14.28125 5.898438 14.355469 6.117188 L 14.503906 6.78125 L 10.601562 6.78125 Z M 7.214844 7.8125 L 16.710938 7.8125 C 17.964844 7.8125 18.992188 8.699219 19.289062 9.953125 L 4.710938 9.953125 C 4.933594 8.699219 5.964844 7.8125 7.214844 7.8125 M 19.363281 13.636719 L 19.363281 20.417969 C 19.363281 21.007812 18.847656 21.527344 18.257812 21.527344 L 5.667969 21.527344 C 5.078125 21.527344 4.636719 21.007812 4.636719 20.417969 L 4.636719 10.984375 L 19.363281 10.984375 Z M 22.453125 11.5 L 21.71875 12.828125 C 21.644531 12.972656 21.496094 13.121094 21.277344 13.121094 L 20.394531 13.121094 L 20.394531 11.058594 L 22.160156 11.058594 C 22.308594 11.058594 22.453125 11.207031 22.453125 11.351562 L 22.453125 11.5 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(92.54902%,94.117647%,97.647059%);fill-opacity:1;" d="M 7.804688 17.324219 C 8.089844 17.324219 8.320312 17.554688 8.320312 17.839844 C 8.320312 18.125 8.089844 18.355469 7.804688 18.355469 C 7.519531 18.355469 7.289062 18.125 7.289062 17.839844 C 7.289062 17.554688 7.519531 17.324219 7.804688 17.324219 M 7.804688 16.808594 C 7.4375 16.808594 7.214844 16.585938 7.214844 16.21875 L 7.214844 13.121094 C 7.214844 12.753906 7.4375 12.53125 7.804688 12.53125 C 8.097656 12.53125 8.320312 12.753906 8.320312 13.121094 L 8.320312 16.21875 C 8.320312 16.585938 8.097656 16.808594 7.804688 16.808594 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(54.509804%,69.411765%,81.960784%);fill-opacity:1;" d="M 16.195312 12.015625 L 16.195312 19.902344 C 16.195312 20.492188 15.679688 21.007812 15.09375 21.007812 L 6.699219 21.007812 C 6.183594 21.007812 5.667969 20.492188 5.667969 19.902344 L 5.667969 12.015625 C 5.667969 11.5 5.226562 10.984375 4.636719 10.984375 L 4.636719 20.417969 C 4.636719 21.007812 5.078125 21.527344 5.667969 21.527344 L 18.257812 21.527344 C 18.847656 21.527344 19.363281 21.007812 19.363281 20.417969 L 19.363281 10.984375 L 17.226562 10.984375 C 16.636719 10.984375 16.195312 11.5 16.195312 12.015625 M 8.246094 7.8125 L 7.214844 7.8125 C 5.964844 7.8125 4.933594 8.699219 4.710938 9.953125 L 4.933594 9.953125 C 5.375 9.953125 5.742188 9.730469 5.890625 9.363281 C 6.332031 8.476562 7.214844 7.8125 8.246094 7.8125 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(0.392157%,26.666667%,38.823529%);fill-opacity:1;" d="M 18.773438 5.75 C 18.625 5.75 18.480469 5.675781 18.40625 5.527344 C 18.183594 5.308594 18.257812 4.9375 18.40625 4.792969 C 18.699219 4.644531 18.773438 4.421875 18.773438 4.128906 C 18.773438 3.90625 18.699219 3.6875 18.480469 3.539062 C 18.257812 3.316406 18.257812 3.023438 18.40625 2.800781 C 18.625 2.582031 18.992188 2.507812 19.140625 2.726562 C 19.582031 3.097656 19.878906 3.613281 19.878906 4.128906 C 19.878906 4.71875 19.582031 5.234375 19.140625 5.601562 L 18.773438 5.75 M 18.773438 1.03125 C 19.058594 1.03125 19.289062 1.261719 19.289062 1.546875 C 19.289062 1.832031 19.058594 2.0625 18.773438 2.0625 C 18.488281 2.0625 18.257812 1.832031 18.257812 1.546875 C 18.257812 1.261719 18.488281 1.03125 18.773438 1.03125 M 16.710938 5.75 L 16.269531 5.527344 C 16.050781 5.308594 16.121094 4.9375 16.34375 4.792969 C 16.5625 4.644531 16.710938 4.421875 16.710938 4.128906 C 16.710938 3.90625 16.5625 3.6875 16.417969 3.539062 C 15.902344 3.097656 15.679688 2.652344 15.679688 2.0625 C 15.679688 1.472656 15.902344 1.03125 16.417969 0.589844 C 16.5625 0.367188 16.933594 0.441406 17.152344 0.664062 C 17.300781 0.8125 17.300781 1.179688 17.078125 1.402344 C 16.785156 1.546875 16.710938 1.769531 16.710938 2.0625 C 16.710938 2.285156 16.785156 2.507812 17.007812 2.652344 C 17.519531 3.097656 17.742188 3.613281 17.742188 4.128906 C 17.742188 4.71875 17.519531 5.234375 17.007812 5.601562 L 16.710938 5.75 "></path></g></svg></span><label><input type="text" name="time-cook" value="01:00:00" class="input input-xs max-w-24 html-duration-picker"></label></div>"#,
                r#"<div class="flex justify-self-center items-center gap-1 cursor-default" title="Prep time"><span data-tip="Prep time" class="tooltip tooltip-left"><svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="24px" height="14px" viewBox="0 0 23 14" version="1.1"><defs><linearGradient id="linear0" gradientUnits="userSpaceOnUse" x1="-125.300003" y1="85.900002" x2="-64.599998" y2="85.900002" gradientTransform="matrix(0.000000000000000013,-0.225806,0.219048,0.000000000000000014,-7.447619,-14.451613)"><stop offset="0.1" style="stop-color:rgb(67.058824%,23.921569%,8.235294%);stop-opacity:1;"></stop><stop offset="0.5" style="stop-color:rgb(78.431373%,51.372549%,30.588235%);stop-opacity:1;"></stop><stop offset="0.8" style="stop-color:rgb(90.196078%,77.254902%,53.333333%);stop-opacity:1;"></stop><stop offset="1" style="stop-color:rgb(94.509804%,87.45098%,62.352941%);stop-opacity:1;"></stop></linearGradient></defs><g id="surface1"><path style=" stroke:none;fill-rule:evenodd;fill:rgb(95.294118%,82.745099%,64.705884%);fill-opacity:1;" d="M 0 8.128906 L 0.21875 6.324219 C 0.4375 5.644531 0.65625 5.195312 1.3125 4.96875 L 8.542969 2.484375 L 8.542969 1.804688 L 8.980469 0.675781 L 9.855469 0.453125 L 10.734375 0.453125 L 10.953125 0.675781 L 12.265625 1.128906 C 13.003906 1 13.761719 1.078125 14.457031 1.355469 C 15.125 1.453125 15.785156 1.601562 16.429688 1.804688 L 17.523438 1.804688 L 18.617188 0.902344 L 20.589844 0.453125 C 21.246094 0.675781 21.6875 0.902344 21.90625 1.355469 C 22.34375 1.804688 22.5625 2.710938 22.34375 4.066406 L 22.125 4.515625 C 22.5625 4.742188 22.78125 5.195312 22.78125 5.644531 L 22.78125 7.675781 L 22.125 8.804688 L 21.027344 9.484375 L 17.304688 11.289062 L 16.210938 11.742188 L 12.921875 13.324219 L 12.046875 13.773438 L 11.390625 14 L 10.078125 14 L 8.542969 13.546875 C 6.269531 12.441406 4.007812 11.308594 1.753906 10.160156 L 0.65625 9.257812 C 0.21875 9.03125 0 8.582031 0 8.128906 Z M 0 8.128906 "></path><path style=" stroke:none;fill-rule:evenodd;fill:url(#linear0);" d="M 1.3125 4.742188 L 8.542969 2.03125 L 8.542969 1.582031 L 8.980469 0.453125 C 9.199219 0.226562 9.636719 0 9.855469 0.226562 L 10.953125 0.226562 L 12.265625 0.902344 C 13.003906 0.773438 13.761719 0.851562 14.457031 1.128906 L 15.769531 1.355469 C 16.308594 1.65625 16.933594 1.738281 17.523438 1.582031 L 17.523438 1.355469 C 17.742188 1.128906 18.179688 0.675781 18.617188 0.675781 C 19.277344 0.226562 19.933594 0.226562 20.589844 0.226562 C 21.027344 0.453125 21.6875 0.675781 21.90625 1.128906 C 22.34375 1.582031 22.5625 2.484375 22.34375 3.839844 L 22.125 4.289062 C 22.5625 4.515625 22.78125 4.96875 22.78125 5.417969 L 22.78125 6.546875 L 22.5625 7.453125 L 22.125 8.582031 L 21.027344 9.257812 L 17.085938 11.289062 L 16.210938 11.515625 L 12.921875 13.097656 L 12.046875 13.546875 L 11.390625 13.773438 L 9.855469 13.773438 L 8.324219 13.324219 C 6.121094 12.21875 3.929688 11.089844 1.753906 9.933594 L 1.535156 9.933594 L 0.4375 9.03125 L 0 7.902344 C 0 7.292969 0.0742188 6.6875 0.21875 6.097656 C 0.21875 5.417969 0.65625 4.96875 1.3125 4.742188 Z M 1.3125 4.742188 "></path><path style="fill:none;stroke-width:0.3;stroke-linecap:butt;stroke-linejoin:miter;stroke:rgb(95.294118%,82.745099%,64.705884%);stroke-opacity:1;stroke-miterlimit:4;" d="M 5.991848 21.001116 L 39.999151 8.995536 L 39.00051 6.00279 L 40.997792 2.006696 L 46.008832 0 L 47.007473 0 L 49.004755 1.003348 L 50.003397 1.003348 L 55.995245 3.996094 C 59.579654 2.923549 63.413723 2.923549 66.998132 3.996094 L 73.007812 6.00279 C 75.272588 6.763951 77.733526 6.763951 79.998302 6.00279 L 84.991508 2.006696 C 88.005265 1.003348 91.001189 0 93.997113 1.003348 C 96.993037 1.003348 99.008152 2.006696 101.005435 3.996094 C 103.002717 6.00279 103.002717 9.998884 102.004076 16.001674 L 102.004076 18.008371 L 104.001359 23.007812 L 105 28.007254 L 104.001359 33.006696 L 101.005435 37.00279 L 95.994395 40.998884 L 78.99966 49.008371 L 75.005095 50.997768 L 74.006454 50.997768 L 58.991168 58.003906 L 54.996603 59.993304 L 52.000679 59.993304 L 51.002038 60.996652 L 46.008832 60.996652 L 39.00051 59.007254 C 28.60394 54.128906 18.278702 49.129464 8.006963 43.991629 L 8.006963 43.00558 L 2.995924 39.995536 L 0 33.992746 C 0 31.294085 0.338825 28.612723 0.998641 26.000558 C 1.997283 23.993862 2.995924 22.004464 5.991848 21.001116 Z M 5.991848 21.001116 " transform="matrix(0.219048,0,0,0.225806,0,0)"></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(25.882354%,9.411765%,1.568628%);fill-opacity:1;" d="M 1.3125 8.128906 L 1.09375 7.675781 L 1.3125 6.097656 L 1.753906 5.644531 L 9.855469 2.933594 L 9.636719 2.257812 C 9.710938 1.882812 9.785156 1.503906 9.855469 1.128906 L 10.078125 1.128906 L 10.515625 1.355469 L 10.734375 1.355469 L 12.265625 2.03125 C 12.914062 1.859375 13.589844 1.859375 14.238281 2.03125 C 14.910156 2.207031 15.570312 2.433594 16.210938 2.710938 L 16.648438 2.710938 L 17.960938 2.484375 L 18.398438 2.03125 L 19.058594 1.582031 L 20.371094 1.355469 L 21.246094 1.804688 L 21.246094 3.839844 L 21.027344 4.066406 L 20.371094 4.515625 L 20.589844 4.515625 L 21.464844 4.96875 L 21.90625 5.417969 L 21.90625 6.324219 L 21.6875 7 L 21.464844 7.675781 L 20.589844 8.128906 C 19.933594 8.582031 18.839844 9.257812 16.867188 9.933594 L 12.484375 11.96875 L 11.828125 12.417969 C 11.617188 12.515625 11.394531 12.589844 11.171875 12.644531 L 10.296875 12.644531 L 8.980469 12.195312 C 6.703125 11.09375 4.441406 9.964844 2.191406 8.804688 Z M 1.3125 8.128906 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(65.490198%,51.372552%,26.274511%);fill-opacity:1;" d="M 6.351562 5.195312 C 8.921875 4.820312 11.476562 4.371094 14.019531 3.839844 L 15.332031 4.289062 L 16.429688 4.515625 C 17.304688 4.515625 17.742188 4.289062 17.960938 4.066406 L 18.179688 3.613281 L 18.617188 3.160156 L 19.933594 2.484375 L 21.027344 2.03125 L 21.027344 3.839844 L 20.808594 4.066406 C 20.371094 4.289062 19.933594 4.515625 19.277344 4.289062 L 17.960938 4.742188 L 19.496094 4.96875 L 21.464844 5.644531 L 21.464844 7.226562 L 21.246094 7.453125 L 20.371094 8.128906 C 17.839844 9.550781 15.203125 10.757812 12.484375 11.742188 L 11.171875 12.417969 C 10.515625 12.417969 9.636719 12.417969 8.980469 11.96875 C 6.324219 10.59375 3.695312 9.164062 1.09375 7.675781 L 1.3125 7 L 1.535156 6.324219 Z M 6.351562 5.195312 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(56.078434%,44.313726%,22.352941%);fill-opacity:1;" d="M 4.382812 7.902344 L 3.503906 7.902344 L 3.285156 9.257812 L 3.066406 9.03125 L 3.285156 7.675781 L 2.847656 7.226562 L 2.628906 7.453125 L 2.410156 8.804688 L 2.191406 8.582031 L 1.972656 8.582031 L 2.410156 7.226562 L 1.972656 7 L 1.753906 8.355469 L 1.3125 8.128906 L 1.535156 6.546875 L 6.351562 6.324219 L 10.078125 8.128906 L 10.953125 10.839844 L 11.171875 12.417969 L 10.296875 12.417969 L 10.515625 10.839844 L 9.417969 10.613281 L 9.199219 12.195312 L 8.542969 11.96875 L 8.761719 10.386719 L 8.105469 10.160156 L 7.886719 11.515625 L 7.449219 11.289062 L 7.449219 9.710938 L 7.230469 9.03125 L 6.570312 9.257812 L 6.132812 10.613281 L 5.476562 10.386719 L 5.914062 9.03125 L 4.820312 8.128906 L 4.601562 8.355469 L 4.382812 9.710938 L 4.160156 9.484375 Z M 4.382812 7.902344 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(52.941179%,41.176471%,18.82353%);fill-opacity:1;" d="M 19.933594 2.484375 L 19.933594 2.710938 C 18.839844 3.160156 18.617188 3.839844 19.496094 4.289062 L 18.617188 4.289062 L 17.960938 4.742188 L 19.496094 4.96875 L 21.464844 5.644531 L 21.464844 7.226562 L 21.246094 7.453125 L 20.371094 8.128906 C 17.839844 9.550781 15.203125 10.757812 12.484375 11.742188 L 11.828125 12.195312 L 10.515625 12.417969 L 10.734375 12.195312 L 10.734375 9.710938 L 10.515625 8.804688 C 13.609375 6.628906 16.75 4.519531 19.933594 2.484375 Z M 19.933594 2.484375 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(47.450981%,36.470589%,16.862746%);fill-opacity:1;" d="M 21.027344 7.453125 C 21.464844 7 21.464844 6.546875 21.464844 5.644531 L 21.464844 7.226562 L 21.246094 7.453125 L 20.371094 8.128906 C 17.839844 9.550781 15.203125 10.757812 12.484375 11.742188 L 11.828125 12.195312 L 10.515625 12.417969 L 10.734375 12.195312 L 11.171875 12.195312 L 12.046875 11.96875 C 15.042969 10.46875 18.035156 8.960938 21.027344 7.453125 Z M 21.027344 7.453125 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(87.450981%,71.764708%,40.784314%);fill-opacity:1;" d="M 1.753906 6.097656 C 5.445312 4.722656 9.167969 3.441406 12.921875 2.257812 L 14.238281 2.484375 L 15.550781 2.933594 L 16.648438 3.160156 C 17.523438 3.160156 17.960938 2.933594 18.179688 2.710938 L 18.617188 2.257812 L 19.058594 2.03125 C 19.714844 1.582031 20.152344 1.582031 20.808594 1.804688 C 21.246094 2.03125 21.246094 2.484375 20.808594 2.710938 L 19.496094 3.160156 L 18.179688 3.613281 L 19.058594 4.289062 C 19.855469 4.75 20.660156 5.203125 21.464844 5.644531 C 21.6875 5.871094 21.246094 6.324219 20.589844 6.773438 C 17.578125 8.257812 14.511719 9.613281 11.390625 10.839844 C 10.734375 11.066406 9.855469 10.839844 8.980469 10.613281 C 6.472656 9.3125 3.988281 7.957031 1.535156 6.546875 L 1.535156 6.097656 Z M 1.753906 6.097656 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(79.215688%,64.313728%,33.333334%);fill-opacity:1;" d="M 2.628906 7.226562 L 2.410156 7.226562 L 4.820312 6.097656 L 6.351562 4.742188 L 8.105469 3.839844 C 9.554688 3.546875 11.015625 3.320312 12.484375 3.160156 C 12.921875 3.160156 13.582031 2.933594 14.019531 2.484375 L 14.457031 2.484375 C 12.945312 3.640625 11.078125 4.203125 9.199219 4.066406 C 8.105469 4.066406 7.230469 4.289062 6.570312 4.742188 L 5.039062 6.097656 C 4.601562 6.546875 3.722656 7 2.628906 7.226562 Z M 2.628906 7.226562 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(79.215688%,64.313728%,33.333334%);fill-opacity:1;" d="M 5.257812 7 C 4.601562 7 3.941406 7.226562 3.503906 7.675781 L 3.285156 7.675781 C 4.5625 6.878906 5.976562 6.34375 7.449219 6.097656 L 10.296875 5.417969 L 11.609375 4.742188 L 12.484375 4.066406 L 14.675781 2.484375 L 14.894531 2.710938 L 12.921875 4.289062 L 10.953125 5.644531 C 9.855469 6.324219 7.886719 6.773438 5.257812 7 Z M 1.972656 6.324219 L 3.066406 5.871094 L 4.160156 5.195312 L 6.132812 4.515625 L 5.476562 4.96875 L 3.722656 6.097656 L 2.191406 6.773438 L 1.972656 6.773438 L 1.535156 6.546875 Z M 9.855469 3.160156 L 11.609375 2.710938 L 13.363281 2.257812 L 13.582031 2.257812 C 13.144531 2.710938 12.484375 2.933594 11.828125 2.933594 Z M 18.617188 7.675781 C 19.277344 6.546875 20.152344 5.871094 21.246094 5.417969 L 21.464844 5.644531 C 20.808594 5.871094 20.152344 6.324219 19.714844 7 Z M 9.199219 7.902344 C 10.078125 7.902344 10.953125 7.675781 12.046875 7 L 14.019531 5.417969 L 15.550781 4.066406 C 16.210938 3.613281 16.648438 3.160156 17.304688 3.160156 L 18.179688 2.710938 L 20.589844 1.804688 L 20.808594 1.804688 L 20.589844 2.03125 L 18.398438 2.933594 L 16.210938 3.839844 L 14.457031 5.417969 C 13.800781 6.324219 13.144531 6.773438 12.703125 7 C 12.265625 7.453125 11.390625 7.902344 10.078125 8.128906 L 7.886719 8.582031 L 6.570312 9.257812 L 5.914062 8.804688 L 7.230469 8.355469 Z M 16.867188 6.097656 C 17.304688 5.195312 17.960938 4.742188 18.839844 4.289062 L 19.496094 4.515625 L 17.742188 5.871094 L 15.992188 7.902344 C 15.113281 8.582031 14.457031 9.03125 13.582031 9.257812 L 10.953125 9.710938 L 9.417969 10.613281 L 8.761719 10.386719 L 10.515625 9.484375 L 12.703125 9.03125 C 14.382812 8.582031 15.851562 7.542969 16.867188 6.097656 Z M 16.867188 6.097656 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(79.215688%,64.313728%,33.333334%);fill-opacity:1;" d="M 6.789062 7.675781 C 5.914062 7.675781 5.257812 7.902344 4.601562 8.355469 L 4.160156 8.128906 C 4.820312 7.675781 5.914062 7.226562 7.230469 7.226562 L 10.953125 6.097656 C 12.046875 5.644531 12.921875 4.96875 13.582031 4.289062 L 15.332031 2.710938 L 15.992188 2.933594 L 14.019531 4.515625 L 12.265625 5.871094 L 9.636719 7.226562 Z M 20.152344 4.742188 L 20.589844 4.96875 L 18.839844 6.324219 C 18.179688 6.773438 17.742188 7.453125 17.304688 8.355469 C 14.960938 9.164062 12.625 9.992188 10.296875 10.839844 L 12.703125 9.933594 L 14.894531 9.03125 C 15.992188 8.582031 17.304688 7.453125 18.617188 5.644531 Z M 13.144531 7.453125 C 14.671875 6.238281 16.203125 5.035156 17.742188 3.839844 C 17.960938 3.386719 19.058594 2.933594 21.027344 2.03125 L 21.027344 2.484375 C 20.402344 2.570312 19.804688 2.800781 19.277344 3.160156 L 18.179688 3.613281 L 18.398438 3.839844 L 15.769531 6.324219 L 13.582031 8.128906 L 10.515625 8.804688 C 9.417969 9.03125 8.761719 9.484375 8.105469 9.933594 L 7.449219 9.710938 C 9.289062 8.8125 11.191406 8.058594 13.144531 7.453125 Z M 6.570312 5.871094 L 6.570312 5.644531 L 6.789062 5.195312 L 7.230469 4.515625 L 8.761719 4.289062 L 10.515625 4.289062 C 10.734375 4.515625 10.734375 4.742188 10.296875 4.96875 L 8.761719 5.644531 L 7.449219 5.871094 L 7.449219 5.195312 L 8.324219 4.742188 L 9.199219 4.515625 L 9.417969 4.742188 L 8.761719 5.195312 L 8.980469 4.96875 L 8.324219 4.96875 L 8.105469 5.195312 C 7.886719 5.417969 8.105469 5.417969 8.324219 5.417969 L 9.199219 5.195312 L 9.855469 4.742188 L 9.855469 4.515625 L 8.761719 4.515625 L 7.449219 4.96875 L 6.789062 5.417969 Z M 6.570312 5.871094 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(41.960785%,25.098041%,14.117648%);fill-opacity:1;" d="M 9.855469 3.160156 L 11.171875 2.710938 L 12.265625 3.839844 L 14.238281 5.417969 L 15.550781 7 L 16.210938 8.582031 L 15.992188 9.484375 L 15.332031 9.710938 L 14.894531 9.710938 L 14.894531 9.257812 L 15.113281 9.03125 L 15.332031 8.582031 L 14.675781 7.675781 L 14.457031 7.453125 L 14.457031 7.226562 L 14.238281 6.773438 L 14.019531 6.773438 C 13.304688 7.003906 12.570312 7.15625 11.828125 7.226562 L 11.171875 7.226562 L 10.734375 6.097656 L 10.078125 4.289062 Z M 9.855469 3.160156 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(47.843137%,47.843137%,47.843137%);fill-opacity:1;" d="M 11.171875 7 L 11.390625 5.871094 L 12.265625 4.96875 L 13.582031 4.96875 L 14.894531 5.195312 L 15.113281 5.871094 L 14.894531 6.097656 L 12.921875 6.773438 L 11.609375 7 Z M 11.171875 7 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(59.215689%,59.215689%,59.215689%);fill-opacity:1;" d="M 12.265625 6.097656 L 12.046875 6.546875 L 12.265625 7 L 11.171875 7 L 11.390625 6.324219 Z M 12.265625 6.097656 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(92.54902%,92.54902%,92.54902%);fill-opacity:1;" d="M 9.855469 1.582031 L 10.734375 1.804688 L 12.921875 2.933594 C 13.75 3.667969 14.488281 4.5 15.113281 5.417969 L 14.894531 5.417969 L 14.019531 4.289062 L 12.484375 2.710938 L 10.734375 1.804688 Z M 9.855469 1.582031 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(78.039217%,78.039217%,78.039217%);fill-opacity:1;" d="M 9.855469 1.582031 L 10.078125 1.582031 L 10.515625 1.804688 C 10.609375 3.429688 11.140625 4.992188 12.046875 6.324219 L 11.171875 7 L 10.953125 6.546875 C 10.421875 5.246094 10.054688 3.882812 9.855469 2.484375 Z M 9.855469 1.582031 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(89.411765%,89.411765%,89.411765%);fill-opacity:1;" d="M 10.078125 3.386719 L 10.078125 2.933594 L 10.734375 2.933594 L 10.734375 3.160156 Z M 9.855469 2.03125 L 10.515625 2.03125 L 10.515625 2.710938 L 9.855469 2.710938 Z M 11.828125 6.097656 L 11.171875 6.773438 L 10.953125 6.324219 L 11.609375 5.871094 Z M 10.296875 4.515625 L 10.078125 3.839844 L 10.734375 3.613281 L 10.953125 4.066406 Z M 11.390625 5.195312 L 10.734375 5.644531 L 10.515625 4.96875 L 10.953125 4.515625 Z M 11.390625 5.195312 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(56.470591%,56.470591%,56.470591%);fill-opacity:1;" d="M 14.238281 6.546875 L 14.019531 6.324219 L 14.019531 5.871094 L 14.675781 5.871094 L 15.113281 6.097656 L 15.113281 6.324219 L 14.894531 6.546875 Z M 14.238281 6.546875 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(70.19608%,70.19608%,70.19608%);fill-opacity:1;" d="M 14.019531 5.871094 L 14.238281 5.644531 L 14.894531 5.644531 L 15.113281 5.871094 L 15.113281 6.097656 L 14.238281 6.097656 Z M 14.019531 5.871094 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(55.686277%,35.686275%,17.254902%);fill-opacity:1;" d="M 14.238281 6.773438 L 14.238281 6.097656 L 14.894531 6.097656 L 15.550781 6.773438 L 16.429688 8.128906 L 16.429688 8.582031 L 15.769531 9.484375 C 15.113281 9.710938 14.675781 9.710938 14.894531 9.257812 L 15.113281 8.582031 L 15.113281 8.128906 L 14.894531 7.675781 L 14.675781 7.453125 L 14.457031 6.773438 Z M 14.238281 6.773438 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(43.137255%,27.058825%,13.725491%);fill-opacity:1;" d="M 15.769531 8.355469 L 16.210938 7.675781 L 16.429688 8.128906 L 16.429688 8.582031 C 16.429688 9.03125 16.210938 9.484375 15.769531 9.484375 L 14.894531 9.484375 L 14.894531 9.03125 L 15.113281 8.582031 L 15.332031 8.355469 Z M 15.769531 8.355469 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(64.313728%,44.705883%,26.666668%);fill-opacity:1;" d="M 14.675781 6.324219 L 14.457031 6.097656 L 14.457031 5.871094 L 15.113281 5.871094 L 15.769531 6.097656 L 16.429688 7.226562 L 16.429688 8.582031 C 15.992188 8.804688 15.550781 9.03125 15.113281 8.804688 L 15.113281 8.355469 L 15.550781 7.902344 L 15.332031 7.453125 L 14.894531 7.226562 L 14.675781 6.773438 Z M 14.675781 6.324219 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 15.113281 8.128906 L 15.550781 7.902344 L 15.332031 8.355469 L 15.332031 8.804688 L 15.113281 8.804688 Z M 14.457031 5.871094 L 14.894531 6.546875 L 15.332031 7.453125 L 15.113281 7.453125 L 14.894531 6.773438 L 14.894531 6.546875 L 14.675781 6.546875 L 14.238281 6.097656 Z M 16.429688 7.675781 L 16.429688 7.453125 C 16.648438 7.902344 16.648438 8.355469 16.210938 8.582031 Z M 16.429688 7.675781 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 14.894531 6.097656 L 15.332031 6.546875 L 15.550781 6.773438 L 15.550781 7 L 15.769531 7.453125 L 15.769531 8.128906 L 15.550781 8.804688 L 15.550781 7 L 14.675781 5.871094 Z M 14.894531 6.097656 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 15.550781 6.324219 L 15.992188 6.546875 L 16.210938 7.226562 L 16.210938 8.128906 L 15.992188 8.804688 L 15.992188 6.546875 C 15.695312 6.324219 15.402344 6.101562 15.113281 5.871094 L 15.332031 5.871094 Z M 15.550781 6.324219 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 15.113281 5.871094 L 15.332031 6.324219 L 15.550781 6.773438 L 15.769531 7.226562 L 15.992188 7.453125 L 15.992188 8.128906 L 15.769531 8.804688 L 15.769531 7 L 15.550781 6.773438 L 15.332031 6.324219 L 14.894531 5.871094 Z M 15.332031 5.871094 L 15.769531 6.324219 L 15.992188 6.546875 L 15.550781 6.324219 Z M 15.332031 5.871094 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(43.137255%,27.058825%,13.725491%);fill-opacity:1;" d="M 16.210938 8.355469 C 15.992188 8.582031 15.769531 8.804688 15.550781 8.582031 L 15.332031 8.355469 L 15.992188 8.128906 Z M 16.210938 8.355469 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(69.411767%,69.411767%,69.411767%);fill-opacity:1;" d="M 16.210938 8.355469 L 15.992188 8.582031 L 15.332031 8.582031 L 15.769531 8.355469 Z M 16.210938 8.355469 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(49.019608%,49.019608%,49.019608%);fill-opacity:1;" d="M 16.210938 8.355469 L 15.992188 8.582031 L 15.332031 8.582031 L 15.769531 8.582031 L 15.992188 8.355469 Z M 16.210938 8.355469 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 15.992188 6.773438 L 15.769531 6.773438 L 15.992188 7 L 15.992188 6.773438 L 15.992188 7 L 15.769531 7 L 15.769531 6.773438 Z M 15.992188 6.773438 "></path></g></svg></span><label><input type="text" name="time-prep" value="00:02:00" class="input input-xs max-w-24 html-duration-picker"></label></div>"#,
                r#"<table class="table table-zebra table-xs"><thead><tr><th>Nutrition (per 100g)</th><th>Amount</th></tr></thead><tbody><tr><td>Calories</td><td><label><input type="text" name="calories" autocomplete="off" placeholder="368kcal" class="input input-xs max-w-24" value="300 kcal"></label></td></tr><tr><td>Total carbs</td><td><label><input type="text" name="total-carbohydrates" autocomplete="off" placeholder="35g" class="input input-xs max-w-24" value="55g"></label></td></tr><tr><td>Sugars</td><td><label><input type="text" name="sugars" autocomplete="off" placeholder="3g" class="input input-xs max-w-24" value="43g"></label></td></tr><tr><td>Protein</td><td><label><input type="text" name="protein" autocomplete="off" placeholder="21g" class="input input-xs max-w-24" value="7g"></label></td></tr><tr><td>Total fat</td><td><label><input type="text" name="total-fat" autocomplete="off" placeholder="15g" class="input input-xs max-w-24" value="6g"></label></td></tr><tr><td>Saturated fat</td><td><label><input type="text" name="saturated-fat" autocomplete="off" placeholder="1.8g" class="input input-xs max-w-24" value="1g"></label></td></tr><tr><td>Unsaturated fat</td><td><label><input type="text" name="unsaturated-fat" autocomplete="off" placeholder="1.8g" class="input input-xs max-w-24" value="2g"></label></td></tr><tr><td>Trans fat</td><td><label><input type="text" name="trans-fat" autocomplete="off" placeholder="1.8g" class="input input-xs max-w-24" value="3g"></label></td></tr><tr><td>Cholesterol</td><td><label><input type="text" name="cholesterol" autocomplete="off" placeholder="1.1mg" class="input input-xs max-w-24" value="5mg"></label></td></tr><tr><td>Sodium</td><td><label><input type="text" name="sodium" autocomplete="off" placeholder="100mg" class="input input-xs max-w-24" value="12mg"></label></td></tr><tr><td>Fiber</td><td><label><input type="text" name="fiber" autocomplete="off" placeholder="8g" class="input input-xs max-w-24" value="10g"></label></td></tr></tbody></table>"#,
                r#"<h2 class="font-semibold text-center pb-2"><span class="underline">Tools</span></h2>"#,
                r#"<ol id="tools-list" class="pl-4"><li class="pb-2"><div class="grid grid-flow-col items-center"><label class="flex gap-1"><div class="inline-block h-4 cursor-move handle mt-1"><svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"></path></svg></div><input type="text" name="tool" placeholder="1 frying pan" class="input input-bordered input-sm w-full" value="1 wok" _="on keydown if event.key is 'Enter' halt the event then call addItem(event)"></label><div class="ml-2 flex gap-2"><button type="button" class="btn btn-square btn-sm btn-outline btn-success" title="Shortcut: Enter" onclick="addItem(event)">+</button><button type="button" class="delete-button btn btn-square btn-sm btn-outline btn-error""#,
                r#"<input type="text" name="tool" placeholder="1 frying pan" class="input input-bordered input-sm w-full" value="1 frying pan" _="on keydown if event.key is 'Enter' halt the event then call addItem(event)">"#,
                r#"<h2 class="font-semibold text-center pb-2"><span class="underline">Ingredients</span><sup class="text-red-600">*</sup></h2>"#,
                r#"<ol id="ingredients-list" class="pl-4"><li class="pb-2"><div class="grid grid-flow-col items-center"><label class="flex gap-1"><div class="inline-block h-4 cursor-move handle mt-1"><svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"></path></svg></div><input required type="text" name="ingredient" value="1 cup blue spinach" placeholder="1 cup of chopped onions" class="input input-bordered input-sm w-full" _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))"></label><div class="ml-2 flex gap-2"><button type="button" class="btn btn-square btn-sm btn-outline btn-success" title="Shortcut: Enter" onclick="addItem(event)">+</button><button type="button" class="delete-button btn btn-square btn-sm btn-outline btn-error""#,
                r#"<input required type="text" name="ingredient" value="1/2 tbsp cinnamon" placeholder="1 cup of chopped onions" class="input input-bordered input-sm w-full" _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))">"#,
                r#"<input required type="text" name="ingredient" value="4 pounds top quality chicken filet" placeholder="1 cup of chopped onions" class="input input-bordered input-sm w-full" _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))">"#,
                r#"<input required type="text" name="ingredient" value="1/8 cup lemon juice" placeholder="1 cup of chopped onions" class="input input-bordered input-sm w-full" _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))">"#,
                r#"<h2 class="font-semibold text-center pb-2"><span class="underline">Instructions</span><sup class="text-red-600">*</sup></h2>"#,
                r##"<ol id="instructions-list" class="grid list-decimal"><li class="pt-2 md:pl-0"><div class="flex"><label class="w-11/12"><textarea required name="instruction" rows="4" class="textarea textarea-bordered w-full" placeholder="Mix all ingredients together" _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))">Mix all these ingredients</textarea></label><div class="grid gap-2 ml-2"><button type="button" class="btn btn-square btn-sm btn-outline btn-success" title="Shortcut: CTRL + Enter" onclick="addItem(event)">+</button><button type="button" class="delete-button btn btn-square btn-sm btn-outline btn-error"##,
                r#"<textarea required name="instruction" rows="4" class="textarea textarea-bordered w-full" placeholder="Mix all ingredients together" _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))">Turn the oven at 300 F</textarea>"#,
                r#"<textarea required name="instruction" rows="4" class="textarea textarea-bordered w-full" placeholder="Mix all ingredients together" _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))">Soak the chicken in the lemon juice</textarea>"#,
                r#"<textarea required name="instruction" rows="4" class="textarea textarea-bordered w-full" placeholder="Mix all ingredients together" _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))">Mix all these ingredients</textarea>"#,
                r#"<button class="btn btn-primary btn-block btn-sm">Submit</button>"#,
            ];
            for want in expected {
                res.assert_text_contains(want);
            }
        }

        fn recipe_for_create_to_details(
            recipe_c: RecipeForCreate,
            reference: &RecipeDetails,
        ) -> RecipeDetails {
            let mut keywords = recipe_c.keywords;
            keywords.sort();

            RecipeDetails {
                recipe: Recipe {
                    id: 1,
                    name: recipe_c.name,
                    description: recipe_c.description,
                    image: reference.recipe.image,
                    yield_: recipe_c.yield_.unwrap_or(4),
                    language: "eng".into(),
                    measurement_system_id: 2,
                    source: recipe_c.source,
                    created_at: reference.recipe.created_at,
                    updated_at: reference.recipe.updated_at,
                    user_id: 1,
                },
                additional_images: reference.additional_images.clone(),
                category: recipe_c.category.unwrap_or("uncategorized".into()),
                cuisine: recipe_c.cuisine,
                ingredients: recipe_c.ingredients,
                instructions: recipe_c.instructions,
                keywords,
                nutrition: recipe_c.nutrition.map(|n| Nutrition {
                    id: 1,
                    recipe_id: 1,
                    calories_kcal: n.calories_kcal,
                    total_carbohydrates: n.total_carbohydrates,
                    sugars_g: n.sugars_g,
                    protein_g: n.protein_g,
                    total_fat_g: n.total_fat_g,
                    saturated_fat_g: n.saturated_fat_g,
                    unsaturated_fat_g: n.unsaturated_fat_g,
                    cholesterol_mg: n.cholesterol_mg,
                    sodium_mg: n.sodium_mg,
                    fiber_g: n.fiber_g,
                    trans_fat_g: n.trans_fat_g,
                    serving_size: n.serving_size,
                }),
                times: Times {
                    id: 1,
                    recipe_id: 1,
                    prep_seconds: recipe_c.times.clone().unwrap_or_default().prep_seconds,
                    cook_seconds: recipe_c.times.clone().unwrap_or_default().cook_seconds,
                    total_seconds: recipe_c.times.clone().unwrap_or_default().prep_seconds
                        + recipe_c.times.clone().unwrap_or_default().cook_seconds,
                },
                tools: recipe_c
                    .tools
                    .into_iter()
                    .enumerate()
                    .map(|(idx, t)| ToolRecipe {
                        name: t.name,
                        quantity: t.quantity,
                        tool_order: (idx + 1) as i16,
                    })
                    .collect(),
                videos: vec![],
            }
        }
    }

    mod tests_scale_recipe {
        use super::*;
        use axum_test::{TestServer, TestWebSocket};
        use models::Recipe;
        use models::recipe::test_utils::a_complete_recipe_for_create;
        use recipe_schema::Sections;

        fn base_uri(recipe_id: i64) -> String {
            format!("/recipes/{recipe_id}/scale")
        }

        async fn setup(config: Config) -> Result<(TestServer, TestWebSocket)> {
            let (server, ws_server) = build_server_ws(config.clone()).await?;
            let state = create_app_state(config).await;
            let _ = Recipe::create(&state.mm, 1, &a_complete_recipe_for_create()).await?;
            Ok((server, ws_server))
        }

        #[tokio::test]
        async fn test_must_be_logged_in() -> Result<()> {
            assert_must_be_logged_in(Method::GET, &base_uri(1)).await
        }

        #[tokio::test]
        async fn test_yield_query_param_must_be_specified() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, _ws_server) = setup(config).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_bad_request();
            res.assert_text("Failed to deserialize query string: missing field `yield`");
            Ok(())
        }

        #[tokio::test]
        async fn test_query_param_must_not_be_negative() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, _ws_server) = setup(config).await?;

            let res = server.get(&format!("{}?yield=-1", base_uri(1))).await;

            res.assert_status_bad_request();
            res.assert_text(
                "Failed to deserialize query string: yield: invalid digit found in string",
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_query_param_must_be_greater_than_0() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = setup(config).await?;

            let res = server.get(&format!("{}?yield=0", base_uri(1))).await;

            res.assert_status_bad_request();
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Yield must be greater than zero.","status":"alert-error","title":"Operation Failed"}}"#).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_cannot_find_recipe_in_database() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = setup(config).await?;

            let res = server.get(&format!("{}?yield=8", base_uri(999))).await;

            res.assert_status_not_found();
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Recipe not found.","status":"alert-error","title":"Operation Failed"}}"#).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_valid_double_yield() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, _ws_server) = setup(config.clone()).await?;
            let state = create_app_state(config).await;
            let recipe_id = Recipe::create(
                &state.mm,
                1,
                &RecipeForCreate {
                    name: "Best Chinese Kale".into(),
                    yield_: Some(4),
                    ingredients: Sections::from([
                        (
                            "Sauce".into(),
                            Vec::<String>::from([
                                "1 cup blue spinach".into(),
                                "1/2 tbsp cinnamon".into(),
                                "2lb chicken".into(),
                                "1/2 cup bread loaf".into(),
                                "½ tbsp beef broth".into(),
                                "7 1/2 cups flour".into(),
                                "2 big apples".into(),
                                "Lots of big apples".into(),
                                "2.5 slices of bacon".into(),
                                "2 1/3 cans of bamboo sticks".into(),
                                "1½can of tomato paste".into(),
                                "6 ¾ peanut butter jars".into(),
                                "7.5mL of whiskey".into(),
                                "2 tsp lemon juice".into(),
                            ]),
                        ),
                        (
                            "Main".into(),
                            Vec::<String>::from([
                                "Ground ginger".into(),
                                "3 Large or 4 medium ripe Hass avocados".into(),
                                "1/4-1/2 teaspoon salt plus more for seasoning".into(),
                                "1/2 fresh pineapple, cored and cut into 1 1/2-inch pieces".into(),
                                "Un sac de chips de 1kg".into(),
                                "Two 15-ounce can Goya beans".into(),
                                "4 pounds top quality chicken filet".into(),
                                "1/8 cup lemon juice".into(),
                            ]),
                        ),
                    ]),
                    instructions: Sections::from([(
                        "Sauce".into(),
                        Vec::<String>::from(["Mix all these ingredients".into()]),
                    )]),
                    ..Default::default()
                },
            )
            .await?;

            let res = server
                .get(&format!("{}?yield=8", base_uri(recipe_id)))
                .await;

            res.assert_status_ok();
            assert_html(
                res.clone(),
                vec![
                    r#"<span class="pl-2">2 cup blue spinach</span>"#,
                    r#"<span class="pl-2">1 tbsp cinnamon</span>"#,
                    r#"<span class="pl-2">4 lb chicken</span>"#,
                    r#"<span class="pl-2">1 cup bread loaf</span>"#,
                    r#"<span class="pl-2">1 tbsp beef broth</span>"#,
                    r#"<span class="pl-2">15 cup flour</span>"#,
                    r#"<span class="pl-2">4 big apples</span>"#,
                    r#"<span class="pl-2">Lots of big apples</span>"#,
                    r#"<span class="pl-2">5 slices of bacon</span>"#,
                    r#"<span class="pl-2">4 2/3 cans of bamboo sticks</span>"#,
                    r#"<span class="pl-2">3can of tomato paste</span>"#,
                    r#"<span class="pl-2">13 1/2 peanut butter jars</span>"#,
                    r#"<span class="pl-2">1 1/2 cl of whiskey</span>"#,
                    r#"<span class="pl-2">1 1/3 tbsp lemon juice</span>"#,
                    r#"<span class="pl-2">Ground ginger</span>"#,
                    r#"<span class="pl-2">1 fresh pineapple, cored and cut into 1 1/2-inch pieces</span>"#,
                    r#"<span class="pl-2">Un sac de chips de 2 kg</span>"#,
                    r#"<span class="pl-2">Two 30-ounce can Goya beans</span>"#,
                    r#"<span class="pl-2">8 lb top quality chicken filet</span>"#,
                    r#"<span class="pl-2">1 1/3 tbsp lemon juice</span>"#,
                ],
            );
            Ok(())
        }
    }

    mod tests_share_recipe {
        use super::*;

        use diesel::internal::derives::multiconnection::chrono;
        use diesel::prelude::*;
        use diesel_async::RunQueryDsl;

        use crate::recipes_routes::ShareRecipeForm;
        use app::state::AppState;
        use models::Recipe;
        use models::recipe::test_utils::a_complete_recipe_for_create;
        use models::share::ShareRecipe;
        use repository::schema;

        fn base_uri(recipe_id: i64) -> String {
            format!("/recipes/{recipe_id}/share")
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::POST, &base_uri(1)).await
        }

        #[tokio::test]
        async fn test_default_expires_at_time_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let _ = Recipe::create(&state.mm, 1, &a_complete_recipe_for_create()).await?;

            let res = server
                .post(&base_uri(1))
                .form(&ShareRecipeForm { datetime: None })
                .await;

            let share = get_first_shared_recipe(state).await;
            res.assert_status_ok();
            assert_html(
                res,
                vec![
                    &format!(
                        r#"<label><input class="input" type="url" value="http://localhost:8078/shared/r/{}" readonly="readonly"></label>"#,
                        share.link
                    ),
                    &format!(
                        r#"<button class="btn btn-neutral" id="copy-button" title="Copy to clipboard" onClick="copyToClipboard(http://localhost:8078/shared/r/{})">Copy</button>"#,
                        share.link
                    ),
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_custom_expires_at_time_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let _ = Recipe::create(&state.mm, 1, &a_complete_recipe_for_create()).await?;
            let expires_at = (chrono::Utc::now() + chrono::Duration::days(31)).naive_utc();

            let res = server
                .post(&base_uri(1))
                .form(&ShareRecipeForm {
                    datetime: Some(expires_at.to_string()),
                })
                .await;

            let share = get_first_shared_recipe(state).await;
            res.assert_status_ok();
            assert_html(
                res,
                vec![
                    &format!(
                        r#"<label><input class="input" type="url" value="http://localhost:8078/shared/r/{}" readonly="readonly"></label>"#,
                        share.link
                    ),
                    &format!(
                        r#"<button class="btn btn-neutral" id="copy-button" title="Copy to clipboard" onClick="copyToClipboard(http://localhost:8078/shared/r/{})">Copy</button>"#,
                        share.link
                    ),
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_invalid_expires_at_time_defaults_to_7_days_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let _ = Recipe::create(&state.mm, 1, &a_complete_recipe_for_create()).await?;
            let now = chrono::Utc::now().naive_utc();

            let res = server
                .post(&base_uri(1))
                .form(&ShareRecipeForm {
                    datetime: Some("hello".into()),
                })
                .await;

            let share = get_first_shared_recipe(state).await;
            res.assert_status_ok();
            pretty_assertions::assert_eq!(
                share.expires_at.signed_duration_since(now).num_days(),
                7
            );
            Ok(())
        }

        async fn get_first_shared_recipe(state: AppState) -> ShareRecipe {
            let mut conn = state
                .mm
                .pool
                .get()
                .await
                .expect("a connection from the pool");

            schema::shares_recipes::table
                .filter(
                    schema::shares_recipes::recipe_id
                        .eq(1)
                        .and(schema::shares_recipes::user_id.eq(1)),
                )
                .first::<ShareRecipe>(&mut conn)
                .await
                .expect("a share recipe must have been fetched")
        }
    }

    mod tests_recipes {
        use super::*;
        use models::recipe::test_utils::a_complete_recipe_for_create;

        use models::Recipe;
        use testing::utils::{assert_html, assert_must_be_logged_in, create_app_state};

        const BASE_URI: &str = "/recipes";

        #[tokio::test]
        async fn test_get_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, BASE_URI).await
        }

        #[tokio::test]
        async fn test_get_user_has_no_recipes_ok() -> Result<()> {
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
        async fn test_get_user_has_recipes_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
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
                    r##"<form class="w-72 flex md:w-96" hx-get="/recipes/search" hx-vals="{"page": 1}" hx-target="#list-recipes" hx-push-url="true" hx-trigger="submit, change target:.sort-option"><div><label class="input input-sm flex justify-between px-0 gap-2 z-20"><button id="search-shortcut" type="button" class="pl-2" popovertarget="search-help" _="on click toggle .hidden on #search-help"><svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 self-center" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg></button><input id="search_recipes" class="w-full" type="search" name="q" placeholder="Search for recipes..." value="""##,
                    r#"<button type="submit" class="px-2 btn btn-sm btn-primary"><svg class="w-4 h-4" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 20"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z"></path></svg><span class="sr-only">Search</span></button></label></div><div class="dropdown dropdown-left ml-1"><div tabindex="0" role="button" class="btn btn-sm p-1"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25H12"></path></svg></div><div tabindex="0" class="dropdown-content z-10 menu menu-sm p-2 shadow bg-base-200 w-36 sm:menu-md prose"><h4 class="text-center underline">Sort</h4><fieldset class="fieldset flex"><label class="label cursor-pointer text-inherit" for="sort-opt-default"><input id="sort-opt-default" type="radio" name="sort" class="radio radio-sm sort-option" value="default"><span class="ml-1">Default</span></label></fieldset><fieldset class="fieldset flex"><label class="label cursor-pointer text-inherit" for="sort-opt-a-z"><input id="sort-opt-a-z" type="radio" name="sort" class="radio radio-sm sort-option" value="a-z"><span class="ml-1">Name:<br/>A to Z</span></label></fieldset><fieldset class="fieldset flex"><label class="label cursor-pointer text-inherit" for="sort-opt-z-a"><input id="sort-opt-z-a" type="radio" name="sort" class="radio radio-sm sort-option" value="z-a"><span class="ml-1">Name:<br/>Z to A</span></label></fieldset><fieldset class="fieldset flex"><label class="label cursor-pointer text-inherit" for="sort-opt-new-old"><input id="sort-opt-new-old" type="radio" name="sort" class="radio radio-sm sort-option" value="new-old"><span class="ml-1">Date created:<br/>Newest to oldest</span></label></fieldset><fieldset class="fieldset flex"><label class="label cursor-pointer text-inherit" for="sort-opt-old-new"><input id="sort-opt-old-new" type="radio" name="sort" class="radio radio-sm sort-option" value="old-new"><span class="ml-1">Date created:<br/>Oldest to newest</span></label></fieldset><fieldset class="fieldset flex"><label class="label cursor-pointer text-inherit" for="sort-opt-random"><input id="sort-opt-random" type="radio" name="sort" class="radio radio-sm sort-option" value="random"><span class="ml-1">Random</span></label></fieldset></div></div></form>"#,
                    r#"<div id="list-recipes" class="min-h-[79vh]"><article class="grid gap-4 p-4 text-sm place-items-center grid-cols-1 sm:grid-cols-2 md:m-auto md:max-w-7xl md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 md:text-base">"#,
                    r#"<div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex">"#,
                    r##"<section class="card-side sm:card card-compact card-border bg-base-100 shadow-lg indicator w-full"><span class="hidden sm:block"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral indicator-item indicator-center" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "cat:dinner"}" _="on click put &quot;cat:dinner&quot; into #search_recipes.value">dinner</span></span><figure class="relative cursor-pointer" hx-get="/recipes/1" hx-target="#content" hx-push-url="true" hx-trigger="mousedown" hx-swap="innerHTML show:window:top transition:true"><img class="h-28 w-24 object-cover rounded-t-lg sm:h-40 sm:min-w-full sm:w-full" src="/data/images/Placeholders/placeholder.recipe.webp" alt="Image of the Best Chinese Kale0 recipe"><div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex"><p class="p-2 text-sm">This is the most delicious recipe!</p></div></figure><div class="card-body justify-between"><h2 class="sm:font-semibold sm:w-[25ch] sm:break-words sm:min-h-14">Best Chinese Kale0</h2><div class="sm:max-h-14 sm:overflow-y-auto sm:content-end sm:min-h-14"><div class="flex flex-col flex-wrap overflow-x-auto max-h-12 pb-2 sm:pb-0 sm:max-h-none sm:flex-auto sm:flex-row"><span class="sm:hidden"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "cat:dinner"}" _="on click put &quot;cat:dinner&quot; into #search_recipes.value">dinner</span></span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "tag":tofu}" _="on click put &quot;tag:tofu&quot; into #search-recipes.value">tofu</span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "tag":vegetarian}" _="on click put &quot;tag:vegetarian&quot; into #search-recipes.value">vegetarian</span></div></div><div class="card-actions flex-col-reverse h-fit"><button class="btn btn-block btn-xs btn-outline sm:btn-sm" hx-get="/recipes/1" hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true">View</button></div></div></section>"##,
                    r##"<section class="card-side sm:card card-compact card-border bg-base-100 shadow-lg indicator w-full"><span class="hidden sm:block"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral indicator-item indicator-center" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "cat:dinner"}" _="on click put &quot;cat:dinner&quot; into #search_recipes.value">dinner</span></span><figure class="relative cursor-pointer" hx-get="/recipes/2" hx-target="#content" hx-push-url="true" hx-trigger="mousedown" hx-swap="innerHTML show:window:top transition:true"><img class="h-28 w-24 object-cover rounded-t-lg sm:h-40 sm:min-w-full sm:w-full" src="/data/images/Placeholders/placeholder.recipe.webp" alt="Image of the Best Chinese Kale1 recipe"><div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex"><p class="p-2 text-sm">This is the most delicious recipe!</p></div></figure><div class="card-body justify-between"><h2 class="sm:font-semibold sm:w-[25ch] sm:break-words sm:min-h-14">Best Chinese Kale1</h2><div class="sm:max-h-14 sm:overflow-y-auto sm:content-end sm:min-h-14"><div class="flex flex-col flex-wrap overflow-x-auto max-h-12 pb-2 sm:pb-0 sm:max-h-none sm:flex-auto sm:flex-row"><span class="sm:hidden"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "cat:dinner"}" _="on click put &quot;cat:dinner&quot; into #search_recipes.value">dinner</span></span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "tag":tofu}" _="on click put &quot;tag:tofu&quot; into #search-recipes.value">tofu</span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "tag":vegetarian}" _="on click put &quot;tag:vegetarian&quot; into #search-recipes.value">vegetarian</span></div></div><div class="card-actions flex-col-reverse h-fit"><button class="btn btn-block btn-xs btn-outline sm:btn-sm" hx-get="/recipes/2" hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true">View</button></div></div></section>"##,
                    r##"<section class="card-side sm:card card-compact card-border bg-base-100 shadow-lg indicator w-full"><span class="hidden sm:block"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral indicator-item indicator-center" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "cat:dinner"}" _="on click put &quot;cat:dinner&quot; into #search_recipes.value">dinner</span></span><figure class="relative cursor-pointer" hx-get="/recipes/3" hx-target="#content" hx-push-url="true" hx-trigger="mousedown" hx-swap="innerHTML show:window:top transition:true"><img class="h-28 w-24 object-cover rounded-t-lg sm:h-40 sm:min-w-full sm:w-full" src="/data/images/Placeholders/placeholder.recipe.webp" alt="Image of the Best Chinese Kale2 recipe"><div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex"><p class="p-2 text-sm">This is the most delicious recipe!</p></div></figure><div class="card-body justify-between"><h2 class="sm:font-semibold sm:w-[25ch] sm:break-words sm:min-h-14">Best Chinese Kale2</h2><div class="sm:max-h-14 sm:overflow-y-auto sm:content-end sm:min-h-14"><div class="flex flex-col flex-wrap overflow-x-auto max-h-12 pb-2 sm:pb-0 sm:max-h-none sm:flex-auto sm:flex-row"><span class="sm:hidden"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "cat:dinner"}" _="on click put &quot;cat:dinner&quot; into #search_recipes.value">dinner</span></span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "tag":tofu}" _="on click put &quot;tag:tofu&quot; into #search-recipes.value">tofu</span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "tag":vegetarian}" _="on click put &quot;tag:vegetarian&quot; into #search-recipes.value">vegetarian</span></div></div><div class="card-actions flex-col-reverse h-fit"><button class="btn btn-block btn-xs btn-outline sm:btn-sm" hx-get="/recipes/3" hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true">View</button></div></div></section>"##,
                    r#"><footer id="pagination" class="footer footer-center bg-base-200 pb-12 p-2 md:pb-2 text-base-content gap-2" style="grid-auto-flow: row;" onload="updateAddCookbookUrl(1)"><div class="join gap-0"><button class="join-item btn btn-disabled w-12" title="Previous page" aria-label="Previous page">‹</button><button class="join-item btn btn-active w-12" aria-current="page" aria-label="Page 1, current page">1</button><button class="join-item btn btn-disabled w-12" title="Next page" aria-label="Next page">›</button></div><div class="text-center mt-2"><p class="text-sm text-base-content/70">Showing <span class="font-semibold text-base-content">1</span>-<span class="font-semibold text-base-content">3</span> of <span id="search-count" class="font-medium">3</span> results</p></div></footer>"#,
                ],
            );
            Ok(())
        }
    }

    mod tests_recipe {
        use super::*;
        use models::recipe::test_utils::a_complete_recipe_for_create;

        use axum::http::{HeaderValue, StatusCode};
        use axum_test::TestResponse;
        use uuid::Uuid;

        use models::Recipe;
        use models::recipe::{RecipeForCreate, VideoForCreate};
        use testing::utils::{
            assert_html, assert_must_be_logged_in, assert_ws_message, build_server_ws,
            create_app_state,
        };

        fn base_uri(recipe_id: i64) -> String {
            format!("/recipes/{recipe_id}")
        }

        // region GET /recipes/{id}
        #[tokio::test]
        async fn test_get_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, &base_uri(1)).await
        }

        #[tokio::test]
        async fn test_get_recipe_exists_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
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
            let state = create_app_state(config).await;
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
            let state = create_app_state(config).await;
            let mut recipe = a_complete_recipe_for_create();
            recipe.videos.clear();
            recipe.images = vec![];
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
            let state = create_app_state(config).await;
            let mut recipe = a_complete_recipe_for_create();
            recipe.videos.clear();
            let img1 = Uuid::new_v4();
            recipe.images = vec![img1];
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
            let state = create_app_state(config).await;
            let mut recipe = a_complete_recipe_for_create();
            recipe.videos.clear();
            recipe.images = vec![Uuid::nil(), Uuid::nil()];
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
            let state = create_app_state(config).await;
            let mut recipe = a_complete_recipe_for_create();
            recipe.videos = vec![VideoForCreate {
                video: Uuid::new_v4(),
                duration: None,
                content_url: Some("https://example.com/embed/yg8FG4".into()),
                embed_url: None,
            }];
            recipe.images = vec![];
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
            let state = create_app_state(config).await;
            let mut recipe = a_complete_recipe_for_create();
            recipe.videos = vec![
                VideoForCreate {
                    video: Uuid::new_v4(),
                    duration: None,
                    content_url: Some("https://example.com/embed/yg8FG4".into()),
                    embed_url: None,
                },
                VideoForCreate {
                    video: Uuid::new_v4(),
                    duration: None,
                    content_url: None,
                    embed_url: Some("https://example.com/embed/yg8FG4".into()),
                },
            ];
            recipe.images = vec![];
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
            let state = create_app_state(config).await;
            let mut recipe = a_complete_recipe_for_create();
            recipe.videos = vec![
                VideoForCreate {
                    video: Uuid::nil(),
                    duration: None,
                    content_url: Some("https://example.com/embed/yg8FG4".into()),
                    embed_url: None,
                },
                VideoForCreate {
                    video: Uuid::nil(),
                    duration: None,
                    content_url: None,
                    embed_url: Some("https://example.com/embed/yg8FG4".into()),
                },
            ];
            recipe.images = vec![Uuid::nil(), Uuid::nil()];
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
            assert_must_be_logged_in(Method::DELETE, &base_uri(1)).await
        }

        #[tokio::test]
        async fn test_delete_recipe_does_not_exist() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;

            let res = server.delete(&base_uri(1)).await;

            res.assert_status_internal_server_error();
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Recipe could not be deleted.","status":"alert-error","title":"Operation Failed"}}"#).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_delete_recipe_exists_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
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
                    r#"<div class="badge badge-sm badge-neutral m-1 flex-auto">tofu</div><div class="badge badge-sm badge-neutral m-1 flex-auto">vegetarian</div>"#,
                    r##"<fieldset class="fieldset"><legend>Servings</legend><input id="yield" type="number" min="1" name="yield" value="4" class="input" hx-get="/recipes/1/scale" hx-trigger="input" hx-target="#ingredients-instructions-container"></fieldset>"##,
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

    mod tests_recipes_add_import {
        use super::*;

        use axum_test::http::StatusCode;

        use models::Recipe;
        use testing::utils::{
            HIDDEN_WS_NOTIFICATION, assert_ws_message, build_server_ws, create_app_state,
            open_test_file,
        };

        const BASE_URI: &str = "/recipes/add/import";

        #[tokio::test]
        async fn test_must_be_logged_in() -> Result<()> {
            assert_must_be_logged_in(Method::POST, BASE_URI).await
        }

        #[tokio::test]
        async fn test_post_payload_too_large() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;
            let large_payload = "x".repeat(51 * 1024 * 1024);

            let res = server
                .post(BASE_URI)
                .multipart(
                    MultipartForm::new()
                        .add_part("app", Part::text("mealmaster"))
                        .add_part(
                            "file",
                            Part::bytes(large_payload.into_bytes())
                                .file_name("cookmate1.mcb")
                                .mime_type("application/octet-stream"),
                        ),
                )
                .await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_post_error_parsing_files() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            let file = open_test_file("integrations/kalorio1.txt");

            let res = server
                .post(BASE_URI)
                .multipart(
                    MultipartForm::new()
                        .add_part("app", Part::text("mealmaster"))
                        .add_part(
                            "file",
                            Part::bytes(file.into_inner())
                                .file_name("kalorio1.txt")
                                .mime_type("application/octet-stream"),
                        ),
                )
                .await;

            res.assert_status(StatusCode::ACCEPTED);
            assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Parsing recipes...</p><div id="export-progress"><progress max="100" value="1.00"></progress></div></div></div>"#).await;
            assert_ws_message(&mut ws_server, HIDDEN_WS_NOTIFICATION).await;
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"An error occurred while parsing the recipes. Please check the logs.","status":"alert-error","title":"Operation Failed"}}"#).await;
            let state = create_app_state(config).await;
            pretty_assertions::assert_eq!(Recipe::count(&state.mm, 1).await?, 0);
            Ok(())
        }

        #[tokio::test]
        async fn test_post_valid_request() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            let file = open_test_file("integrations/kalorio1.txt");

            let res = server
                .post(BASE_URI)
                .multipart(
                    MultipartForm::new()
                        .add_part("app", Part::text("kalorio"))
                        .add_part(
                            "file",
                            Part::bytes(file.into_inner())
                                .file_name("kalorio1.txt")
                                .mime_type("application/octet-stream"),
                        ),
                )
                .await;

            res.assert_status(StatusCode::ACCEPTED);
            assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Parsing recipes...</p><div id="export-progress"><progress max="100" value="1.00"></progress></div></div></div>"#).await;
            assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Saving recipes</p><div id="export-progress"><progress max="100" value="33.33"></progress></div></div></div>"#).await;
            assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Saving recipes</p><div id="export-progress"><progress max="100" value="66.67"></progress></div></div></div>"#).await;
            assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Saving recipes</p><div id="export-progress"><progress max="100" value="100.00"></progress></div></div></div>"#).await;
            assert_ws_message(&mut ws_server, HIDDEN_WS_NOTIFICATION).await;
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","action":"View /reports?view=latest","message":"Imported 3 recipes. Skipped 0.","status":"alert-info","title":"Operation Successful"}}"#).await;
            let state = create_app_state(config).await;
            pretty_assertions::assert_eq!(Recipe::count(&state.mm, 1).await?, 3);
            Ok(())
        }
    }

    mod tests_recipe_add_manual {
        use super::*;

        use axum::http::HeaderValue;
        use axum_test::TestResponse;
        use chrono::Duration;
        use uuid::Uuid;

        use models::recipe::{
            Nutrition, NutritionForCreate, RecipeForCreate, Times, TimesForCreate, ToolForCreate,
            ToolRecipe, VideoForCreate,
        };
        use models::{Recipe, RecipeDetails};
        use recipe_schema::Sections;
        use testing::utils::{assert_must_be_logged_in, create_app_state};

        const BASE_URI: &str = "/recipes/add/manual";

        #[tokio::test]
        async fn test_get_must_be_logged_in_ok() -> Result<()> {
            let _ = assert_must_be_logged_in(Method::GET, BASE_URI).await;
            assert_must_be_logged_in(Method::POST, BASE_URI).await
        }

        #[tokio::test]
        async fn test_get_logged_in_htmx_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let mut server = build_server_logged_in(config).await?;
            server.add_header(
                axum_htmx::headers::HX_REQUEST,
                HeaderValue::from_static("true"),
            );

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            assert_recipe_form(res);
            Ok(())
        }

        #[tokio::test]
        async fn test_get_logged_in_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            assert_recipe_form(res);
            Ok(())
        }

        #[tokio::test]
        async fn test_post_missing_required_title_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;
            let recipe = RecipeForCreate {
                ingredients: Sections::from([("".into(), vec!["1 apple".to_string()])]),
                instructions: Sections::from([("".into(), vec!["Mix the apples".to_string()])]),
                ..Default::default()
            };
            let form = create_form(&recipe);

            let res = server.post(BASE_URI).multipart(form).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_post_missing_required_ingredients_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;
            let recipe = RecipeForCreate {
                name: "Best Chinese Kale".to_string(),
                instructions: Sections::from([("".into(), vec!["Mix the apples".to_string()])]),
                ..Default::default()
            };
            let form = create_form(&recipe);

            let res = server.post(BASE_URI).multipart(form).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_post_missing_required_instructions_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;
            let recipe = RecipeForCreate {
                name: "Best Chinese Kale".to_string(),
                ingredients: Sections::from([("".into(), vec!["8 apples".to_string()])]),
                ..Default::default()
            };
            let form = create_form(&recipe);

            let res = server.post(BASE_URI).multipart(form).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_post_missing_fields_defaults_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let recipe = RecipeForCreate {
                name: "Best Chinese Kale".to_string(),
                instructions: Sections::from([("".into(), vec!["Mix the apples".to_string()])]),
                ingredients: Sections::from([("".into(), vec!["8 apples".to_string()])]),
                ..Default::default()
            };
            let form = create_form(&recipe);

            let res = server.post(BASE_URI).multipart(form).await;

            res.assert_status_see_other();
            let state = create_app_state(config.clone()).await;
            let got = Recipe::get(&state.mm, 1, 1).await?;
            pretty_assertions::assert_eq!(got.category, "uncategorized");
            pretty_assertions::assert_eq!(got.recipe.yield_, 1);
            Ok(())
        }

        #[tokio::test]
        async fn test_post_can_only_be_one_category_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let recipe = RecipeForCreate {
                name: "Best Chinese Kale".to_string(),
                instructions: Sections::from([("".into(), vec!["Mix the apples".to_string()])]),
                ingredients: Sections::from([("".into(), vec!["8 apples".to_string()])]),
                category: Some("breakfast,dinner".into()),
                ..Default::default()
            };
            let form = create_form(&recipe);

            let res = server.post(BASE_URI).multipart(form).await;

            res.assert_status_see_other();
            let state = create_app_state(config.clone()).await;
            let got = Recipe::get(&state.mm, 1, 1).await?;
            pretty_assertions::assert_eq!(got.category, "breakfast");
            Ok(())
        }

        #[tokio::test]
        async fn test_post_duplicates_are_removed_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let recipe = RecipeForCreate {
                name: "Best Chinese Kale".to_string(),
                instructions: Sections::from([(
                    "".into(),
                    vec![
                        "Mix the apples".to_string(),
                        "Eat".to_string(),
                        "Mix the apples".to_string(),
                    ],
                )]),
                ingredients: Sections::from([(
                    "".into(),
                    vec![
                        "8 apples".to_string(),
                        "4 oranges".to_string(),
                        "8 apples".to_string(),
                    ],
                )]),
                keywords: vec!["drinks".into(), "vodka".into(), "drinks".into()],
                tools: vec![
                    ToolForCreate {
                        quantity: 1,
                        name: "1 wok".into(),
                    },
                    ToolForCreate {
                        quantity: 1,
                        name: "1 wok".into(),
                    },
                ],
                ..Default::default()
            };
            let form = create_form(&recipe);

            let res = server.post(BASE_URI).multipart(form).await;

            res.assert_status_see_other();
            let state = create_app_state(config.clone()).await;
            let got = Recipe::get(&state.mm, 1, 1).await?;
            pretty_assertions::assert_eq!(
                got.keywords,
                vec!["drinks".to_string(), "vodka".to_string()]
            );
            pretty_assertions::assert_eq!(
                got.ingredients,
                Sections::from([(
                    "".into(),
                    vec!["8 apples".to_string(), "4 oranges".to_string()]
                )])
            );
            pretty_assertions::assert_eq!(
                got.instructions,
                Sections::from([(
                    "".into(),
                    vec!["Mix the apples".to_string(), "Eat".to_string()]
                )])
            );
            pretty_assertions::assert_eq!(
                got.tools,
                vec![ToolRecipe {
                    name: "wok".into(),
                    tool_order: 1,
                    quantity: 1,
                }]
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_post_subcategories_are_possible() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let recipe = RecipeForCreate {
                name: "Best Chinese Kale".to_string(),
                instructions: Sections::from([("".into(), vec!["Mix the apples".to_string()])]),
                ingredients: Sections::from([("".into(), vec!["8 apples".to_string()])]),
                category: Some("drinks:vodka".into()),
                ..Default::default()
            };
            let form = create_form(&recipe);

            let res = server.post(BASE_URI).multipart(form).await;

            res.assert_status_see_other();
            let state = create_app_state(config.clone()).await;
            let got = Recipe::get(&state.mm, 1, 1).await?;
            pretty_assertions::assert_eq!(got.category, "drinks:vodka");
            Ok(())
        }

        #[tokio::test]
        async fn test_post_submit_recipe_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let recipe = RecipeForCreate {
                name: "Best Chinese Kale".into(),
                description: Some("Your mouth will drool like never before".into()),
                images: vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()],
                measurement_system_id: 2,
                yield_: Some(6),
                source: Some("My mother's maple syrup recipes cookbook".into()),
                videos: vec![VideoForCreate {
                    video: Uuid::new_v4(),
                    duration: Some(Duration::hours(1)),
                    content_url: Some("https://www.youtube.com/watch?v=1".into()),
                    embed_url: Some("https://www.youtube.com/embed/embed".into()),
                }],
                category: Some("dinner".into()),
                instructions: Sections::from([
                    (
                        "Prepare".into(),
                        vec!["Mix the apples".into(), "Mix the blueberries".into()],
                    ),
                    (
                        "Execution".into(),
                        vec!["Add whip cream and whisk the fruits until smooth".into()],
                    ),
                ]),
                keywords: vec!["fruits".into(), "strawberries".into(), "healthy".into()],
                nutrition: Some(NutritionForCreate {
                    calories_kcal: Some(1),
                    total_carbohydrates: Some(2),
                    sugars_g: Some(3),
                    protein_g: Some(4),
                    total_fat_g: Some(5),
                    saturated_fat_g: Some(6),
                    unsaturated_fat_g: Some(7),
                    cholesterol_mg: Some(8),
                    sodium_mg: Some(9),
                    fiber_g: Some(10),
                    trans_fat_g: Some(11),
                    serving_size: Some("12".into()),
                }),
                times: Some(TimesForCreate {
                    prep_seconds: 3600,
                    cook_seconds: 72000,
                }),
                ingredients: Sections::from([
                    (
                        "Prepare".into(),
                        vec!["8 apples".into(), "5 lbs blueberries".into()],
                    ),
                    ("Execution".into(), vec!["200g 30% whip cream".into()]),
                ]),
                cuisine: Some("japanese".into()),
                tools: vec![
                    ToolForCreate {
                        name: "large bowl".into(),
                        quantity: 1,
                    },
                    ToolForCreate {
                        name: "whisk".into(),
                        quantity: 2,
                    },
                ],
            };
            let form = create_form(&recipe);

            let res = server.post(BASE_URI).multipart(form).await;

            res.assert_status_see_other();
            let state = create_app_state(config.clone()).await;
            let got = Recipe::get(&state.mm, 1, 1).await?;
            let times = recipe.times.expect("Should have times");
            pretty_assertions::assert_eq!(
                got,
                RecipeDetails {
                    recipe: Recipe {
                        id: 1,
                        name: recipe.name,
                        description: recipe.description,
                        image: Some(got.recipe.image.expect("A main image")),
                        yield_: 6,
                        language: "eng".into(),
                        measurement_system_id: 2,
                        source: recipe.source,
                        created_at: got.recipe.created_at,
                        updated_at: got.recipe.updated_at,
                        user_id: 1,
                    },
                    additional_images: got.additional_images.clone(),
                    category: recipe.category.expect("Should have category"),
                    cuisine: recipe.cuisine,
                    ingredients: Sections::from([(
                        "".into(),
                        recipe
                            .ingredients
                            .iter()
                            .flat_map(|(_, xs)| xs.iter().cloned()) // Flatten ingredients
                            .collect::<Vec<String>>()
                    )]),
                    instructions: Sections::from([(
                        "".into(),
                        recipe
                            .instructions
                            .iter()
                            .flat_map(|(_, xs)| xs.iter().cloned()) // Flatten ingredients
                            .collect::<Vec<String>>()
                    )]),
                    keywords: vec!["fruits".into(), "healthy".into(), "strawberries".into()],
                    nutrition: Some(Nutrition {
                        id: 1,
                        recipe_id: 1,
                        calories_kcal: Some(1),
                        total_carbohydrates: Some(2),
                        sugars_g: Some(3),
                        protein_g: Some(4),
                        total_fat_g: Some(5),
                        saturated_fat_g: Some(6),
                        unsaturated_fat_g: Some(7),
                        cholesterol_mg: Some(8),
                        sodium_mg: Some(9),
                        fiber_g: Some(10),
                        trans_fat_g: Some(11),
                        serving_size: Some("12".into()),
                    }),
                    times: Times {
                        id: 1,
                        recipe_id: 1,
                        prep_seconds: times.prep_seconds,
                        cook_seconds: times.cook_seconds,
                        total_seconds: times.prep_seconds + times.cook_seconds,
                    },
                    tools: recipe
                        .tools
                        .into_iter()
                        .enumerate()
                        .map(|(idx, tool_c)| ToolRecipe {
                            name: tool_c.name,
                            quantity: tool_c.quantity,
                            tool_order: (idx + 1) as i16,
                        })
                        .collect(),
                    videos: vec![],
                }
            );
            Ok(())
        }

        fn assert_recipe_form(res: TestResponse) {
            let expected = [
                r#"<title hx-swap-oob="true">Add Recipe Manually | Recipya</title>"#,
                r##"<form class="card-body" style="padding: 0" enctype="multipart/form-data" hx-post="/recipes/add/manual" hx-indicator="#fullscreen-loader">"##,
                r#"<input required type="text" name="title" placeholder="Title of the recipe*" autocomplete="off" class="input w-full text-center rounded-t-lg rounded-b-none bg-base-200">"#,
                r#"<img src="" alt="" class="object-cover mb-2 w-full max-h-[39rem]"><span class="grid gap-1 max-w-sm" style="margin: auto auto 0.25rem;"><div class="mr-1"><input type="file" accept="image/*,video/*" name="media" class="file-input file-input-sm file-input-bordered w-full max-w-sm""#,
                r#"<input id="servings" type="number" min="1" name="yield" value="1" class="input input-sm w-11/12">"#,
                r#"<input id="category" type="text" list="categories" name="category" class="input input-sm w-11/12" placeholder="Breakfast" autocomplete="off" value=""><datalist id="categories"><option>uncategorized</option><option>appetizers</option><option>bread</option><option>breakfasts</option><option>condiments</option><option>dessert</option><option>lunch</option><option>main dish</option><option>salad</option><option>side dish</option><option>snacks</option><option>soups</option><option>stews</option></datalist>"#,
                r#"<textarea name="description" placeholder="This Thai curry chicken will make you drool." class="textarea w-full h-full resize-none"></textarea>"#,
                r#"<div class="flex justify-self-center items-center gap-1 cursor-default" title="Cooking time"><span data-tip="Cook time" class="tooltip tooltip-left"><svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="24px" height="23px" viewBox="0 0 24 23" version="1.1"><g id="surface1"><path style=" stroke:none;fill-rule:nonzero;fill:rgb(62.745098%,64.705882%,65.882353%);fill-opacity:1;" d="M 4.636719 10.984375 L 4.636719 20.417969 C 4.636719 21.007812 5.078125 21.527344 5.667969 21.527344 L 18.257812 21.527344 C 18.847656 21.527344 19.363281 21.007812 19.363281 20.417969 L 19.363281 10.984375 Z M 4.636719 10.984375 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(76.862745%,76.862745%,76.862745%);fill-opacity:1;" d="M 19.289062 9.953125 C 18.992188 8.699219 17.964844 7.8125 16.710938 7.8125 L 7.214844 7.8125 C 5.964844 7.8125 4.933594 8.699219 4.710938 9.953125 Z M 19.289062 9.953125 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(54.509804%,69.411765%,81.960784%);fill-opacity:1;" d="M 16.710938 7.8125 L 13.914062 7.8125 C 14.945312 7.8125 15.828125 8.476562 16.269531 9.363281 C 16.417969 9.730469 16.785156 9.953125 17.226562 9.953125 L 19.289062 9.953125 C 18.992188 8.699219 17.964844 7.8125 16.710938 7.8125 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(0.392157%,26.666667%,38.823529%);fill-opacity:1;" d="M 22.160156 9.953125 L 20.320312 9.953125 C 20.097656 8.183594 18.550781 6.78125 16.710938 6.78125 L 15.535156 6.78125 L 15.3125 5.898438 C 15.09375 5.160156 14.503906 4.644531 13.765625 4.644531 L 11.191406 4.644531 C 10.453125 4.644531 9.792969 5.160156 9.644531 5.898438 L 9.421875 6.78125 L 7.214844 6.78125 C 5.449219 6.78125 3.902344 8.183594 3.605469 9.953125 L 1.765625 9.953125 C 1.03125 9.953125 0.441406 10.542969 0.441406 11.277344 C 0.441406 11.5 0.441406 11.722656 0.589844 11.941406 L 1.25 13.269531 C 1.546875 13.785156 2.0625 14.152344 2.648438 14.152344 L 3.605469 14.152344 L 3.605469 20.417969 C 3.605469 21.597656 4.492188 22.558594 5.667969 22.558594 L 18.257812 22.558594 C 19.4375 22.558594 20.394531 21.597656 20.394531 20.417969 L 20.394531 14.152344 L 21.351562 14.152344 C 21.9375 14.152344 22.453125 13.859375 22.75 13.269531 L 23.410156 11.867188 C 23.558594 11.648438 23.558594 11.5 23.558594 11.277344 C 23.558594 10.542969 22.894531 9.953125 22.160156 9.953125 M 3.605469 13.121094 L 2.648438 13.121094 C 2.429688 13.121094 2.28125 12.972656 2.136719 12.753906 L 1.472656 11.5 L 1.472656 11.277344 C 1.472656 11.132812 1.621094 10.984375 1.765625 10.984375 L 3.605469 10.984375 Z M 10.75 6.117188 C 10.75 5.972656 10.96875 5.75 11.265625 5.75 L 13.839844 5.75 C 14.0625 5.75 14.28125 5.898438 14.355469 6.117188 L 14.503906 6.78125 L 10.601562 6.78125 Z M 7.214844 7.8125 L 16.710938 7.8125 C 17.964844 7.8125 18.992188 8.699219 19.289062 9.953125 L 4.710938 9.953125 C 4.933594 8.699219 5.964844 7.8125 7.214844 7.8125 M 19.363281 13.636719 L 19.363281 20.417969 C 19.363281 21.007812 18.847656 21.527344 18.257812 21.527344 L 5.667969 21.527344 C 5.078125 21.527344 4.636719 21.007812 4.636719 20.417969 L 4.636719 10.984375 L 19.363281 10.984375 Z M 22.453125 11.5 L 21.71875 12.828125 C 21.644531 12.972656 21.496094 13.121094 21.277344 13.121094 L 20.394531 13.121094 L 20.394531 11.058594 L 22.160156 11.058594 C 22.308594 11.058594 22.453125 11.207031 22.453125 11.351562 L 22.453125 11.5 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(92.54902%,94.117647%,97.647059%);fill-opacity:1;" d="M 7.804688 17.324219 C 8.089844 17.324219 8.320312 17.554688 8.320312 17.839844 C 8.320312 18.125 8.089844 18.355469 7.804688 18.355469 C 7.519531 18.355469 7.289062 18.125 7.289062 17.839844 C 7.289062 17.554688 7.519531 17.324219 7.804688 17.324219 M 7.804688 16.808594 C 7.4375 16.808594 7.214844 16.585938 7.214844 16.21875 L 7.214844 13.121094 C 7.214844 12.753906 7.4375 12.53125 7.804688 12.53125 C 8.097656 12.53125 8.320312 12.753906 8.320312 13.121094 L 8.320312 16.21875 C 8.320312 16.585938 8.097656 16.808594 7.804688 16.808594 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(54.509804%,69.411765%,81.960784%);fill-opacity:1;" d="M 16.195312 12.015625 L 16.195312 19.902344 C 16.195312 20.492188 15.679688 21.007812 15.09375 21.007812 L 6.699219 21.007812 C 6.183594 21.007812 5.667969 20.492188 5.667969 19.902344 L 5.667969 12.015625 C 5.667969 11.5 5.226562 10.984375 4.636719 10.984375 L 4.636719 20.417969 C 4.636719 21.007812 5.078125 21.527344 5.667969 21.527344 L 18.257812 21.527344 C 18.847656 21.527344 19.363281 21.007812 19.363281 20.417969 L 19.363281 10.984375 L 17.226562 10.984375 C 16.636719 10.984375 16.195312 11.5 16.195312 12.015625 M 8.246094 7.8125 L 7.214844 7.8125 C 5.964844 7.8125 4.933594 8.699219 4.710938 9.953125 L 4.933594 9.953125 C 5.375 9.953125 5.742188 9.730469 5.890625 9.363281 C 6.332031 8.476562 7.214844 7.8125 8.246094 7.8125 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(0.392157%,26.666667%,38.823529%);fill-opacity:1;" d="M 18.773438 5.75 C 18.625 5.75 18.480469 5.675781 18.40625 5.527344 C 18.183594 5.308594 18.257812 4.9375 18.40625 4.792969 C 18.699219 4.644531 18.773438 4.421875 18.773438 4.128906 C 18.773438 3.90625 18.699219 3.6875 18.480469 3.539062 C 18.257812 3.316406 18.257812 3.023438 18.40625 2.800781 C 18.625 2.582031 18.992188 2.507812 19.140625 2.726562 C 19.582031 3.097656 19.878906 3.613281 19.878906 4.128906 C 19.878906 4.71875 19.582031 5.234375 19.140625 5.601562 L 18.773438 5.75 M 18.773438 1.03125 C 19.058594 1.03125 19.289062 1.261719 19.289062 1.546875 C 19.289062 1.832031 19.058594 2.0625 18.773438 2.0625 C 18.488281 2.0625 18.257812 1.832031 18.257812 1.546875 C 18.257812 1.261719 18.488281 1.03125 18.773438 1.03125 M 16.710938 5.75 L 16.269531 5.527344 C 16.050781 5.308594 16.121094 4.9375 16.34375 4.792969 C 16.5625 4.644531 16.710938 4.421875 16.710938 4.128906 C 16.710938 3.90625 16.5625 3.6875 16.417969 3.539062 C 15.902344 3.097656 15.679688 2.652344 15.679688 2.0625 C 15.679688 1.472656 15.902344 1.03125 16.417969 0.589844 C 16.5625 0.367188 16.933594 0.441406 17.152344 0.664062 C 17.300781 0.8125 17.300781 1.179688 17.078125 1.402344 C 16.785156 1.546875 16.710938 1.769531 16.710938 2.0625 C 16.710938 2.285156 16.785156 2.507812 17.007812 2.652344 C 17.519531 3.097656 17.742188 3.613281 17.742188 4.128906 C 17.742188 4.71875 17.519531 5.234375 17.007812 5.601562 L 16.710938 5.75 "></path></g></svg></span><label><input type="text" name="time-cook" value="00:30:00" class="input input-xs max-w-24 html-duration-picker"></label></div>"#,
                r#"<div class="flex justify-self-center items-center gap-1 cursor-default" title="Prep time"><span data-tip="Prep time" class="tooltip tooltip-left"><svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="24px" height="14px" viewBox="0 0 23 14" version="1.1"><defs><linearGradient id="linear0" gradientUnits="userSpaceOnUse" x1="-125.300003" y1="85.900002" x2="-64.599998" y2="85.900002" gradientTransform="matrix(0.000000000000000013,-0.225806,0.219048,0.000000000000000014,-7.447619,-14.451613)"><stop offset="0.1" style="stop-color:rgb(67.058824%,23.921569%,8.235294%);stop-opacity:1;"></stop><stop offset="0.5" style="stop-color:rgb(78.431373%,51.372549%,30.588235%);stop-opacity:1;"></stop><stop offset="0.8" style="stop-color:rgb(90.196078%,77.254902%,53.333333%);stop-opacity:1;"></stop><stop offset="1" style="stop-color:rgb(94.509804%,87.45098%,62.352941%);stop-opacity:1;"></stop></linearGradient></defs><g id="surface1"><path style=" stroke:none;fill-rule:evenodd;fill:rgb(95.294118%,82.745099%,64.705884%);fill-opacity:1;" d="M 0 8.128906 L 0.21875 6.324219 C 0.4375 5.644531 0.65625 5.195312 1.3125 4.96875 L 8.542969 2.484375 L 8.542969 1.804688 L 8.980469 0.675781 L 9.855469 0.453125 L 10.734375 0.453125 L 10.953125 0.675781 L 12.265625 1.128906 C 13.003906 1 13.761719 1.078125 14.457031 1.355469 C 15.125 1.453125 15.785156 1.601562 16.429688 1.804688 L 17.523438 1.804688 L 18.617188 0.902344 L 20.589844 0.453125 C 21.246094 0.675781 21.6875 0.902344 21.90625 1.355469 C 22.34375 1.804688 22.5625 2.710938 22.34375 4.066406 L 22.125 4.515625 C 22.5625 4.742188 22.78125 5.195312 22.78125 5.644531 L 22.78125 7.675781 L 22.125 8.804688 L 21.027344 9.484375 L 17.304688 11.289062 L 16.210938 11.742188 L 12.921875 13.324219 L 12.046875 13.773438 L 11.390625 14 L 10.078125 14 L 8.542969 13.546875 C 6.269531 12.441406 4.007812 11.308594 1.753906 10.160156 L 0.65625 9.257812 C 0.21875 9.03125 0 8.582031 0 8.128906 Z M 0 8.128906 "></path><path style=" stroke:none;fill-rule:evenodd;fill:url(#linear0);" d="M 1.3125 4.742188 L 8.542969 2.03125 L 8.542969 1.582031 L 8.980469 0.453125 C 9.199219 0.226562 9.636719 0 9.855469 0.226562 L 10.953125 0.226562 L 12.265625 0.902344 C 13.003906 0.773438 13.761719 0.851562 14.457031 1.128906 L 15.769531 1.355469 C 16.308594 1.65625 16.933594 1.738281 17.523438 1.582031 L 17.523438 1.355469 C 17.742188 1.128906 18.179688 0.675781 18.617188 0.675781 C 19.277344 0.226562 19.933594 0.226562 20.589844 0.226562 C 21.027344 0.453125 21.6875 0.675781 21.90625 1.128906 C 22.34375 1.582031 22.5625 2.484375 22.34375 3.839844 L 22.125 4.289062 C 22.5625 4.515625 22.78125 4.96875 22.78125 5.417969 L 22.78125 6.546875 L 22.5625 7.453125 L 22.125 8.582031 L 21.027344 9.257812 L 17.085938 11.289062 L 16.210938 11.515625 L 12.921875 13.097656 L 12.046875 13.546875 L 11.390625 13.773438 L 9.855469 13.773438 L 8.324219 13.324219 C 6.121094 12.21875 3.929688 11.089844 1.753906 9.933594 L 1.535156 9.933594 L 0.4375 9.03125 L 0 7.902344 C 0 7.292969 0.0742188 6.6875 0.21875 6.097656 C 0.21875 5.417969 0.65625 4.96875 1.3125 4.742188 Z M 1.3125 4.742188 "></path><path style="fill:none;stroke-width:0.3;stroke-linecap:butt;stroke-linejoin:miter;stroke:rgb(95.294118%,82.745099%,64.705884%);stroke-opacity:1;stroke-miterlimit:4;" d="M 5.991848 21.001116 L 39.999151 8.995536 L 39.00051 6.00279 L 40.997792 2.006696 L 46.008832 0 L 47.007473 0 L 49.004755 1.003348 L 50.003397 1.003348 L 55.995245 3.996094 C 59.579654 2.923549 63.413723 2.923549 66.998132 3.996094 L 73.007812 6.00279 C 75.272588 6.763951 77.733526 6.763951 79.998302 6.00279 L 84.991508 2.006696 C 88.005265 1.003348 91.001189 0 93.997113 1.003348 C 96.993037 1.003348 99.008152 2.006696 101.005435 3.996094 C 103.002717 6.00279 103.002717 9.998884 102.004076 16.001674 L 102.004076 18.008371 L 104.001359 23.007812 L 105 28.007254 L 104.001359 33.006696 L 101.005435 37.00279 L 95.994395 40.998884 L 78.99966 49.008371 L 75.005095 50.997768 L 74.006454 50.997768 L 58.991168 58.003906 L 54.996603 59.993304 L 52.000679 59.993304 L 51.002038 60.996652 L 46.008832 60.996652 L 39.00051 59.007254 C 28.60394 54.128906 18.278702 49.129464 8.006963 43.991629 L 8.006963 43.00558 L 2.995924 39.995536 L 0 33.992746 C 0 31.294085 0.338825 28.612723 0.998641 26.000558 C 1.997283 23.993862 2.995924 22.004464 5.991848 21.001116 Z M 5.991848 21.001116 " transform="matrix(0.219048,0,0,0.225806,0,0)"></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(25.882354%,9.411765%,1.568628%);fill-opacity:1;" d="M 1.3125 8.128906 L 1.09375 7.675781 L 1.3125 6.097656 L 1.753906 5.644531 L 9.855469 2.933594 L 9.636719 2.257812 C 9.710938 1.882812 9.785156 1.503906 9.855469 1.128906 L 10.078125 1.128906 L 10.515625 1.355469 L 10.734375 1.355469 L 12.265625 2.03125 C 12.914062 1.859375 13.589844 1.859375 14.238281 2.03125 C 14.910156 2.207031 15.570312 2.433594 16.210938 2.710938 L 16.648438 2.710938 L 17.960938 2.484375 L 18.398438 2.03125 L 19.058594 1.582031 L 20.371094 1.355469 L 21.246094 1.804688 L 21.246094 3.839844 L 21.027344 4.066406 L 20.371094 4.515625 L 20.589844 4.515625 L 21.464844 4.96875 L 21.90625 5.417969 L 21.90625 6.324219 L 21.6875 7 L 21.464844 7.675781 L 20.589844 8.128906 C 19.933594 8.582031 18.839844 9.257812 16.867188 9.933594 L 12.484375 11.96875 L 11.828125 12.417969 C 11.617188 12.515625 11.394531 12.589844 11.171875 12.644531 L 10.296875 12.644531 L 8.980469 12.195312 C 6.703125 11.09375 4.441406 9.964844 2.191406 8.804688 Z M 1.3125 8.128906 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(65.490198%,51.372552%,26.274511%);fill-opacity:1;" d="M 6.351562 5.195312 C 8.921875 4.820312 11.476562 4.371094 14.019531 3.839844 L 15.332031 4.289062 L 16.429688 4.515625 C 17.304688 4.515625 17.742188 4.289062 17.960938 4.066406 L 18.179688 3.613281 L 18.617188 3.160156 L 19.933594 2.484375 L 21.027344 2.03125 L 21.027344 3.839844 L 20.808594 4.066406 C 20.371094 4.289062 19.933594 4.515625 19.277344 4.289062 L 17.960938 4.742188 L 19.496094 4.96875 L 21.464844 5.644531 L 21.464844 7.226562 L 21.246094 7.453125 L 20.371094 8.128906 C 17.839844 9.550781 15.203125 10.757812 12.484375 11.742188 L 11.171875 12.417969 C 10.515625 12.417969 9.636719 12.417969 8.980469 11.96875 C 6.324219 10.59375 3.695312 9.164062 1.09375 7.675781 L 1.3125 7 L 1.535156 6.324219 Z M 6.351562 5.195312 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(56.078434%,44.313726%,22.352941%);fill-opacity:1;" d="M 4.382812 7.902344 L 3.503906 7.902344 L 3.285156 9.257812 L 3.066406 9.03125 L 3.285156 7.675781 L 2.847656 7.226562 L 2.628906 7.453125 L 2.410156 8.804688 L 2.191406 8.582031 L 1.972656 8.582031 L 2.410156 7.226562 L 1.972656 7 L 1.753906 8.355469 L 1.3125 8.128906 L 1.535156 6.546875 L 6.351562 6.324219 L 10.078125 8.128906 L 10.953125 10.839844 L 11.171875 12.417969 L 10.296875 12.417969 L 10.515625 10.839844 L 9.417969 10.613281 L 9.199219 12.195312 L 8.542969 11.96875 L 8.761719 10.386719 L 8.105469 10.160156 L 7.886719 11.515625 L 7.449219 11.289062 L 7.449219 9.710938 L 7.230469 9.03125 L 6.570312 9.257812 L 6.132812 10.613281 L 5.476562 10.386719 L 5.914062 9.03125 L 4.820312 8.128906 L 4.601562 8.355469 L 4.382812 9.710938 L 4.160156 9.484375 Z M 4.382812 7.902344 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(52.941179%,41.176471%,18.82353%);fill-opacity:1;" d="M 19.933594 2.484375 L 19.933594 2.710938 C 18.839844 3.160156 18.617188 3.839844 19.496094 4.289062 L 18.617188 4.289062 L 17.960938 4.742188 L 19.496094 4.96875 L 21.464844 5.644531 L 21.464844 7.226562 L 21.246094 7.453125 L 20.371094 8.128906 C 17.839844 9.550781 15.203125 10.757812 12.484375 11.742188 L 11.828125 12.195312 L 10.515625 12.417969 L 10.734375 12.195312 L 10.734375 9.710938 L 10.515625 8.804688 C 13.609375 6.628906 16.75 4.519531 19.933594 2.484375 Z M 19.933594 2.484375 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(47.450981%,36.470589%,16.862746%);fill-opacity:1;" d="M 21.027344 7.453125 C 21.464844 7 21.464844 6.546875 21.464844 5.644531 L 21.464844 7.226562 L 21.246094 7.453125 L 20.371094 8.128906 C 17.839844 9.550781 15.203125 10.757812 12.484375 11.742188 L 11.828125 12.195312 L 10.515625 12.417969 L 10.734375 12.195312 L 11.171875 12.195312 L 12.046875 11.96875 C 15.042969 10.46875 18.035156 8.960938 21.027344 7.453125 Z M 21.027344 7.453125 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(87.450981%,71.764708%,40.784314%);fill-opacity:1;" d="M 1.753906 6.097656 C 5.445312 4.722656 9.167969 3.441406 12.921875 2.257812 L 14.238281 2.484375 L 15.550781 2.933594 L 16.648438 3.160156 C 17.523438 3.160156 17.960938 2.933594 18.179688 2.710938 L 18.617188 2.257812 L 19.058594 2.03125 C 19.714844 1.582031 20.152344 1.582031 20.808594 1.804688 C 21.246094 2.03125 21.246094 2.484375 20.808594 2.710938 L 19.496094 3.160156 L 18.179688 3.613281 L 19.058594 4.289062 C 19.855469 4.75 20.660156 5.203125 21.464844 5.644531 C 21.6875 5.871094 21.246094 6.324219 20.589844 6.773438 C 17.578125 8.257812 14.511719 9.613281 11.390625 10.839844 C 10.734375 11.066406 9.855469 10.839844 8.980469 10.613281 C 6.472656 9.3125 3.988281 7.957031 1.535156 6.546875 L 1.535156 6.097656 Z M 1.753906 6.097656 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(79.215688%,64.313728%,33.333334%);fill-opacity:1;" d="M 2.628906 7.226562 L 2.410156 7.226562 L 4.820312 6.097656 L 6.351562 4.742188 L 8.105469 3.839844 C 9.554688 3.546875 11.015625 3.320312 12.484375 3.160156 C 12.921875 3.160156 13.582031 2.933594 14.019531 2.484375 L 14.457031 2.484375 C 12.945312 3.640625 11.078125 4.203125 9.199219 4.066406 C 8.105469 4.066406 7.230469 4.289062 6.570312 4.742188 L 5.039062 6.097656 C 4.601562 6.546875 3.722656 7 2.628906 7.226562 Z M 2.628906 7.226562 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(79.215688%,64.313728%,33.333334%);fill-opacity:1;" d="M 5.257812 7 C 4.601562 7 3.941406 7.226562 3.503906 7.675781 L 3.285156 7.675781 C 4.5625 6.878906 5.976562 6.34375 7.449219 6.097656 L 10.296875 5.417969 L 11.609375 4.742188 L 12.484375 4.066406 L 14.675781 2.484375 L 14.894531 2.710938 L 12.921875 4.289062 L 10.953125 5.644531 C 9.855469 6.324219 7.886719 6.773438 5.257812 7 Z M 1.972656 6.324219 L 3.066406 5.871094 L 4.160156 5.195312 L 6.132812 4.515625 L 5.476562 4.96875 L 3.722656 6.097656 L 2.191406 6.773438 L 1.972656 6.773438 L 1.535156 6.546875 Z M 9.855469 3.160156 L 11.609375 2.710938 L 13.363281 2.257812 L 13.582031 2.257812 C 13.144531 2.710938 12.484375 2.933594 11.828125 2.933594 Z M 18.617188 7.675781 C 19.277344 6.546875 20.152344 5.871094 21.246094 5.417969 L 21.464844 5.644531 C 20.808594 5.871094 20.152344 6.324219 19.714844 7 Z M 9.199219 7.902344 C 10.078125 7.902344 10.953125 7.675781 12.046875 7 L 14.019531 5.417969 L 15.550781 4.066406 C 16.210938 3.613281 16.648438 3.160156 17.304688 3.160156 L 18.179688 2.710938 L 20.589844 1.804688 L 20.808594 1.804688 L 20.589844 2.03125 L 18.398438 2.933594 L 16.210938 3.839844 L 14.457031 5.417969 C 13.800781 6.324219 13.144531 6.773438 12.703125 7 C 12.265625 7.453125 11.390625 7.902344 10.078125 8.128906 L 7.886719 8.582031 L 6.570312 9.257812 L 5.914062 8.804688 L 7.230469 8.355469 Z M 16.867188 6.097656 C 17.304688 5.195312 17.960938 4.742188 18.839844 4.289062 L 19.496094 4.515625 L 17.742188 5.871094 L 15.992188 7.902344 C 15.113281 8.582031 14.457031 9.03125 13.582031 9.257812 L 10.953125 9.710938 L 9.417969 10.613281 L 8.761719 10.386719 L 10.515625 9.484375 L 12.703125 9.03125 C 14.382812 8.582031 15.851562 7.542969 16.867188 6.097656 Z M 16.867188 6.097656 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(79.215688%,64.313728%,33.333334%);fill-opacity:1;" d="M 6.789062 7.675781 C 5.914062 7.675781 5.257812 7.902344 4.601562 8.355469 L 4.160156 8.128906 C 4.820312 7.675781 5.914062 7.226562 7.230469 7.226562 L 10.953125 6.097656 C 12.046875 5.644531 12.921875 4.96875 13.582031 4.289062 L 15.332031 2.710938 L 15.992188 2.933594 L 14.019531 4.515625 L 12.265625 5.871094 L 9.636719 7.226562 Z M 20.152344 4.742188 L 20.589844 4.96875 L 18.839844 6.324219 C 18.179688 6.773438 17.742188 7.453125 17.304688 8.355469 C 14.960938 9.164062 12.625 9.992188 10.296875 10.839844 L 12.703125 9.933594 L 14.894531 9.03125 C 15.992188 8.582031 17.304688 7.453125 18.617188 5.644531 Z M 13.144531 7.453125 C 14.671875 6.238281 16.203125 5.035156 17.742188 3.839844 C 17.960938 3.386719 19.058594 2.933594 21.027344 2.03125 L 21.027344 2.484375 C 20.402344 2.570312 19.804688 2.800781 19.277344 3.160156 L 18.179688 3.613281 L 18.398438 3.839844 L 15.769531 6.324219 L 13.582031 8.128906 L 10.515625 8.804688 C 9.417969 9.03125 8.761719 9.484375 8.105469 9.933594 L 7.449219 9.710938 C 9.289062 8.8125 11.191406 8.058594 13.144531 7.453125 Z M 6.570312 5.871094 L 6.570312 5.644531 L 6.789062 5.195312 L 7.230469 4.515625 L 8.761719 4.289062 L 10.515625 4.289062 C 10.734375 4.515625 10.734375 4.742188 10.296875 4.96875 L 8.761719 5.644531 L 7.449219 5.871094 L 7.449219 5.195312 L 8.324219 4.742188 L 9.199219 4.515625 L 9.417969 4.742188 L 8.761719 5.195312 L 8.980469 4.96875 L 8.324219 4.96875 L 8.105469 5.195312 C 7.886719 5.417969 8.105469 5.417969 8.324219 5.417969 L 9.199219 5.195312 L 9.855469 4.742188 L 9.855469 4.515625 L 8.761719 4.515625 L 7.449219 4.96875 L 6.789062 5.417969 Z M 6.570312 5.871094 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(41.960785%,25.098041%,14.117648%);fill-opacity:1;" d="M 9.855469 3.160156 L 11.171875 2.710938 L 12.265625 3.839844 L 14.238281 5.417969 L 15.550781 7 L 16.210938 8.582031 L 15.992188 9.484375 L 15.332031 9.710938 L 14.894531 9.710938 L 14.894531 9.257812 L 15.113281 9.03125 L 15.332031 8.582031 L 14.675781 7.675781 L 14.457031 7.453125 L 14.457031 7.226562 L 14.238281 6.773438 L 14.019531 6.773438 C 13.304688 7.003906 12.570312 7.15625 11.828125 7.226562 L 11.171875 7.226562 L 10.734375 6.097656 L 10.078125 4.289062 Z M 9.855469 3.160156 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(47.843137%,47.843137%,47.843137%);fill-opacity:1;" d="M 11.171875 7 L 11.390625 5.871094 L 12.265625 4.96875 L 13.582031 4.96875 L 14.894531 5.195312 L 15.113281 5.871094 L 14.894531 6.097656 L 12.921875 6.773438 L 11.609375 7 Z M 11.171875 7 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(59.215689%,59.215689%,59.215689%);fill-opacity:1;" d="M 12.265625 6.097656 L 12.046875 6.546875 L 12.265625 7 L 11.171875 7 L 11.390625 6.324219 Z M 12.265625 6.097656 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(92.54902%,92.54902%,92.54902%);fill-opacity:1;" d="M 9.855469 1.582031 L 10.734375 1.804688 L 12.921875 2.933594 C 13.75 3.667969 14.488281 4.5 15.113281 5.417969 L 14.894531 5.417969 L 14.019531 4.289062 L 12.484375 2.710938 L 10.734375 1.804688 Z M 9.855469 1.582031 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(78.039217%,78.039217%,78.039217%);fill-opacity:1;" d="M 9.855469 1.582031 L 10.078125 1.582031 L 10.515625 1.804688 C 10.609375 3.429688 11.140625 4.992188 12.046875 6.324219 L 11.171875 7 L 10.953125 6.546875 C 10.421875 5.246094 10.054688 3.882812 9.855469 2.484375 Z M 9.855469 1.582031 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(89.411765%,89.411765%,89.411765%);fill-opacity:1;" d="M 10.078125 3.386719 L 10.078125 2.933594 L 10.734375 2.933594 L 10.734375 3.160156 Z M 9.855469 2.03125 L 10.515625 2.03125 L 10.515625 2.710938 L 9.855469 2.710938 Z M 11.828125 6.097656 L 11.171875 6.773438 L 10.953125 6.324219 L 11.609375 5.871094 Z M 10.296875 4.515625 L 10.078125 3.839844 L 10.734375 3.613281 L 10.953125 4.066406 Z M 11.390625 5.195312 L 10.734375 5.644531 L 10.515625 4.96875 L 10.953125 4.515625 Z M 11.390625 5.195312 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(56.470591%,56.470591%,56.470591%);fill-opacity:1;" d="M 14.238281 6.546875 L 14.019531 6.324219 L 14.019531 5.871094 L 14.675781 5.871094 L 15.113281 6.097656 L 15.113281 6.324219 L 14.894531 6.546875 Z M 14.238281 6.546875 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(70.19608%,70.19608%,70.19608%);fill-opacity:1;" d="M 14.019531 5.871094 L 14.238281 5.644531 L 14.894531 5.644531 L 15.113281 5.871094 L 15.113281 6.097656 L 14.238281 6.097656 Z M 14.019531 5.871094 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(55.686277%,35.686275%,17.254902%);fill-opacity:1;" d="M 14.238281 6.773438 L 14.238281 6.097656 L 14.894531 6.097656 L 15.550781 6.773438 L 16.429688 8.128906 L 16.429688 8.582031 L 15.769531 9.484375 C 15.113281 9.710938 14.675781 9.710938 14.894531 9.257812 L 15.113281 8.582031 L 15.113281 8.128906 L 14.894531 7.675781 L 14.675781 7.453125 L 14.457031 6.773438 Z M 14.238281 6.773438 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(43.137255%,27.058825%,13.725491%);fill-opacity:1;" d="M 15.769531 8.355469 L 16.210938 7.675781 L 16.429688 8.128906 L 16.429688 8.582031 C 16.429688 9.03125 16.210938 9.484375 15.769531 9.484375 L 14.894531 9.484375 L 14.894531 9.03125 L 15.113281 8.582031 L 15.332031 8.355469 Z M 15.769531 8.355469 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(64.313728%,44.705883%,26.666668%);fill-opacity:1;" d="M 14.675781 6.324219 L 14.457031 6.097656 L 14.457031 5.871094 L 15.113281 5.871094 L 15.769531 6.097656 L 16.429688 7.226562 L 16.429688 8.582031 C 15.992188 8.804688 15.550781 9.03125 15.113281 8.804688 L 15.113281 8.355469 L 15.550781 7.902344 L 15.332031 7.453125 L 14.894531 7.226562 L 14.675781 6.773438 Z M 14.675781 6.324219 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 15.113281 8.128906 L 15.550781 7.902344 L 15.332031 8.355469 L 15.332031 8.804688 L 15.113281 8.804688 Z M 14.457031 5.871094 L 14.894531 6.546875 L 15.332031 7.453125 L 15.113281 7.453125 L 14.894531 6.773438 L 14.894531 6.546875 L 14.675781 6.546875 L 14.238281 6.097656 Z M 16.429688 7.675781 L 16.429688 7.453125 C 16.648438 7.902344 16.648438 8.355469 16.210938 8.582031 Z M 16.429688 7.675781 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 14.894531 6.097656 L 15.332031 6.546875 L 15.550781 6.773438 L 15.550781 7 L 15.769531 7.453125 L 15.769531 8.128906 L 15.550781 8.804688 L 15.550781 7 L 14.675781 5.871094 Z M 14.894531 6.097656 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 15.550781 6.324219 L 15.992188 6.546875 L 16.210938 7.226562 L 16.210938 8.128906 L 15.992188 8.804688 L 15.992188 6.546875 C 15.695312 6.324219 15.402344 6.101562 15.113281 5.871094 L 15.332031 5.871094 Z M 15.550781 6.324219 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 15.113281 5.871094 L 15.332031 6.324219 L 15.550781 6.773438 L 15.769531 7.226562 L 15.992188 7.453125 L 15.992188 8.128906 L 15.769531 8.804688 L 15.769531 7 L 15.550781 6.773438 L 15.332031 6.324219 L 14.894531 5.871094 Z M 15.332031 5.871094 L 15.769531 6.324219 L 15.992188 6.546875 L 15.550781 6.324219 Z M 15.332031 5.871094 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(43.137255%,27.058825%,13.725491%);fill-opacity:1;" d="M 16.210938 8.355469 C 15.992188 8.582031 15.769531 8.804688 15.550781 8.582031 L 15.332031 8.355469 L 15.992188 8.128906 Z M 16.210938 8.355469 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(69.411767%,69.411767%,69.411767%);fill-opacity:1;" d="M 16.210938 8.355469 L 15.992188 8.582031 L 15.332031 8.582031 L 15.769531 8.355469 Z M 16.210938 8.355469 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(49.019608%,49.019608%,49.019608%);fill-opacity:1;" d="M 16.210938 8.355469 L 15.992188 8.582031 L 15.332031 8.582031 L 15.769531 8.582031 L 15.992188 8.355469 Z M 16.210938 8.355469 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 15.992188 6.773438 L 15.769531 6.773438 L 15.992188 7 L 15.992188 6.773438 L 15.992188 7 L 15.769531 7 L 15.769531 6.773438 Z M 15.992188 6.773438 "></path></g></svg></span><label><input type="text" name="time-prep" value="00:15:00" class="input input-xs max-w-24 html-duration-picker"></label></div>"#,
                r#"<table class="table table-zebra table-xs"><thead><tr><th>Nutrition (per 100g)</th><th>Amount</th></tr></thead><tbody><tr><td>Calories</td><td><label><input type="text" name="calories" autocomplete="off" placeholder="368kcal" class="input input-xs max-w-24"></label></td></tr><tr><td>Total carbs</td><td><label><input type="text" name="total-carbohydrates" autocomplete="off" placeholder="35g" class="input input-xs max-w-24"></label></td></tr><tr><td>Sugars</td><td><label><input type="text" name="sugars" autocomplete="off" placeholder="3g" class="input input-xs max-w-24"></label></td></tr><tr><td>Protein</td><td><label><input type="text" name="protein" autocomplete="off" placeholder="21g" class="input input-xs max-w-24"></label></td></tr><tr><td>Total fat</td><td><label><input type="text" name="total-fat" autocomplete="off" placeholder="15g" class="input input-xs max-w-24"></label></td></tr><tr><td>Saturated fat</td><td><label><input type="text" name="saturated-fat" autocomplete="off" placeholder="1.8g" class="input input-xs max-w-24"></label></td></tr><tr><td>Unsaturated fat</td><td><label><input type="text" name="unsaturated-fat" autocomplete="off" placeholder="1.8g" class="input input-xs max-w-24"></label></td></tr><tr><td>Trans fat</td><td><label><input type="text" name="trans-fat" autocomplete="off" placeholder="1.8g" class="input input-xs max-w-24"></label></td></tr><tr><td>Cholesterol</td><td><label><input type="text" name="cholesterol" autocomplete="off" placeholder="1.1mg" class="input input-xs max-w-24"></label></td></tr><tr><td>Sodium</td><td><label><input type="text" name="sodium" autocomplete="off" placeholder="100mg" class="input input-xs max-w-24"></label></td></tr><tr><td>Fiber</td><td><label><input type="text" name="fiber" autocomplete="off" placeholder="8g" class="input input-xs max-w-24"></label></td></tr></tbody></table>"#,
                r#"<h2 class="font-semibold text-center pb-2"><span class="underline">Tools</span></h2>"#,
                r#"<ol id="tools-list" class="pl-4"><li class="pb-2"><div class="grid grid-flow-col items-center"><label class="flex gap-1"><div class="inline-block h-4 cursor-move handle mt-1"><svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"></path></svg></div><input type="text" name="tool" placeholder="1 frying pan" class="input input-bordered input-sm w-full" value="" _="on keydown if event.key is 'Enter' halt the event then call addItem(event)"></label><div class="ml-2 flex gap-2"><button type="button" class="btn btn-square btn-sm btn-outline btn-success" title="Shortcut: Enter" onclick="addItem(event)">+</button><button type="button" class="delete-button btn btn-square btn-sm btn-outline btn-error""#,
                r#"<h2 class="font-semibold text-center pb-2"><span class="underline">Ingredients</span><sup class="text-red-600">*</sup></h2>"#,
                r#"<ol id="ingredients-list" class="pl-4"><li class="pb-2"><div class="grid grid-flow-col items-center"><label class="flex gap-1"><div class="inline-block h-4 cursor-move handle mt-1"><svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"></path></svg></div><input required type="text" name="ingredient" value="" placeholder="1 cup of chopped onions" class="input input-bordered input-sm w-full" _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))"></label><div class="ml-2 flex gap-2"><button type="button" class="btn btn-square btn-sm btn-outline btn-success" title="Shortcut: Enter" onclick="addItem(event)">+</button><button type="button" class="delete-button btn btn-square btn-sm btn-outline btn-error""#,
                r#"<h2 class="font-semibold text-center pb-2"><span class="underline">Instructions</span><sup class="text-red-600">*</sup></h2>"#,
                r#"<ol id="instructions-list" class="grid list-decimal"><li class="pt-2 md:pl-0"><div class="flex"><label class="w-11/12"><textarea required name="instruction" rows="4" class="textarea textarea-bordered w-full" placeholder="Mix all ingredients together" _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))"></textarea></label><div class="grid gap-2 ml-2"><button type="button" class="btn btn-square btn-sm btn-outline btn-success" title="Shortcut: CTRL + Enter" onclick="addItem(event)">+</button><button type="button" class="delete-button btn btn-square btn-sm btn-outline btn-error""#,
                r#"<button class="btn btn-primary btn-block btn-sm">Submit</button>"#,
            ];
            for want in expected {
                res.assert_text_contains(want);
            }
        }
    }

    mod tests_recipe_categories {
        use super::*;
        use crate::recipes_routes::RecipeCategoryForm;
        use axum_test::http::StatusCode;
        use models::Recipe;
        use models::recipe::test_utils::a_complete_recipe_for_create;
        use testing::utils::{assert_ws_message, build_server_ws, create_app_state};

        const BASE_URI: &str = "/recipes/categories";

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::POST, BASE_URI).await?;
            assert_must_be_logged_in(Method::DELETE, BASE_URI).await?;
            Ok(())
        }

        #[tokio::test]
        async fn test_post_category_cannot_be_empty_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server
                .post(BASE_URI)
                .form(&RecipeCategoryForm {
                    category: String::new(),
                })
                .await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_post_user_already_has_category_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config).await?;

            let res = server
                .post(BASE_URI)
                .form(&RecipeCategoryForm {
                    category: "uncategorized".into(),
                })
                .await;

            res.assert_status_internal_server_error();
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Failed to add recipe category.","status":"alert-error","title":"Operation Failed"}}"#).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_post_add_category_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server
                .post(BASE_URI)
                .form(&RecipeCategoryForm {
                    category: "fish".into(),
                })
                .await;

            res.assert_status_ok();
            res.assert_text_contains(r#"<div class="badge badge-outline p-3 pr-0"><form class="inline-flex" hx-delete="/recipes/categories" hx-target="closest <div/>" hx-swap="delete"><input type="hidden" name="category" value="fish"><span class="select-none">fish</span><button class="btn btn-xs btn-ghost" type="submit">X</button></form></div><div class="badge badge-outline p-3 pr-0"><form class="inline-flex" hx-post="/recipes/categories" hx-target="closest <div/>" hx-swap="outerHTML"><label class="input"><input required type="text" placeholder="New category" class="input input-ghost input-xs w-[16ch] focus:outline-none" name="category" autocomplete="off"></label><button class="btn btn-xs btn-ghost">&#10003;</button></form></div>"#);
            Ok(())
        }

        #[tokio::test]
        async fn test_delete_category_empty_ok() -> Result<()> {
            send_delete_400(String::new()).await
        }

        #[tokio::test]
        async fn test_delete_cannot_delete_uncategorized_ok() -> Result<()> {
            send_delete_400("uncategorized".into()).await
        }

        #[tokio::test]
        async fn test_delete_nonexistent_category_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server
                .delete(BASE_URI)
                .form(&RecipeCategoryForm {
                    category: "ukraine".into(),
                })
                .await;

            res.assert_status(StatusCode::NO_CONTENT);
            Ok(())
        }

        #[tokio::test]
        async fn test_delete_category_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let category = String::from("midnight dinner");
            let state = create_app_state(config.clone()).await;
            let _ = Recipe::create(&state.mm, 1, &a_complete_recipe_for_create()).await?;
            Recipe::add_category(&state.mm, &category, 1).await?;

            let res = server
                .delete(BASE_URI)
                .form(&RecipeCategoryForm { category })
                .await;

            res.assert_status(StatusCode::NO_CONTENT);
            Ok(())
        }

        async fn send_delete_400(category: String) -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config).await?;

            let res = server
                .delete(BASE_URI)
                .form(&RecipeCategoryForm {
                    category: category.to_string(),
                })
                .await;

            res.assert_status_bad_request();
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Category cannot be empty or uncategorized.","status":"alert-error","title":"Operation Failed"}}"#).await;
            Ok(())
        }
    }

    mod tests_recipe_add_website {
        use super::*;

        use crate::recipes_routes::RecipeScrapeForm;
        use diesel::prelude::*;
        use diesel_async::RunQueryDsl;
        use models::report::ReportLog;
        use recipya_scraper::tests::support::scraper::scrape_test_websites;
        use repository::schema;
        use reqwest::StatusCode;
        use std::time::Duration;
        use testing::utils::{
            HIDDEN_WS_NOTIFICATION, assert_ws_message, build_server_ws, create_app_state,
        };

        const BASE_URI: &str = "/recipes/add/website";

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::POST, BASE_URI).await
        }

        #[tokio::test]
        async fn test_no_input_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config).await?;

            let res = server
                .post(BASE_URI)
                .form(&RecipeScrapeForm { urls: "".into() })
                .await;

            res.assert_status_bad_request();
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"No valid URLs found.","status":"alert-error","title":"Operation Failed"}}"#).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_no_valid_urls_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config).await?;

            let res = server
                .post(BASE_URI)
                .form(&RecipeScrapeForm {
                    urls: "I am a pig\noink oink".into(),
                })
                .await;

            res.assert_status_bad_request();
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"No valid URLs found.","status":"alert-error","title":"Operation Failed"}}"#).await;
            Ok(())
        }

        #[tokio::test]
        #[serial_test::serial]
        async fn test_add_one_valid_url_from_unsupported_websites_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;

            let res = server
                .post(BASE_URI)
                .form(&RecipeScrapeForm {
                    urls: "https://www.example.com".into(),
                })
                .await;

            res.assert_status(StatusCode::ACCEPTED);
            assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetched 1/1</p><div id="export-progress"><progress max="100" value="100.00"></progress></div></div></div>"#).await;
            assert_ws_message(&mut ws_server, HIDDEN_WS_NOTIFICATION).await;
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","action":"View /reports?view=latest","message":"Fetching the recipe failed.","status":"alert-info","title":"Operation Failed"}}"#).await;
            tokio::time::sleep(Duration::from_millis(500)).await;
            let got_logs = fetch_logs(config.clone()).await?;
            pretty_assertions::assert_eq!(got_logs, vec![example_report_log()]);
            Ok(())
        }

        #[tokio::test]
        #[serial_test::serial]
        async fn test_add_one_valid_url_from_supported_websites_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            scrape_test_websites(1).await?;

            let res = server
                .post(BASE_URI)
                .form(&RecipeScrapeForm {
                    urls: "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/"
                        .into(),
                })
                .await;

            res.assert_status(StatusCode::ACCEPTED);
            assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetched 1/1</p><div id="export-progress"><progress max="100" value="100.00"></progress></div></div></div>"#).await;
            assert_ws_message(&mut ws_server, HIDDEN_WS_NOTIFICATION).await;
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","action":"View /reports?view=latest","message":"Fetching the recipe failed.","status":"alert-info","title":"Operation Failed"}}"#).await;
            tokio::time::sleep(Duration::from_millis(50)).await;
            let got_logs = fetch_logs(config.clone()).await?;
            pretty_assertions::assert_eq!(got_logs, vec![all_recipes_report_log(1)]);
            Ok(())
        }

        #[tokio::test]
        #[serial_test::serial]
        async fn test_add_duplicates_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            scrape_test_websites(1).await?;

            let res = server
                .post(BASE_URI)
                .form(&RecipeScrapeForm {
                    urls: "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/\nhttps://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies".into(),
                })
                .await;

            res.assert_status(StatusCode::ACCEPTED);
            assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetched 1/1</p><div id="export-progress"><progress max="100" value="100.00"></progress></div></div></div>"#).await;
            assert_ws_message(&mut ws_server, HIDDEN_WS_NOTIFICATION).await;
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","action":"View /reports?view=latest","message":"Fetching the recipe failed.","status":"alert-info","title":"Operation Failed"}}"#).await;
            tokio::time::sleep(Duration::from_millis(50)).await;
            let got_logs = fetch_logs(config.clone()).await?;
            pretty_assertions::assert_eq!(got_logs, vec![all_recipes_report_log(1)]);
            Ok(())
        }

        #[tokio::test]
        #[serial_test::serial]
        async fn test_add_a_website_that_has_already_been_added_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            let form = RecipeScrapeForm {
                urls: "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/\nhttps://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies".into(),
            };
            scrape_test_websites(1).await?;
            let _ = server.post(BASE_URI).form(&form).await;

            let res = server.post(BASE_URI).form(&form).await;

            res.assert_status(StatusCode::ACCEPTED);
            assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetched 1/1</p><div id="export-progress"><progress max="100" value="100.00"></progress></div></div></div>"#).await;
            assert_ws_message(&mut ws_server, HIDDEN_WS_NOTIFICATION).await;
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","action":"View /reports?view=latest","message":"Fetching the recipe failed.","status":"alert-info","title":"Operation Failed"}}"#).await;
            tokio::time::sleep(Duration::from_millis(100)).await;
            let got_logs = fetch_logs(config.clone()).await?;
            pretty_assertions::assert_eq!(
                got_logs,
                vec![all_recipes_report_log(1), all_recipes_report_log(2)]
            );
            Ok(())
        }

        #[tokio::test]
        #[serial_test::serial]
        async fn test_add_many_valid_urls_from_supported_websites_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            scrape_test_websites(1).await?;
            scrape_test_websites(2).await?;
            scrape_test_websites(3).await?;

            let res = server.post(BASE_URI).form(&RecipeScrapeForm {
                urls: "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/\nhttps://www.acouplecooks.com/chicken-meatballs-baked\nhttps://addapinch.com/easy-grape-jelly-meatballs-recipe/".into(),
            }).await;

            res.assert_status(StatusCode::ACCEPTED);
            assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetched 1/3</p><div id="export-progress"><progress max="100" value="33.33"></progress></div></div></div>"#).await;
            assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetched 2/3</p><div id="export-progress"><progress max="100" value="66.67"></progress></div></div></div>"#).await;
            assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetched 3/3</p><div id="export-progress"><progress max="100" value="100.00"></progress></div></div></div>"#).await;
            assert_ws_message(&mut ws_server, HIDDEN_WS_NOTIFICATION).await;
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","action":"View /reports?view=latest","message":"Fetched: 0. Skipped: 3","status":"alert-info","title":"Operation Successful"}}"#).await;
            tokio::time::sleep(Duration::from_millis(50)).await;
            let got_logs = fetch_logs(config.clone()).await?;
            pretty_assertions::assert_eq!(
                got_logs,
                vec![
                    ReportLog {
                        id: 1,
                        report_id: 1,
                        title: "https://addapinch.com/easy-grape-jelly-meatballs-recipe".to_owned(),
                        is_success: false,
                        is_warning: false,
                        is_error: true,
                        error_reason: "Scraper(DomainNotImplemented)".to_owned(),
                    },
                    ReportLog {
                        id: 2,
                        report_id: 1,
                        title: "https://www.acouplecooks.com/chicken-meatballs-baked".to_owned(),
                        is_success: false,
                        is_warning: false,
                        is_error: true,
                        error_reason: "Scraper(DomainNotImplemented)".to_owned(),
                    },
                    ReportLog {
                        id: 3,
                        report_id: 1,
                        title:
                            "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies"
                                .to_owned(),
                        is_success: false,
                        is_warning: false,
                        is_error: true,
                        error_reason: "Scraper(DomainNotImplemented)".to_owned(),
                    },
                ]
            );
            Ok(())
        }

        async fn fetch_logs(config: Config) -> Result<Vec<ReportLog>> {
            let state = create_app_state(config).await;
            let mut conn = state.mm.pool.get().await?;
            let logs = schema::reports_logs::table
                .select(ReportLog::as_select())
                .load(&mut conn)
                .await?;
            Ok(logs)
        }

        fn all_recipes_report_log(id: i64) -> ReportLog {
            ReportLog {
                id,
                report_id: id,
                title: "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies"
                    .to_string(),
                is_success: false,
                is_warning: false,
                is_error: true,
                error_reason: "Scraper(DomainNotImplemented)".to_string(),
            }
        }

        fn example_report_log() -> ReportLog {
            ReportLog {
                id: 1,
                report_id: 1,
                title: "https://www.example.com/".to_string(),
                is_success: false,
                is_warning: false,
                is_error: true,
                error_reason: "Scraper(UnknownWebsite)".to_string(),
            }
        }
    }

    mod tests_search {
        use super::*;
        use axum_test::TestServer;
        use models::Recipe;
        use models::params::SearchParams;
        use models::recipe::test_utils::a_complete_recipe_for_create;
        use repository::ModelManager;

        const BASE_URI: &str = "/recipes/search";

        async fn insert_recipes(mm: &ModelManager, user_id: i64) -> Result<()> {
            let mut recipe1 = a_complete_recipe_for_create();
            recipe1.name = "Chinese Firmware".to_string();
            let mut recipe2 = a_complete_recipe_for_create();
            recipe2.name = "Lovely Canada".to_string();
            let mut recipe3 = a_complete_recipe_for_create();
            recipe3.name = "Lovely Ukraine".to_string();
            for recipe in [recipe1, recipe2, recipe3] {
                let _ = Recipe::create(mm, user_id, &recipe).await?;
            }
            Ok(())
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, BASE_URI).await
        }

        #[tokio::test]
        async fn test_page_below_1_returns_recipes_from_first_page() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let mut server: TestServer = build_server_logged_in(config).await?;
            server.add_header(axum_htmx::HX_REQUEST, "true");
            insert_recipes(&state.mm, 1).await?;

            let res = server
                .get(BASE_URI)
                .add_query_params(SearchParams {
                    q: Some("lovely".to_string()),
                    sort: None,
                    page: Some(0),
                })
                .await;

            res.assert_status_ok();
            res.assert_header(axum_htmx::HX_RETARGET, "#list-recipes");
            assert_html(
                res,
                vec![
                    r#"<h2 class="sm:font-semibold sm:w-[25ch] sm:break-words sm:min-h-14">Lovely Ukraine</h2>"#,
                    r#"<h2 class="sm:font-semibold sm:w-[25ch] sm:break-words sm:min-h-14">Lovely Canada</h2>"#,
                    r#"<button class="join-item btn btn-disabled w-12" title="Previous page" aria-label="Previous page">‹</button><button class="join-item btn btn-active w-12" aria-current="page" aria-label="Page 1, current page">1</button><button class="join-item btn btn-disabled w-12" title="Next page" aria-label="Next page">›</button>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_empty_query_redirects_to_recipes_index() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let mut server: TestServer = build_server_logged_in(config).await?;
            server.add_header(axum_htmx::HX_REQUEST, "true");
            insert_recipes(&state.mm, 1).await?;

            let res = server
                .get(BASE_URI)
                .add_query_params(SearchParams {
                    q: Some("".to_string()),
                    sort: None,
                    page: Some(0),
                })
                .await;

            res.assert_status_ok();
            res.assert_header(axum_htmx::HX_RETARGET, "#content");
            assert_html(
                res,
                vec![
                    r#"<h2 class="sm:font-semibold sm:w-[25ch] sm:break-words sm:min-h-14">Chinese Firmware</h2>"#,
                    r#"<h2 class="sm:font-semibold sm:w-[25ch] sm:break-words sm:min-h-14">Lovely Ukraine</h2>"#,
                    r#"<h2 class="sm:font-semibold sm:w-[25ch] sm:break-words sm:min-h-14">Lovely Canada</h2>"#,
                    r#"<button class="join-item btn btn-disabled w-12" title="Previous page" aria-label="Previous page">‹</button><button class="join-item btn btn-active w-12" aria-current="page" aria-label="Page 1, current page">1</button><button class="join-item btn btn-disabled w-12" title="Next page" aria-label="Next page">›</button>"#,
                ],
            );

            Ok(())
        }

        #[tokio::test]
        async fn test_no_results() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let mut server: TestServer = build_server_logged_in(config).await?;
            server.add_header(axum_htmx::HX_REQUEST, "true");
            insert_recipes(&state.mm, 1).await?;

            let res = server
                .get(BASE_URI)
                .add_query_params(SearchParams {
                    q: Some("kool-aid".to_string()),
                    sort: None,
                    page: Some(0),
                })
                .await;

            res.assert_status_ok();
            assert_html(
                res,
                vec![
                    r#"<div class="grid place-content-center text-sm text-center h-3/5 md:text-base"><p>No results found.</p></div>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_results_query1() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let mut server: TestServer = build_server_logged_in(config).await?;
            server.add_header(axum_htmx::HX_REQUEST, "true");
            insert_recipes(&state.mm, 1).await?;

            let res = server
                .get(BASE_URI)
                .add_query_params(SearchParams {
                    q: Some("love".to_string()),
                    sort: None,
                    page: Some(0),
                })
                .await;

            res.assert_status_ok();
            assert_html(
                res,
                vec![
                    r##"<section class="card-side sm:card card-compact card-border bg-base-100 shadow-lg indicator w-full"><span class="hidden sm:block"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral indicator-item indicator-center" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "cat:dinner"}" _="on click put &quot;cat:dinner&quot; into #search_recipes.value">dinner</span></span><figure class="relative cursor-pointer" hx-get="/recipes/2" hx-target="#content" hx-push-url="true" hx-trigger="mousedown" hx-swap="innerHTML show:window:top transition:true"><img class="h-28 w-24 object-cover rounded-t-lg sm:h-40 sm:min-w-full sm:w-full" src="/data/images/Placeholders/placeholder.recipe.webp" alt="Image of the Lovely Canada recipe"><div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex"><p class="p-2 text-sm">This is the most delicious recipe!</p></div></figure><div class="card-body justify-between"><h2 class="sm:font-semibold sm:w-[25ch] sm:break-words sm:min-h-14">Lovely Canada</h2><div class="sm:max-h-14 sm:overflow-y-auto sm:content-end sm:min-h-14"><div class="flex flex-col flex-wrap overflow-x-auto max-h-12 pb-2 sm:pb-0 sm:max-h-none sm:flex-auto sm:flex-row"><span class="sm:hidden"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "cat:dinner"}" _="on click put &quot;cat:dinner&quot; into #search_recipes.value">dinner</span></span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "tag":tofu}" _="on click put &quot;tag:tofu&quot; into #search-recipes.value">tofu</span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "tag":vegetarian}" _="on click put &quot;tag:vegetarian&quot; into #search-recipes.value">vegetarian</span></div></div><div class="card-actions flex-col-reverse h-fit"><button class="btn btn-block btn-xs btn-outline sm:btn-sm" hx-get="/recipes/2" hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true">View</button></div></div></section>"##,
                    r##"<section class="card-side sm:card card-compact card-border bg-base-100 shadow-lg indicator w-full"><span class="hidden sm:block"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral indicator-item indicator-center" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "cat:dinner"}" _="on click put &quot;cat:dinner&quot; into #search_recipes.value">dinner</span></span><figure class="relative cursor-pointer" hx-get="/recipes/3" hx-target="#content" hx-push-url="true" hx-trigger="mousedown" hx-swap="innerHTML show:window:top transition:true"><img class="h-28 w-24 object-cover rounded-t-lg sm:h-40 sm:min-w-full sm:w-full" src="/data/images/Placeholders/placeholder.recipe.webp" alt="Image of the Lovely Ukraine recipe"><div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex"><p class="p-2 text-sm">This is the most delicious recipe!</p></div></figure><div class="card-body justify-between"><h2 class="sm:font-semibold sm:w-[25ch] sm:break-words sm:min-h-14">Lovely Ukraine</h2><div class="sm:max-h-14 sm:overflow-y-auto sm:content-end sm:min-h-14"><div class="flex flex-col flex-wrap overflow-x-auto max-h-12 pb-2 sm:pb-0 sm:max-h-none sm:flex-auto sm:flex-row"><span class="sm:hidden"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "cat:dinner"}" _="on click put &quot;cat:dinner&quot; into #search_recipes.value">dinner</span></span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "tag":tofu}" _="on click put &quot;tag:tofu&quot; into #search-recipes.value">tofu</span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "tag":vegetarian}" _="on click put &quot;tag:vegetarian&quot; into #search-recipes.value">vegetarian</span></div></div><div class="card-actions flex-col-reverse h-fit"><button class="btn btn-block btn-xs btn-outline sm:btn-sm" hx-get="/recipes/3" hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true">View</button></div></div></section>"##,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_results_query2() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let mut server: TestServer = build_server_logged_in(config).await?;
            server.add_header(axum_htmx::HX_REQUEST, "true");
            insert_recipes(&state.mm, 1).await?;

            let res = server
                .get(BASE_URI)
                .add_query_params(SearchParams {
                    q: Some("Chinese".to_string()),
                    sort: None,
                    page: Some(0),
                })
                .await;

            res.assert_status_ok();
            assert_html(
                res,
                vec![
                    r##"<section class="card-side sm:card card-compact card-border bg-base-100 shadow-lg indicator w-full"><span class="hidden sm:block"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral indicator-item indicator-center" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "cat:dinner"}" _="on click put &quot;cat:dinner&quot; into #search_recipes.value">dinner</span></span><figure class="relative cursor-pointer" hx-get="/recipes/1" hx-target="#content" hx-push-url="true" hx-trigger="mousedown" hx-swap="innerHTML show:window:top transition:true"><img class="h-28 w-24 object-cover rounded-t-lg sm:h-40 sm:min-w-full sm:w-full" src="/data/images/Placeholders/placeholder.recipe.webp" alt="Image of the Chinese Firmware recipe"><div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex"><p class="p-2 text-sm">This is the most delicious recipe!</p></div></figure><div class="card-body justify-between"><h2 class="sm:font-semibold sm:w-[25ch] sm:break-words sm:min-h-14">Chinese Firmware</h2><div class="sm:max-h-14 sm:overflow-y-auto sm:content-end sm:min-h-14"><div class="flex flex-col flex-wrap overflow-x-auto max-h-12 pb-2 sm:pb-0 sm:max-h-none sm:flex-auto sm:flex-row"><span class="sm:hidden"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "cat:dinner"}" _="on click put &quot;cat:dinner&quot; into #search_recipes.value">dinner</span></span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "tag":tofu}" _="on click put &quot;tag:tofu&quot; into #search-recipes.value">tofu</span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "tag":vegetarian}" _="on click put &quot;tag:vegetarian&quot; into #search-recipes.value">vegetarian</span></div></div><div class="card-actions flex-col-reverse h-fit"><button class="btn btn-block btn-xs btn-outline sm:btn-sm" hx-get="/recipes/1" hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true">View</button></div></div></section>"##,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_results_query3() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let mut server: TestServer = build_server_logged_in(config).await?;
            server.add_header(axum_htmx::HX_REQUEST, "true");
            insert_recipes(&state.mm, 1).await?;

            let res = server
                .get(BASE_URI)
                .add_query_params(SearchParams {
                    q: Some("LOVELY".to_string()),
                    sort: None,
                    page: Some(0),
                })
                .await;

            res.assert_status_ok();
            assert_html(
                res,
                vec![
                    r##"<section class="card-side sm:card card-compact card-border bg-base-100 shadow-lg indicator w-full"><span class="hidden sm:block"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral indicator-item indicator-center" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "cat:dinner"}" _="on click put &quot;cat:dinner&quot; into #search_recipes.value">dinner</span></span><figure class="relative cursor-pointer" hx-get="/recipes/2" hx-target="#content" hx-push-url="true" hx-trigger="mousedown" hx-swap="innerHTML show:window:top transition:true"><img class="h-28 w-24 object-cover rounded-t-lg sm:h-40 sm:min-w-full sm:w-full" src="/data/images/Placeholders/placeholder.recipe.webp" alt="Image of the Lovely Canada recipe"><div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex"><p class="p-2 text-sm">This is the most delicious recipe!</p></div></figure><div class="card-body justify-between"><h2 class="sm:font-semibold sm:w-[25ch] sm:break-words sm:min-h-14">Lovely Canada</h2><div class="sm:max-h-14 sm:overflow-y-auto sm:content-end sm:min-h-14"><div class="flex flex-col flex-wrap overflow-x-auto max-h-12 pb-2 sm:pb-0 sm:max-h-none sm:flex-auto sm:flex-row"><span class="sm:hidden"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "cat:dinner"}" _="on click put &quot;cat:dinner&quot; into #search_recipes.value">dinner</span></span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "tag":tofu}" _="on click put &quot;tag:tofu&quot; into #search-recipes.value">tofu</span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "tag":vegetarian}" _="on click put &quot;tag:vegetarian&quot; into #search-recipes.value">vegetarian</span></div></div><div class="card-actions flex-col-reverse h-fit"><button class="btn btn-block btn-xs btn-outline sm:btn-sm" hx-get="/recipes/2" hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true">View</button></div></div></section>"##,
                    r##"<section class="card-side sm:card card-compact card-border bg-base-100 shadow-lg indicator w-full"><span class="hidden sm:block"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral indicator-item indicator-center" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "cat:dinner"}" _="on click put &quot;cat:dinner&quot; into #search_recipes.value">dinner</span></span><figure class="relative cursor-pointer" hx-get="/recipes/3" hx-target="#content" hx-push-url="true" hx-trigger="mousedown" hx-swap="innerHTML show:window:top transition:true"><img class="h-28 w-24 object-cover rounded-t-lg sm:h-40 sm:min-w-full sm:w-full" src="/data/images/Placeholders/placeholder.recipe.webp" alt="Image of the Lovely Ukraine recipe"><div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex"><p class="p-2 text-sm">This is the most delicious recipe!</p></div></figure><div class="card-body justify-between"><h2 class="sm:font-semibold sm:w-[25ch] sm:break-words sm:min-h-14">Lovely Ukraine</h2><div class="sm:max-h-14 sm:overflow-y-auto sm:content-end sm:min-h-14"><div class="flex flex-col flex-wrap overflow-x-auto max-h-12 pb-2 sm:pb-0 sm:max-h-none sm:flex-auto sm:flex-row"><span class="sm:hidden"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "cat:dinner"}" _="on click put &quot;cat:dinner&quot; into #search_recipes.value">dinner</span></span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "tag":tofu}" _="on click put &quot;tag:tofu&quot; into #search-recipes.value">tofu</span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{"q": "tag":vegetarian}" _="on click put &quot;tag:vegetarian&quot; into #search-recipes.value">vegetarian</span></div></div><div class="card-actions flex-col-reverse h-fit"><button class="btn btn-block btn-xs btn-outline sm:btn-sm" hx-get="/recipes/3" hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true">View</button></div></div></section>"##,
                ],
            );
            Ok(())
        }
    }

    mod tests_supported_applications {
        use super::*;

        use axum::http::header::CONTENT_TYPE;

        use testing::utils::assert_must_be_logged_in;

        const BASE_URI: &str = "/recipes/supported-applications";

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, BASE_URI).await
        }

        #[tokio::test]
        async fn test_returns_list_of_apps_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            res.assert_header(CONTENT_TYPE, "text/html; charset=utf-8");
            res.assert_text_contains(r#"<tr class="text-center"><td>1</td><td><a class="underline" href="https://www.accuchef.com" target="_blank">AccuChef</a></td><td></td></tr><tr class="text-center"><td>2</td><td><a class="underline" href="https://www.bigoven.com" target="_blank">BigOven</a></td><td>.txt</td></tr><tr class="text-center"><td>3</td><td><a class="underline" href="https://cheftap.com" target="_blank">ChefTap</a></td><td>.txt</td></tr><tr class="text-center"><td>4</td><td><a class="underline" href="https://cooklang.org/" target="_blank">Cooklang</a></td><td>.cook</td></tr><tr class="text-center"><td>5</td><td><a class="underline" href="https://cooklang.org/" target="_blank">COOKmate</a></td><td>.mcb, .mmf, .rk, .xml</td></tr><tr class="text-center"><td>6</td><td><a class="underline" href="https://crouton.app" target="_blank">Crouton</a></td><td>.crumb</td></tr><tr class="text-center"><td>7</td><td><a class="underline" href="https://easy-recipe-deluxe.software.informer.com" target="_blank">Easy Recipe Deluxe</a></td><td></td></tr><tr class="text-center"><td>8</td><td><a class="underline" href="https://www.kalorio.de" target="_blank">Kalorio</a></td><td>.txt, .xml</td></tr><tr class="text-center"><td>9</td><td><a class="underline" href="https://www.mastercook.com" target="_blank">MasterCook</a></td><td>.mx2, .mxp, .mz2, .txt</td></tr><tr class="text-center"><td>10</td><td><a class="underline" href="https://web.archive.org/web/20081221021301/http://episoft.home.comcast.net/~episoft/mmdown.htm" target="_blank">Meal-Master</a></td><td>.mx2, .mxp, .mz2, .txt</td></tr><tr class="text-center"><td>11</td><td><a class="underline" href="https://www.paprikaapp.com" target="_blank">Paprika</a></td><td>.paprikarecipes</td></tr><tr class="text-center"><td>12</td><td><a class="underline" href="https://recipekeeperonline.com" target="_blank">Recipe Keeper</a></td><td></td></tr><tr class="text-center"><td>13</td><td><a class="underline" href="https://recipemd.org/" target="_blank">RecipeMD</a></td><td>.md</td></tr><tr class="text-center"><td>14</td><td><a class="underline" href="https://recipesage.com" target="_blank">RecipeSage</a></td><td>.json, .txt, .xml</td></tr><tr class="text-center"><td>15</td><td><a class="underline" href="https://www.rezkonv.de/" target="_blank">Rezkonv</a></td><td>.rk</td></tr><tr class="text-center"><td>16</td><td><a class="underline" href="https://www.mysaffronapp.com" target="_blank">Saffron</a></td><td>.txt</td></tr>"#);
            Ok(())
        }
    }

    mod tests_supported_websites {
        use super::*;

        use axum_test::http::header::CONTENT_TYPE;

        use testing::utils::assert_must_be_logged_in;

        const BASE_URI: &str = "/recipes/supported-websites";

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, BASE_URI).await
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

    fn create_form(recipe: &RecipeForCreate) -> MultipartForm {
        let mut form = MultipartForm::new().add_part("title", Part::text(&recipe.name));

        for (_section, ingredients) in &recipe.ingredients {
            for ingredient in ingredients {
                form = form.add_part("ingredient", Part::text(ingredient));
            }
        }

        for (_section, instructions) in &recipe.instructions {
            for instruction in instructions {
                form = form.add_part("instruction", Part::text(instruction));
            }
        }

        for tool in &recipe.tools {
            form = form.add_part(
                "tool",
                Part::text(format!("{} {}", tool.quantity, tool.name)),
            );
        }

        for keyword in &recipe.keywords {
            form = form.add_part("keyword", Part::text(keyword));
        }

        if let Some(n) = &recipe.yield_ {
            form = form.add_part("yield", Part::text(n.to_string()));
        }

        for image in &recipe.images {
            let file_name = format!("{image}.jpg");
            form = form.add_part(
                "media",
                Part::file_name(Part::text(image.to_string()), file_name),
            );
        }

        for video in &recipe.videos {
            form = form.add_part(
                "media",
                Part::file_name(Part::text(video.video.to_string()), video.video.to_string()),
            );
        }

        if let Some(times) = &recipe.times {
            form = form.add_part("time-prep", Part::text(seconds_to_hms(times.prep_seconds)));
            form = form.add_part("time-cook", Part::text(seconds_to_hms(times.cook_seconds)));
        }

        if let Some(v) = &recipe.category {
            form = form.add_part("category", Part::text(v))
        }

        if let Some(v) = &recipe.cuisine {
            form = form.add_part("cuisine", Part::text(v))
        }

        if let Some(v) = &recipe.description {
            form = form.add_part("description", Part::text(v))
        }

        if let Some(v) = &recipe.source {
            form = form.add_part("source", Part::text(v))
        }

        if let Some(n) = &recipe.nutrition {
            if let Some(v) = n.calories_kcal {
                form = form.add_part("calories", Part::text(v.to_string()))
            }
            if let Some(v) = n.total_carbohydrates {
                form = form.add_part("total-carbohydrates", Part::text(v.to_string()))
            }
            if let Some(v) = n.sugars_g {
                form = form.add_part("sugars", Part::text(v.to_string()))
            }
            if let Some(v) = n.protein_g {
                form = form.add_part("protein", Part::text(v.to_string()))
            }
            if let Some(v) = n.total_fat_g {
                form = form.add_part("total-fat", Part::text(v.to_string()))
            }
            if let Some(v) = n.saturated_fat_g {
                form = form.add_part("saturated-fat", Part::text(v.to_string()))
            }
            if let Some(v) = n.unsaturated_fat_g {
                form = form.add_part("unsaturated-fat", Part::text(v.to_string()))
            }
            if let Some(v) = n.cholesterol_mg {
                form = form.add_part("cholesterol", Part::text(v.to_string()))
            }
            if let Some(v) = n.sodium_mg {
                form = form.add_part("sodium", Part::text(v.to_string()))
            }
            if let Some(v) = n.fiber_g {
                form = form.add_part("fiber", Part::text(v.to_string()))
            }
            if let Some(v) = n.trans_fat_g {
                form = form.add_part("trans-fat", Part::text(v.to_string()))
            }
            if let Some(v) = &n.serving_size {
                form = form.add_part("serving-size", Part::text(v))
            }
        }

        form
    }

    fn seconds_to_hms(seconds: i32) -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let seconds = seconds % 60;

        format!("{hours:02}:{minutes:02}:{seconds:02}")
    }
}
