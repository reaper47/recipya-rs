#[cfg(test)]
mod tests {
    use axum::http::Method;
    use axum_test::TestServer;

    use config::Config;
    use models::params::SearchParams;
    use models::user::User;
    use models::{Recipe, recipe::structs::test_utils::a_complete_recipe_for_create};
    use repository::ModelManager;
    use testing::utils::{
        TestDb, assert_html, assert_must_be_logged_in, build_server_logged_in, create_app_state,
    };
    use uuid::Uuid;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    const BASE_URI: &str = "/recipes/search";

    async fn prepare_server_with_recipes(config: Config) -> Result<TestServer> {
        let state = create_app_state(config.clone()).await;
        let mut server = build_server_logged_in(config).await?;
        let users = User::all(&state.mm).await?;
        let user_id = users[0].id;
        server.add_header(axum_htmx::HX_REQUEST, "true");
        insert_recipes(&state.mm, user_id).await?;
        Ok(server)
    }

    async fn insert_recipes(mm: &ModelManager, user_id: Uuid) -> Result<()> {
        let mut recipe1 = a_complete_recipe_for_create();
        recipe1.name = "Chinese Firmware".to_string();
        let mut recipe2 = a_complete_recipe_for_create();
        recipe2.name = "Lovely Canada".to_string();
        let mut recipe3 = a_complete_recipe_for_create();
        recipe3.name = "Lovely Ukraine".to_string();
        let mut recipe4 = a_complete_recipe_for_create();
        recipe4.name = "Magic Potion".to_string();
        recipe4.is_favourite = true;
        for recipe in [recipe1, recipe2, recipe3, recipe4] {
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
        let server = prepare_server_with_recipes(config).await?;

        let res = server
            .get(BASE_URI)
            .add_query_params(SearchParams {
                q: Some("lovely".to_string()),
                page: Some(0),
                ..Default::default()
            })
            .await;

        res.assert_status_ok();
        assert_html(
            &res,
            vec![
                r#"<h2 class="font-semibold line-clamp-2">Lovely Ukraine</h2>"#,
                r#"<h2 class="font-semibold line-clamp-2">Lovely Canada</h2>"#,
                r#"<button class="join-item btn btn-disabled w-12" title="Previous page" aria-label="Previous page">‹</button><button class="join-item btn btn-active w-12" aria-current="page" aria-label="Page 1, current page">1</button><button class="join-item btn btn-disabled w-12" title="Next page" aria-label="Next page">›</button>"#,
            ],
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_empty_query_redirects_to_recipes_index() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let server = prepare_server_with_recipes(config).await?;

        let res = server
            .get(BASE_URI)
            .add_query_params(SearchParams {
                q: Some(String::new()),
                page: Some(0),
                ..Default::default()
            })
            .await;

        res.assert_status_ok();
        assert_html(
            &res,
            vec![
                r#"<h2 class="font-semibold line-clamp-2">Chinese Firmware</h2>"#,
                r#"<h2 class="font-semibold line-clamp-2">Lovely Ukraine</h2>"#,
                r#"<h2 class="font-semibold line-clamp-2">Lovely Canada</h2>"#,
                r#"<button class="join-item btn btn-disabled w-12" title="Previous page" aria-label="Previous page">‹</button><button class="join-item btn btn-active w-12" aria-current="page" aria-label="Page 1, current page">1</button><button class="join-item btn btn-disabled w-12" title="Next page" aria-label="Next page">›</button>"#,
            ],
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_no_results() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let server = prepare_server_with_recipes(config).await?;

        let res = server
            .get(BASE_URI)
            .add_query_params(SearchParams {
                q: Some("kool-aid".to_string()),
                page: Some(0),
                ..Default::default()
            })
            .await;

        res.assert_status_ok();
        assert_html(
            &res,
            vec![
                r#"<div id="list-recipes" class="grid place-content-center text-sm text-center h-3/5 md:text-base"><p class="pt-2">No results found.</p></div>"#,
            ],
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_results_query1() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let server = prepare_server_with_recipes(config).await?;

        let res = server
            .get(BASE_URI)
            .add_query_params(SearchParams {
                q: Some("love".to_string()),
                page: Some(0),
                ..Default::default()
            })
            .await;

        res.assert_status_ok();
        assert_html(
            &res,
            vec![
                r##"<section class="card-side card-compact card-border bg-base-200 shadow-lg indicator w-full h-full flex flex-col sm:card"><span class="hidden sm:block"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral indicator-item indicator-center" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{&quot;q&quot;:&quot;\&quot;cat\&quot;: dinner&quot;}" _="on click put &quot;cat:dinner&quot; into #search-recipes.value">dinner</span></span><figure class="relative cursor-pointer" hx-get="/recipes/2" hx-target="#content" hx-push-url="true" hx-trigger="mousedown" hx-swap="innerHTML show:window:top transition:true"><img class="h-28 w-24 object-cover rounded-t-lg sm:h-40 sm:min-w-full sm:w-full" src="/data/images/Placeholders/placeholder.recipe.webp" alt="Image of the Lovely Canada recipe"><div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex"><p class="p-2 text-sm">This is the most delicious recipe!</p></div><button id="favourite-2" class="btn btn-square btn-sm rounded-md absolute top-2 right-2 cursor-default hover:text-secondary" title="Add to favourites" hx-post="/recipes/2/favourite" hx-target="#favourite-2" hx-swap="outerHTML" hx-push-url="false" hx-vals="{&quot;view-recipe&quot;:false}" aria-label="Add to favorites" aria-pressed="false" _="on mousedown halt the event"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12Z"></svg></button></figure><div class="card-body h-full flex flex-col gap-2"><h2 class="font-semibold line-clamp-2">Lovely Canada</h2><div class="h-5"><div class="rating rating-sm"><div class="mask mask-star-2" aria-label="1 star"></div><div class="mask mask-star-2" aria-label="2 star"></div><div class="mask mask-star-2" aria-label="3 star"></div><div class="mask mask-star-2" aria-label="4 star" aria-current="true"></div><div class="mask mask-star-2" aria-label="5 star"></div></div></div><div class="max-h-16 overflow-y-auto"><div class="flex flex-col flex-wrap overflow-x-auto max-h-12 pb-2 sm:pb-0 sm:max-h-none sm:flex-auto sm:flex-row"><span class="sm:hidden"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{&quot;q&quot;:&quot;\&quot;cat\&quot;: dinner&quot;}" _="on click put &quot;cat:dinner&quot; into #search-recipes.value">dinner</span></span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{&quot;q&quot;:&quot;\&quot;tag\&quot;: tofu&quot;}" _="on click put 'tag:tofu' into #search-recipes.value">tofu</span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{&quot;q&quot;:&quot;\&quot;tag\&quot;: vegetarian&quot;}" _="on click put 'tag:vegetarian' into #search-recipes.value">vegetarian</span></div></div><div class="card-actions mt-auto"><button class="btn btn-block btn-xs btn-outline sm:btn-sm" hx-get="/recipes/2" hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true">View</button></div></div></section>"##,
                r##"<section class="card-side card-compact card-border bg-base-200 shadow-lg indicator w-full h-full flex flex-col sm:card"><span class="hidden sm:block"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral indicator-item indicator-center" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{&quot;q&quot;:&quot;\&quot;cat\&quot;: dinner&quot;}" _="on click put &quot;cat:dinner&quot; into #search-recipes.value">dinner</span></span><figure class="relative cursor-pointer" hx-get="/recipes/3" hx-target="#content" hx-push-url="true" hx-trigger="mousedown" hx-swap="innerHTML show:window:top transition:true"><img class="h-28 w-24 object-cover rounded-t-lg sm:h-40 sm:min-w-full sm:w-full" src="/data/images/Placeholders/placeholder.recipe.webp" alt="Image of the Lovely Ukraine recipe"><div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex"><p class="p-2 text-sm">This is the most delicious recipe!</p></div><button id="favourite-3" class="btn btn-square btn-sm rounded-md absolute top-2 right-2 cursor-default hover:text-secondary" title="Add to favourites" hx-post="/recipes/3/favourite" hx-target="#favourite-3" hx-swap="outerHTML" hx-push-url="false" hx-vals="{&quot;view-recipe&quot;:false}" aria-label="Add to favorites" aria-pressed="false" _="on mousedown halt the event"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12Z"></svg></button></figure><div class="card-body h-full flex flex-col gap-2"><h2 class="font-semibold line-clamp-2">Lovely Ukraine</h2><div class="h-5"><div class="rating rating-sm"><div class="mask mask-star-2" aria-label="1 star"></div><div class="mask mask-star-2" aria-label="2 star"></div><div class="mask mask-star-2" aria-label="3 star"></div><div class="mask mask-star-2" aria-label="4 star" aria-current="true"></div><div class="mask mask-star-2" aria-label="5 star"></div></div></div><div class="max-h-16 overflow-y-auto"><div class="flex flex-col flex-wrap overflow-x-auto max-h-12 pb-2 sm:pb-0 sm:max-h-none sm:flex-auto sm:flex-row"><span class="sm:hidden"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{&quot;q&quot;:&quot;\&quot;cat\&quot;: dinner&quot;}" _="on click put &quot;cat:dinner&quot; into #search-recipes.value">dinner</span></span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{&quot;q&quot;:&quot;\&quot;tag\&quot;: tofu&quot;}" _="on click put 'tag:tofu' into #search-recipes.value">tofu</span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{&quot;q&quot;:&quot;\&quot;tag\&quot;: vegetarian&quot;}" _="on click put 'tag:vegetarian' into #search-recipes.value">vegetarian</span></div></div><div class="card-actions mt-auto"><button class="btn btn-block btn-xs btn-outline sm:btn-sm" hx-get="/recipes/3" hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true">View</button></div></div></section>"##,
            ],
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_results_query2() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let server = prepare_server_with_recipes(config).await?;

        let res = server
            .get(BASE_URI)
            .add_query_params(SearchParams {
                q: Some("Chinese".to_string()),
                page: Some(0),
                ..Default::default()
            })
            .await;

        res.assert_status_ok();
        assert_html(
            &res,
            vec![
                r##"<section class="card-side card-compact card-border bg-base-200 shadow-lg indicator w-full h-full flex flex-col sm:card"><span class="hidden sm:block"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral indicator-item indicator-center" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{&quot;q&quot;:&quot;\&quot;cat\&quot;: dinner&quot;}" _="on click put &quot;cat:dinner&quot; into #search-recipes.value">dinner</span></span><figure class="relative cursor-pointer" hx-get="/recipes/1" hx-target="#content" hx-push-url="true" hx-trigger="mousedown" hx-swap="innerHTML show:window:top transition:true"><img class="h-28 w-24 object-cover rounded-t-lg sm:h-40 sm:min-w-full sm:w-full" src="/data/images/Placeholders/placeholder.recipe.webp" alt="Image of the Chinese Firmware recipe"><div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex"><p class="p-2 text-sm">This is the most delicious recipe!</p></div><button id="favourite-1" class="btn btn-square btn-sm rounded-md absolute top-2 right-2 cursor-default hover:text-secondary" title="Add to favourites" hx-post="/recipes/1/favourite" hx-target="#favourite-1" hx-swap="outerHTML" hx-push-url="false" hx-vals="{&quot;view-recipe&quot;:false}" aria-label="Add to favorites" aria-pressed="false" _="on mousedown halt the event"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12Z"></svg></button></figure><div class="card-body h-full flex flex-col gap-2"><h2 class="font-semibold line-clamp-2">Chinese Firmware</h2><div class="h-5"><div class="rating rating-sm"><div class="mask mask-star-2" aria-label="1 star"></div><div class="mask mask-star-2" aria-label="2 star"></div><div class="mask mask-star-2" aria-label="3 star"></div><div class="mask mask-star-2" aria-label="4 star" aria-current="true"></div><div class="mask mask-star-2" aria-label="5 star"></div></div></div><div class="max-h-16 overflow-y-auto"><div class="flex flex-col flex-wrap overflow-x-auto max-h-12 pb-2 sm:pb-0 sm:max-h-none sm:flex-auto sm:flex-row"><span class="sm:hidden"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{&quot;q&quot;:&quot;\&quot;cat\&quot;: dinner&quot;}" _="on click put &quot;cat:dinner&quot; into #search-recipes.value">dinner</span></span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{&quot;q&quot;:&quot;\&quot;tag\&quot;: tofu&quot;}" _="on click put 'tag:tofu' into #search-recipes.value">tofu</span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{&quot;q&quot;:&quot;\&quot;tag\&quot;: vegetarian&quot;}" _="on click put 'tag:vegetarian' into #search-recipes.value">vegetarian</span></div></div><div class="card-actions mt-auto"><button class="btn btn-block btn-xs btn-outline sm:btn-sm" hx-get="/recipes/1" hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true">View</button></div></div></section>"##,
            ],
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_results_query3() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let server = prepare_server_with_recipes(config).await?;

        let res = server
            .get(BASE_URI)
            .add_query_params(SearchParams {
                q: Some("LOVELY".to_string()),
                page: Some(0),
                ..Default::default()
            })
            .await;

        res.assert_status_ok();
        assert_html(
            &res,
            vec![
                r##"<section class="card-side card-compact card-border bg-base-200 shadow-lg indicator w-full h-full flex flex-col sm:card"><span class="hidden sm:block"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral indicator-item indicator-center" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{&quot;q&quot;:&quot;\&quot;cat\&quot;: dinner&quot;}" _="on click put &quot;cat:dinner&quot; into #search-recipes.value">dinner</span></span><figure class="relative cursor-pointer" hx-get="/recipes/2" hx-target="#content" hx-push-url="true" hx-trigger="mousedown" hx-swap="innerHTML show:window:top transition:true"><img class="h-28 w-24 object-cover rounded-t-lg sm:h-40 sm:min-w-full sm:w-full" src="/data/images/Placeholders/placeholder.recipe.webp" alt="Image of the Lovely Canada recipe"><div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex"><p class="p-2 text-sm">This is the most delicious recipe!</p></div><button id="favourite-2" class="btn btn-square btn-sm rounded-md absolute top-2 right-2 cursor-default hover:text-secondary" title="Add to favourites" hx-post="/recipes/2/favourite" hx-target="#favourite-2" hx-swap="outerHTML" hx-push-url="false" hx-vals="{&quot;view-recipe&quot;:false}" aria-label="Add to favorites" aria-pressed="false" _="on mousedown halt the event"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12Z"></svg></button></figure><div class="card-body h-full flex flex-col gap-2"><h2 class="font-semibold line-clamp-2">Lovely Canada</h2><div class="h-5"><div class="rating rating-sm"><div class="mask mask-star-2" aria-label="1 star"></div><div class="mask mask-star-2" aria-label="2 star"></div><div class="mask mask-star-2" aria-label="3 star"></div><div class="mask mask-star-2" aria-label="4 star" aria-current="true"></div><div class="mask mask-star-2" aria-label="5 star"></div></div></div><div class="max-h-16 overflow-y-auto"><div class="flex flex-col flex-wrap overflow-x-auto max-h-12 pb-2 sm:pb-0 sm:max-h-none sm:flex-auto sm:flex-row"><span class="sm:hidden"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{&quot;q&quot;:&quot;\&quot;cat\&quot;: dinner&quot;}" _="on click put &quot;cat:dinner&quot; into #search-recipes.value">dinner</span></span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{&quot;q&quot;:&quot;\&quot;tag\&quot;: tofu&quot;}" _="on click put 'tag:tofu' into #search-recipes.value">tofu</span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{&quot;q&quot;:&quot;\&quot;tag\&quot;: vegetarian&quot;}" _="on click put 'tag:vegetarian' into #search-recipes.value">vegetarian</span></div></div><div class="card-actions mt-auto"><button class="btn btn-block btn-xs btn-outline sm:btn-sm" hx-get="/recipes/2" hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true">View</button></div></div></section>"##,
                r##"<section class="card-side card-compact card-border bg-base-200 shadow-lg indicator w-full h-full flex flex-col sm:card"><span class="hidden sm:block"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral indicator-item indicator-center" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{&quot;q&quot;:&quot;\&quot;cat\&quot;: dinner&quot;}" _="on click put &quot;cat:dinner&quot; into #search-recipes.value">dinner</span></span><figure class="relative cursor-pointer" hx-get="/recipes/3" hx-target="#content" hx-push-url="true" hx-trigger="mousedown" hx-swap="innerHTML show:window:top transition:true"><img class="h-28 w-24 object-cover rounded-t-lg sm:h-40 sm:min-w-full sm:w-full" src="/data/images/Placeholders/placeholder.recipe.webp" alt="Image of the Lovely Ukraine recipe"><div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex"><p class="p-2 text-sm">This is the most delicious recipe!</p></div><button id="favourite-3" class="btn btn-square btn-sm rounded-md absolute top-2 right-2 cursor-default hover:text-secondary" title="Add to favourites" hx-post="/recipes/3/favourite" hx-target="#favourite-3" hx-swap="outerHTML" hx-push-url="false" hx-vals="{&quot;view-recipe&quot;:false}" aria-label="Add to favorites" aria-pressed="false" _="on mousedown halt the event"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12Z"></svg></button></figure><div class="card-body h-full flex flex-col gap-2"><h2 class="font-semibold line-clamp-2">Lovely Ukraine</h2><div class="h-5"><div class="rating rating-sm"><div class="mask mask-star-2" aria-label="1 star"></div><div class="mask mask-star-2" aria-label="2 star"></div><div class="mask mask-star-2" aria-label="3 star"></div><div class="mask mask-star-2" aria-label="4 star" aria-current="true"></div><div class="mask mask-star-2" aria-label="5 star"></div></div></div><div class="max-h-16 overflow-y-auto"><div class="flex flex-col flex-wrap overflow-x-auto max-h-12 pb-2 sm:pb-0 sm:max-h-none sm:flex-auto sm:flex-row"><span class="sm:hidden"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{&quot;q&quot;:&quot;\&quot;cat\&quot;: dinner&quot;}" _="on click put &quot;cat:dinner&quot; into #search-recipes.value">dinner</span></span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{&quot;q&quot;:&quot;\&quot;tag\&quot;: tofu&quot;}" _="on click put 'tag:tofu' into #search-recipes.value">tofu</span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{&quot;q&quot;:&quot;\&quot;tag\&quot;: vegetarian&quot;}" _="on click put 'tag:vegetarian' into #search-recipes.value">vegetarian</span></div></div><div class="card-actions mt-auto"><button class="btn btn-block btn-xs btn-outline sm:btn-sm" hx-get="/recipes/3" hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true">View</button></div></div></section>"##,
            ],
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_empty_query_only_favourites_test() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let server = prepare_server_with_recipes(config).await?;

        let res = server
            .get(BASE_URI)
            .add_query_params(SearchParams {
                is_favourites: Some(true),
                ..Default::default()
            })
            .await;

        res.assert_status_ok();
        assert_html(
            &res,
            vec![r#"<h2 class="font-semibold line-clamp-2">Magic Potion</h2>"#],
        );
        Ok(())
    }
}
