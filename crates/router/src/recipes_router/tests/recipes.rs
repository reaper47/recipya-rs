#[cfg(test)]
mod tests {
    use axum::http::Method;

    use models::{Recipe, settings::UserSettingDetails, user::User};
    use test_db::TestDb;
    use test_fixtures::{assert_html, assert_not_in_html};
    use test_models::a_complete_recipe_for_create;
    use test_utils::{assert_must_be_logged_in, build_server_logged_in, create_app_state};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

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
            &res,
            &[
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
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let mut all_images = Vec::with_capacity(3);
        for i in 0..3 {
            let (mut recipe, images) = a_complete_recipe_for_create();
            all_images.push(images);
            recipe.name.push_str(i.to_string().as_str());
            let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;
        }

        let res = server.get(BASE_URI).await;

        assert_html(
            &res,
            &[
                r#"<title hx-swap-oob="true">Recipes | Recipya</title>"#,
                r##"<form class="flex w-full" hx-get="/recipes/search" hx-vals="{&quot;page&quot;:1}" hx-target="#list-recipes" hx-swap="outerHTML" hx-push-url="true" hx-trigger="submit, change target:.sort-option"><div class="relative w-full"><label class="input input-sm flex justify-between px-0 gap-2 z-20 w-full"><button id="search-shortcut" type="button" class="pl-2" popovertarget="search-help" _="on click toggle .hidden on #search-help"><svg xmlns="http://www.w3.org/2000/svg" class="size-6 self-center hover:text-red-600" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg></button><input id="search-recipes" type="search" name="q" placeholder="Search for recipes..." autocomplete="off" value="""##,
                r#"<div class="dropdown dropdown-left ml-1"><div tabindex="0" role="button" class="btn btn-sm p-1"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25H12"></path></svg></div><div tabindex="0" class="dropdown-content z-10 menu menu-sm p-2 shadow bg-base-200 w-36 sm:menu-md prose"><h4 class="text-center underline">Sort</h4><fieldset class="fieldset flex"><label class="label cursor-pointer text-inherit" for="sort-opt-default"><input id="sort-opt-default" type="radio" name="sort" class="radio radio-sm sort-option" value="default"><span class="ml-1">Default</span></label></fieldset><fieldset class="fieldset flex"><label class="label cursor-pointer text-inherit" for="sort-opt-a-z"><input id="sort-opt-a-z" type="radio" name="sort" class="radio radio-sm sort-option" value="a-z"><span class="ml-1">Name:<br/>A to Z</span></label></fieldset><fieldset class="fieldset flex"><label class="label cursor-pointer text-inherit" for="sort-opt-z-a"><input id="sort-opt-z-a" type="radio" name="sort" class="radio radio-sm sort-option" value="z-a"><span class="ml-1">Name:<br/>Z to A</span></label></fieldset><fieldset class="fieldset flex"><label class="label cursor-pointer text-inherit" for="sort-opt-new-old"><input id="sort-opt-new-old" type="radio" name="sort" class="radio radio-sm sort-option" value="new-old"><span class="ml-1">Date created:<br/>Newest to oldest</span></label></fieldset><fieldset class="fieldset flex"><label class="label cursor-pointer text-inherit" for="sort-opt-old-new"><input id="sort-opt-old-new" type="radio" name="sort" class="radio radio-sm sort-option" value="old-new"><span class="ml-1">Date created:<br/>Oldest to newest</span></label></fieldset><fieldset class="fieldset flex"><label class="label cursor-pointer text-inherit" for="sort-opt-random"><input id="sort-opt-random" type="radio" name="sort" class="radio radio-sm sort-option" value="random"><span class="ml-1">Random</span></label></fieldset></div>"#,
                r#"<div id="list-recipes" class="min-h-0"><article id="list-recipes" class="grid gap-4 p-4 text-sm place-items-center grid-cols-1 sm:grid-cols-2 md:m-auto md:max-w-7xl md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 md:text-base">"#,
                r#"<div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex">"#,
                &format!(
                    r##"<section class="card-side card-compact card-border bg-base-200 shadow-lg indicator w-full h-full flex flex-col sm:card"><span class="hidden sm:block"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral indicator-item indicator-center" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{{&quot;q&quot;:&quot;cat:dinner&quot;}}" _="on click put &quot;cat:dinner&quot; into #search-recipes.value">dinner</span></span><figure class="relative cursor-pointer" hx-get="/recipes/1" hx-target="#content" hx-push-url="true" hx-trigger="mousedown" hx-swap="innerHTML show:window:top transition:true"><img class="h-28 w-24 object-cover rounded-t-lg sm:h-40 sm:min-w-full w-full" src="/data/images/thumbnails/{}.webp" alt="Image of the Best Chinese Kale0 recipe"><div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex"><p class="p-2 text-sm">This is the most delicious recipe!</p></div><button id="favourite-1" class="btn btn-square btn-sm rounded-md absolute top-2 right-2 cursor-default hover:text-secondary" title="Add to favourites" hx-post="/recipes/1/favourite" hx-target="#favourite-1" hx-swap="outerHTML" hx-push-url="false" hx-vals="{{&quot;view-recipe&quot;:false}}" aria-label="Add to favorites" aria-pressed="false" _="on mousedown halt the event"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12Z"></svg></button></figure><div class="card-body h-full flex flex-col gap-2"><h2 class="font-semibold line-clamp-2">Best Chinese Kale0</h2><div class="h-5"><div class="rating rating-sm"><div class="mask mask-star-2" aria-label="1 star"></div><div class="mask mask-star-2" aria-label="2 star"></div><div class="mask mask-star-2" aria-label="3 star"></div><div class="mask mask-star-2" aria-label="4 star" aria-current="true"></div><div class="mask mask-star-2" aria-label="5 star"></div></div></div><div class="max-h-16 overflow-y-auto"><div class="flex flex-col flex-wrap overflow-x-auto max-h-12 pb-2 sm:pb-0 sm:max-h-none sm:flex-auto sm:flex-row"><span class="sm:hidden"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{{&quot;q&quot;:&quot;cat:dinner&quot;}}" _="on click put &quot;cat:dinner&quot; into #search-recipes.value">dinner</span></span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{{&quot;q&quot;:&quot;\&quot;tag\&quot;: tofu&quot;}}" _="on click put 'tag:tofu' into #search-recipes.value">tofu</span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{{&quot;q&quot;:&quot;\&quot;tag\&quot;: vegetarian&quot;}}" _="on click put 'tag:vegetarian' into #search-recipes.value">vegetarian</span></div></div><div class="card-actions mt-auto"><button class="btn btn-block btn-xs btn-outline sm:btn-sm" hx-get="/recipes/1" hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true">View</button></div></div></section>"##,
                    all_images[0].main
                ),
                &format!(
                    r##"<section class="card-side card-compact card-border bg-base-200 shadow-lg indicator w-full h-full flex flex-col sm:card"><span class="hidden sm:block"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral indicator-item indicator-center" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{{&quot;q&quot;:&quot;cat:dinner&quot;}}" _="on click put &quot;cat:dinner&quot; into #search-recipes.value">dinner</span></span><figure class="relative cursor-pointer" hx-get="/recipes/2" hx-target="#content" hx-push-url="true" hx-trigger="mousedown" hx-swap="innerHTML show:window:top transition:true"><img class="h-28 w-24 object-cover rounded-t-lg sm:h-40 sm:min-w-full w-full" src="/data/images/thumbnails/{}.webp" alt="Image of the Best Chinese Kale1 recipe"><div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex"><p class="p-2 text-sm">This is the most delicious recipe!</p></div><button id="favourite-2" class="btn btn-square btn-sm rounded-md absolute top-2 right-2 cursor-default hover:text-secondary" title="Add to favourites" hx-post="/recipes/2/favourite" hx-target="#favourite-2" hx-swap="outerHTML" hx-push-url="false" hx-vals="{{&quot;view-recipe&quot;:false}}" aria-label="Add to favorites" aria-pressed="false" _="on mousedown halt the event"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12Z"></svg></button></figure><div class="card-body h-full flex flex-col gap-2"><h2 class="font-semibold line-clamp-2">Best Chinese Kale1</h2><div class="h-5"><div class="rating rating-sm"><div class="mask mask-star-2" aria-label="1 star"></div><div class="mask mask-star-2" aria-label="2 star"></div><div class="mask mask-star-2" aria-label="3 star"></div><div class="mask mask-star-2" aria-label="4 star" aria-current="true"></div><div class="mask mask-star-2" aria-label="5 star"></div></div></div><div class="max-h-16 overflow-y-auto"><div class="flex flex-col flex-wrap overflow-x-auto max-h-12 pb-2 sm:pb-0 sm:max-h-none sm:flex-auto sm:flex-row"><span class="sm:hidden"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{{&quot;q&quot;:&quot;cat:dinner&quot;}}" _="on click put &quot;cat:dinner&quot; into #search-recipes.value">dinner</span></span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{{&quot;q&quot;:&quot;\&quot;tag\&quot;: tofu&quot;}}" _="on click put 'tag:tofu' into #search-recipes.value">tofu</span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{{&quot;q&quot;:&quot;\&quot;tag\&quot;: vegetarian&quot;}}" _="on click put 'tag:vegetarian' into #search-recipes.value">vegetarian</span></div></div><div class="card-actions mt-auto"><button class="btn btn-block btn-xs btn-outline sm:btn-sm" hx-get="/recipes/2" hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true">View</button></div></div></section>"##,
                    all_images[1].main
                ),
                &format!(
                    r##"<section class="card-side card-compact card-border bg-base-200 shadow-lg indicator w-full h-full flex flex-col sm:card"><span class="hidden sm:block"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral indicator-item indicator-center" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{{&quot;q&quot;:&quot;cat:dinner&quot;}}" _="on click put &quot;cat:dinner&quot; into #search-recipes.value">dinner</span></span><figure class="relative cursor-pointer" hx-get="/recipes/3" hx-target="#content" hx-push-url="true" hx-trigger="mousedown" hx-swap="innerHTML show:window:top transition:true"><img class="h-28 w-24 object-cover rounded-t-lg sm:h-40 sm:min-w-full w-full" src="/data/images/thumbnails/{}.webp" alt="Image of the Best Chinese Kale2 recipe"><div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex"><p class="p-2 text-sm">This is the most delicious recipe!</p></div><button id="favourite-3" class="btn btn-square btn-sm rounded-md absolute top-2 right-2 cursor-default hover:text-secondary" title="Add to favourites" hx-post="/recipes/3/favourite" hx-target="#favourite-3" hx-swap="outerHTML" hx-push-url="false" hx-vals="{{&quot;view-recipe&quot;:false}}" aria-label="Add to favorites" aria-pressed="false" _="on mousedown halt the event"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12Z"></svg></button></figure><div class="card-body h-full flex flex-col gap-2"><h2 class="font-semibold line-clamp-2">Best Chinese Kale2</h2><div class="h-5"><div class="rating rating-sm"><div class="mask mask-star-2" aria-label="1 star"></div><div class="mask mask-star-2" aria-label="2 star"></div><div class="mask mask-star-2" aria-label="3 star"></div><div class="mask mask-star-2" aria-label="4 star" aria-current="true"></div><div class="mask mask-star-2" aria-label="5 star"></div></div></div><div class="max-h-16 overflow-y-auto"><div class="flex flex-col flex-wrap overflow-x-auto max-h-12 pb-2 sm:pb-0 sm:max-h-none sm:flex-auto sm:flex-row"><span class="sm:hidden"><span class="badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{{&quot;q&quot;:&quot;cat:dinner&quot;}}" _="on click put &quot;cat:dinner&quot; into #search-recipes.value">dinner</span></span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{{&quot;q&quot;:&quot;\&quot;tag\&quot;: tofu&quot;}}" _="on click put 'tag:tofu' into #search-recipes.value">tofu</span><span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer" hx-get="/recipes/search" hx-target="#list-recipes" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" hx-vals="{{&quot;q&quot;:&quot;\&quot;tag\&quot;: vegetarian&quot;}}" _="on click put 'tag:vegetarian' into #search-recipes.value">vegetarian</span></div></div><div class="card-actions mt-auto"><button class="btn btn-block btn-xs btn-outline sm:btn-sm" hx-get="/recipes/3" hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true">View</button></div></div></section>"##,
                    all_images[2].main
                ),
                r#"<footer id="pagination-recipes" class="footer footer-center bg-base-200 p-2 gap-2 md:pb-2 mt-auto shrink-0" style="grid-auto-flow: row;" onload="updateAddCookbookUrl(1)"><div class="join gap-0"><button class="join-item btn btn-disabled btn-xs md:btn-sm w-8 md:w-12" title="Previous page" aria-label="Previous page">‹</button><button class="join-item btn btn-active btn-xs md:btn-sm w-8 md:w-12" aria-current="page" aria-label="Page 1, current page">1</button><button class="join-item btn btn-disabled btn-xs md:btn-sm w-8 md:w-12" title="Next page" aria-label="Next page">›</button></div><div class="text-center"><p class="text-xs md:text-sm">Showing <span class="font-semibold text-base-content">1</span>-<span class="font-semibold text-base-content">3</span> of <span id="search-count" class="font-medium">3</span> results</p></div></footer>"#,
            ],
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_get_recipe_has_no_rating() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let server = build_server_logged_in(config.clone()).await?;
        let state = create_app_state(config).await;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (mut recipe, _) = a_complete_recipe_for_create();
        recipe.rating = None;
        let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

        let res = server.get(BASE_URI).await;

        let _ = assert_not_in_html(
            &res,
            &[
                r#"<div class="rating rating-sm"><div class="mask mask-star-2" aria-label="1 star"></div>"#,
            ],
        );
        Ok(())
    }
}
