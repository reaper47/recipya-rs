#[cfg(test)]
mod tests {
    use axum_test::TestResponse;
    use reqwest::Method;

    use test_db::TestDb;
    use test_fixtures::assert_html;
    use test_utils::{assert_must_be_logged_in, build_server_logged_in};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    const BASE_URI: &str = "/recipes/add";

    #[tokio::test]
    async fn test_add_recipe_must_be_logged_in_ok() -> Result<()> {
        assert_must_be_logged_in(Method::GET, BASE_URI).await
    }

    #[tokio::test]
    async fn test_add_recipe_is_htmx_request_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let mut server = build_server_logged_in(config).await?;
        server.add_header(axum_htmx::HX_REQUEST, "true");

        let res = server.get(BASE_URI).await;

        assert_content(&res);
        Ok(())
    }

    #[tokio::test]
    async fn test_add_recipe_is_not_htmx_request_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let server = build_server_logged_in(config).await?;

        let res = server.get(BASE_URI).await;

        assert_content(&res);
        Ok(())
    }

    fn assert_content(res: &TestResponse) {
        assert_html(
            res,
            &[
                r#"<title hx-swap-oob="true">Add Recipe | Recipya</title>"#,
                r#"<img class="object-cover w-full h-40 rounded-t-xl" src="/public/img/recipes/new/manual.webp" alt="Writing on a piece of paper with a traditional pen.">"#,
                r#"<img class="object-cover w-full h-40 rounded-t-xl" src="/public/img/recipes/new/import.webp" alt="Earth connected from end-to-end by telecommunications.">"#,
                r#"<img class="object-cover w-full h-40 rounded-t-xl" src="/public/img/recipes/new/camera.webp" alt="A cellphone used as a camera.">"#,
                r#"<img class="object-cover w-full h-40 rounded-t-xl" src="/public/img/recipes/new/schema.webp" alt="A bunch of shipping containers on a cargo boat.">"#,
                r##"<button class="underline" hx-get="/recipes/supported-websites" hx-target="#search-results" _="on click open #supported-websites-dialog">supported</button>"##,
                r#"<dialog id="websites-dialog" class="modal"><div class="modal-box max-w-md"><form method="dialog"><button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button></form><h3 class="font-bold text-lg mb-1">Fetch recipes from websites</h3><p class="text-sm mb-4">Enter one or more URLs, each on a new line.</p><form class="flex flex-col gap-3" hx-post="/recipes/add/website" hx-swap="none" _="on submit call #websites-dialog.close() then set me.querySelector('textarea').value to ''"><textarea class="textarea w-full text-sm" name="urls" rows="10" placeholder="https://example.com/recipe-1"#,
                r#"</textarea><div class="flex justify-end gap-2"><button type="button" class="btn btn-ghost btn-sm" _="on click set #websites-dialog's querySelector('textarea').value to '' then call #websites-dialog.close()">Cancel</button><button class="btn btn-primary btn-sm">Fetch recipes</button></div></form></div></dialog>"#,
                r#"<dialog id="supported-websites-dialog" class="modal"><div class="modal-box h-2/3"><form method="dialog"><button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button></form><h3 class="mb-1"><label class="floating-label"><input type="search" placeholder="Search a website" class="input input-sm w-11/12" _="on input show <tbody>tr/> in next <table/> when its textContent.toLowerCase() contains my value.toLowerCase()"></label></h3><div class="overflow-x-auto"><table class="table table-zebra table-sm"><thead><tr class="text-center"><th class="py-1">Number</th><th class="py-1">Website</th></tr></thead><tbody id="search-results"></tbody></table></div></div></dialog>"#,
                r#"<dialog class="modal" id="supported-apps-import-dialog"><div class="modal-box h-2/3"><form method="dialog"><button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button></form><h3 class="mb-1"><label class="floating-label"><input type="search" placeholder="Search an application" class="input input-sm w-11/12" _="on input show <tbody>tr/> in next <table/> when its textContent.toLowerCase() contains my value.toLowerCase()"></label></h3><div class="overflow-x-auto"><table class="table table-zebra table-sm"><thead><tr class="text-center"><th class="py-1">Number</th><th class="py-1">Application</th><th class="py-1">File Formats</th></tr></thead><tbody id="application-results"></tbody></table></div></div></dialog>"#,
                r##"<dialog class="modal" id="add-ocr-dialog"><div class="modal-box"><form method="dialog"><button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button></form><h3 class="font-bold text-lg">Scan Recipe</h3><form class="py-4" hx-post="/recipes/add/ocr" hx-encoding="multipart/form-data" hx-indicator="#fullscreen-loader" hx-swap="none" _="on submit call document.querySelector('#add-ocr-dialog').close()"><div class="grid mb-4"><label for="add-ocr-files-input" class="floating-label text-sm font-medium mb-1">Select your recipe's images ordered by page or a recipe document in the PDF format.</label><input id="add-ocr-files-input" type="file" name="files" accept=".jpg, .jpeg, .png, .bmp, .tiff, .heif, .pdf" multiple class="p-2 border border-gray-300 rounded-lg shadow focus:ring-2 focus:ring-purple-600 dark:bg-gray-900 dark:border-none"></div><button class="btn btn-block btn-primary btn-sm">Submit</button></form></div></dialog>"##,
                r##"<dialog class="modal" id="import-recipes-dialog"><div id="import-recipes-dialog-container" class="modal-box max-w-none w-96 max-h-[92vh] p-4 flex flex-col"><form method="dialog"><button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button></form><h3 class="font-bold text-lg">Import Recipes</h3><div class="tabs tabs-lift pt-4"><label class="tab"><input type="radio" name="import-recipe-tab" checked _="on click set #import-recipes-dialog-container.style.width to ''">Software</label><div class="tab-content bg-base-100 border-base-300 p-3"><form class="space-y-4" enctype="multipart/form-data" hx-post="/recipes/add/import/app" hx-indicator="#fullscreen-loader" hx-swap="none" hx-on:htmx:before-request="if(!this.checkValidity()) return false; document.querySelector('#import-recipes-dialog').close()"><div><fieldset class="fieldset"><legend class="fieldset-legend">Choose an application</legend><select class="select block" id="app-select" name="app"><option disabled selected>Pick an application</option><option value="AccuChef">AccuChef</option><option value="BigOven">BigOven</option><option value="ChefTap">ChefTap</option><option value="Computer Cuisine Deluxe">Computer Cuisine Deluxe</option><option value="CookBook">CookBook</option><option value="Cooklang">Cooklang</option><option value="COOKmate">COOKmate</option><option value="Cook'n">Cook'n</option><option value="Copy Me That">Copy Me That</option><option value="Crouton">Crouton</option><option value="Home Cookin">Home Cookin</option><option value="Kalorio">Kalorio</option><option value="Le Collectionneur de Recettes">Le Collectionneur de Recettes</option><option value="MasterCook">MasterCook</option><option value="MealMaster">MealMaster</option><option value="Mr. Cook">Mr. Cook</option><option value="My Recipe Box">My Recipe Box</option><option value="Paprika">Paprika</option><option value="Pepperplate">Pepperplate</option><option value="Recipe Keeper">Recipe Keeper</option><option value="RecipeMD">RecipeMD</option><option value="RecipeSage">RecipeSage</option><option value="Recipya">Recipya</option><option value="Rezkonv">Rezkonv</option><option value="Saffron">Saffron</option><option value="ShopNCook">ShopNCook</option><option value="Umami">Umami</option></select></fieldset><fieldset class="fieldset"><legend class="fieldset-legend">Select a file</legend><input id="import-dialog-file" type="file" name="file" required class="file-input w-full""##,
            ],
        );
    }
}
