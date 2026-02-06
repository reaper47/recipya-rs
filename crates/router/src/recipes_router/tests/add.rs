#[cfg(test)]
mod tests {
    use axum_test::TestResponse;
    use reqwest::Method;

    use testing::utils::{TestDb, assert_html, assert_must_be_logged_in, build_server_logged_in};

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
            &res,
            vec![
                r#"<title hx-swap-oob="true">Add Recipe | Recipya</title>"#,
                r#"<img class="object-cover w-full h-40 rounded-t-xl" src="/public/img/recipes/new/manual.webp" alt="Writing on a piece of paper with a traditional pen.">"#,
                r#"<img class="object-cover w-full h-40 rounded-t-xl" src="/public/img/recipes/new/import.webp" alt="Earth connected from end-to-end by telecommunications.">"#,
                r#"<img class="object-cover w-full h-40 rounded-t-xl" src="/public/img/recipes/new/camera.webp" alt="A cellphone used as a camera.">"#,
                r#"<img class="object-cover w-full h-40 rounded-t-xl" src="/public/img/recipes/new/schema.webp" alt="A bunch of shipping containers on a cargo boat.">"#,
                r##"<button class="underline" hx-get="/recipes/supported-websites" hx-target="#search-results" onclick="document.querySelector('#supported-websites-dialog').showModal()">supported</button>"##,
                r##"<dialog id="websites-dialog" class="modal"><div class="modal-box"><form method="dialog"><button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button></form><h3 class="font-bold text-lg">Fetch recipes from websites</h3><form class="py-4" hx-post="/recipes/add/website" hx-swap="none" _="on submit call #websites-dialog.close() then set me.querySelector('textarea').value to ''"><div class="grid mb-4"><label class="floating-label"><span>Enter one or more URLs, each on a new line.</span><textarea class="textarea w-full whitespace-pre-line" name="urls" rows="5" placeholder="URL 1"##,
                r#"</textarea></label></div><button class="btn btn-block btn-primary btn-sm">Submit</button></form></div></dialog>"#,
                r##"<dialog id="supported-websites-dialog" class="modal"><div class="modal-box h-2/3"><form method="dialog"><button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button></form><h3 class="mb-1"><label class="floating-label"><input type="search" placeholder="Search a website" class="input input-sm w-11/12" _="on input show <tbody>tr/> in next <table/> when its textContent.toLowerCase() contains my value.toLowerCase()"></label></h3><div class="overflow-x-auto"><table class="table table-zebra table-sm"><thead><tr class="text-center"><th class="py-1">Number</th><th class="py-1">Website</th></tr></thead><tbody id="search-results"></tbody></table></div></div></dialog>"##,
                r##"<dialog class="modal" id="supported-apps-import-dialog"><div class="modal-box h-2/3"><form method="dialog"><button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button></form><h3 class="mb-1"><label class="floating-label"><input type="search" placeholder="Search an application" class="input input-sm w-11/12" _="on input show <tbody>tr/> in next <table/> when its textContent.toLowerCase() contains my value.toLowerCase()"></label></h3><div class="overflow-x-auto"><table class="table table-zebra table-sm"><thead><tr class="text-center"><th class="py-1">Number</th><th class="py-1">Application</th><th class="py-1">File Formats</th></tr></thead><tbody id="application-results"></tbody></table></div></div></dialog>"##,
                r##"<dialog class="modal" id="add-ocr-dialog"><div class="modal-box"><form method="dialog"><button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button></form><h3 class="font-bold text-lg">Scan Recipe</h3><form class="py-4" hx-post="/recipes/add/ocr" hx-encoding="multipart/form-data" hx-indicator="#fullscreen-loader" hx-swap="none" _="on submit call document.querySelector('#add-ocr-dialog').close()"><div class="grid mb-4"><label for="add-ocr-files-input" class="floating-label text-sm font-medium mb-1">Select your recipe's images ordered by page or a recipe document in the PDF format.</label><input id="add-ocr-files-input" type="file" name="files" accept=".jpg, .jpeg, .png, .bmp, .tiff, .heif, .pdf" multiple class="p-2 border border-gray-300 rounded-lg shadow focus:ring-2 focus:ring-purple-600 dark:bg-gray-900 dark:border-none"></div><button class="btn btn-block btn-primary btn-sm">Submit</button></form></div></dialog>"##,
                r##"<dialog class="modal" id="import-recipes-dialog"><div id="import-recipes-dialog-container" class="modal-box w-[min(96vw,1100px)] max-h-[92vh] p-4 flex flex-col"><form method="dialog"><button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button></form><h3 class="font-bold text-lg">Import Recipes</h3><div class="tabs tabs-lift pt-4"><label class="tab"><input type="radio" name="import-recipe-tab" checked _="on click remove .max-w-none from #import-recipes-dialog-container">Software</label><div class="tab-content bg-base-100 border-base-300 p-3">"##,
            ],
        )
    }
}
