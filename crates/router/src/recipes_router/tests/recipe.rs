#[cfg(test)]
mod tests {
    use axum::http::Method;
    use axum::http::{HeaderValue, StatusCode};
    use axum_test::TestResponse;
    use uuid::Uuid;

    use config::Config;
    use models::Recipe;
    use models::recipe::structs::media::VideoForCreate;
    use models::recipe::structs::recipe::RecipeForCreate;
    use models::recipe::structs::types::Source;
    use models::settings::UserSettingDetails;
    use models::user::User;
    use test_db::TestDb;
    use test_fixtures::{RecipeImages, assert_html, assert_not_in_html, assert_ws_message};
    use test_models::a_complete_recipe_for_create;
    use test_utils::{
        assert_must_be_logged_in, build_server_logged_in, build_server_ws, create_app_state,
    };

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    fn base_uri(recipe_id: i64) -> String {
        format!("/recipes/{recipe_id}")
    }

    mod tests_get {
        use models::recipe::structs::section::{Item, SectionComponents};

        use super::*;

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, &base_uri(1)).await
        }

        #[tokio::test]
        async fn test_recipe_exists_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let settings = UserSettingDetails::get(&state.mm, user_id).await?;
            let (recipe, images) = a_complete_recipe_for_create();
            let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_ok();
            assert_complete_recipe(&res, &recipe, &images);
            Ok(())
        }

        #[tokio::test]
        async fn test_recipe_exists_flat_ingredients_instructions_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let settings = UserSettingDetails::get(&state.mm, user_id).await?;
            let (mut recipe, _) = a_complete_recipe_for_create();
            recipe.ingredients =
                SectionComponents::Flat(vec![Item::new("ing1"), Item::new("ing2")]);
            recipe.instructions =
                SectionComponents::Flat(vec![Item::new("ins1"), Item::new("ins2")]);
            let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    r#"<div class="col-span-6 px-8 py-2 border-gray-700 md:rounded-bl-none md:col-span-4 print:hidden"><h2 class="font-semibold text-center underline pb-1">Instructions</h2><ol class="grid list-decimal"><li class="min-w-full py-2 select-none hover:bg-base-300" _="on mousedown toggle .line-through toggle .opacity-40 then if I match .line-through then add .invisible to .timer in me else remove .invisible from .timer in me"><div class="flex"><div class="whitespace-pre-line w-full transition-all">ins1</div></div></li><li class="min-w-full py-2 select-none hover:bg-base-300" _="on mousedown toggle .line-through toggle .opacity-40 then if I match .line-through then add .invisible to .timer in me else remove .invisible from .timer in me"><div class="flex"><div class="whitespace-pre-line w-full transition-all">ins2</div></div></li></ol></div>"#,
                    r#"<h1 class="text-sm print:mb-1"><b>Ingredients</b></h1><ul class="col-span-6 w-full print:mb-2" style="column-count: 1"><li class="text-sm"><label><input type="checkbox"></label><span class="pl-2">ing1</span></li><li class="text-sm"><label><input type="checkbox"></label><span class="pl-2">ing2</span></li></ul></div>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_recipe_exists_from_htmx_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let mut server = build_server_logged_in(config.clone()).await?;
            server.add_header(axum_htmx::HX_REQUEST, HeaderValue::from_static("true"));
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let settings = UserSettingDetails::get(&state.mm, user_id).await?;
            let (recipe, images) = a_complete_recipe_for_create();
            let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_ok();
            assert_complete_recipe(&res, &recipe, &images);
            Ok(())
        }

        #[tokio::test]
        async fn test_recipe_not_exist_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&base_uri(999)).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    r#"<title hx-swap-oob="true">Recipe Not Found | Recipya</title>"#,
                    "Recipe Not Found",
                    "The recipe you requested to view is not found.",
                ],
            );
            Ok(())
        }

        #[tracing_test::traced_test]
        #[tokio::test]
        async fn test_recipe_no_media_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let settings = UserSettingDetails::get(&state.mm, user_id).await?;
            let (mut recipe, _) = a_complete_recipe_for_create();
            recipe.videos.clear();
            recipe.images = vec![];
            let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    r#"<img style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src="/data/images/Placeholders/placeholder.recipe.webp">"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_recipe_1_image_0_video_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let settings = UserSettingDetails::get(&state.mm, user_id).await?;
            let (mut recipe, _) = a_complete_recipe_for_create();
            recipe.videos.clear();
            let img1 = Uuid::new_v4();
            recipe.images = vec![img1];
            let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[&format!(
                    "<img id=\"output\" style=\"object-fit: cover\" alt=\"Image of the recipe\" class=\"w-full max-h-80 md:max-h-[34rem]\" src=\"/data/images/{img1}.webp\">"
                )],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_recipe_2_images_0_video_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let settings = UserSettingDetails::get(&state.mm, user_id).await?;
            let (mut recipe, images) = a_complete_recipe_for_create();
            recipe.videos.clear();
            recipe.images = vec![images.main, images.additional];
            let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    &format!(
                        r##"<img style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src="/data/images/{}.webp"><div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 bottom-0"><a class="btn btn-soft btn-sm" href="#media-1">❮</a><a class="btn btn-soft btn-sm" href="#media-1">❯</a></div></div><div id="media-1" class="carousel-item relative w-full">"##,
                        images.main
                    ),
                    &format!(
                        r##"<img style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src="/data/images/{}.webp"><div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 bottom-0"><a class="btn btn-soft btn-sm" href="#media-0">❮</a><a class="btn btn-soft btn-sm" href="#media-0">❯</a></div>"##,
                        images.additional
                    ),
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_recipe_0_images_1_video_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let settings = UserSettingDetails::get(&state.mm, user_id).await?;
            let (mut recipe, _) = a_complete_recipe_for_create();
            recipe.videos = vec![VideoForCreate {
                video: Uuid::new_v4(),
                duration: None,
                content_url: Some("https://example.com/embed/yg8FG4".into()),
                embed_url: None,
            }];
            recipe.images = vec![];
            let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    r#"<video controls preload="metadata" src="https://example.com/embed/yg8FG4"></video>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_recipe_0_images_2_videos_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let settings = UserSettingDetails::get(&state.mm, user_id).await?;
            let (mut recipe, _) = a_complete_recipe_for_create();
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
            let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    r##"<video controls preload="metadata" src="https://example.com/embed/yg8FG4"></video><div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 bottom-0"><a class="btn btn-soft btn-sm" href="#media-1">❮</a><a class="btn btn-soft btn-sm" href="#media-1">❯</a></div></div><div id="media-1" class="carousel-item relative w-full"><iframe src="https://example.com/embed/yg8FG4" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen="" style="height: 100%;width: 100%;"></iframe><div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 bottom-0"><a class="btn btn-soft btn-sm" href="#media-0">❮</a><a class="btn btn-soft btn-sm" href="#media-0">❯</a></div></div>"##,
                    r#"<div id="media-1" class="carousel-item relative w-full"><iframe src="https://example.com/embed/yg8FG4" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen="" style="height: 100%;width: 100%;"></iframe>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_recipe_2_images_2_videos_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let settings = UserSettingDetails::get(&state.mm, user_id).await?;
            let (mut recipe, images) = a_complete_recipe_for_create();
            recipe.videos = vec![
                VideoForCreate {
                    video: images.video,
                    duration: None,
                    content_url: None,
                    embed_url: None,
                },
                VideoForCreate {
                    video: Uuid::nil(),
                    duration: None,
                    content_url: None,
                    embed_url: Some("https://example.com/embed/yg8FG4".into()),
                },
            ];
            let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    &format!(
                        r##"<div class="carousel w-full"><div id="media-0" class="carousel-item relative w-full"><img style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src="/data/images/{}.webp"><div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 bottom-0"><a class="btn btn-soft btn-sm" href="#media-3">❮</a><a class="btn btn-soft btn-sm" href="#media-1">❯</a></div></div>"##,
                        images.main
                    ),
                    &format!(
                        r##"<div id="media-1" class="carousel-item relative w-full"><img style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src="/data/images/{}.webp"><div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 bottom-0"><a class="btn btn-soft btn-sm" href="#media-0">❮</a><a class="btn btn-soft btn-sm" href="#media-2">❯</a></div></div>"##,
                        images.additional
                    ),
                    &format!(
                        r##"<div id="media-2" class="carousel-item relative w-full"><video controls preload="metadata" src="/data/videos/{}.webm" type="video/webm"></video><div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 bottom-0"><a class="btn btn-soft btn-sm" href="#media-1">❮</a><a class="btn btn-soft btn-sm" href="#media-3">❯</a></div>"##,
                        images.video
                    ),
                    r##"<div id="media-3" class="carousel-item relative w-full"><iframe src="https://example.com/embed/yg8FG4" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen="" style="height: 100%;width: 100%;"></iframe><div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 bottom-0"><a class="btn btn-soft btn-sm" href="#media-2">❮</a><a class="btn btn-soft btn-sm" href="#media-0">❯</a></div></div>"##,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_source_is_not_link() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let settings = UserSettingDetails::get(&state.mm, user_id).await?;
            let (mut recipe, _) = a_complete_recipe_for_create();
            recipe.source = Source::new("My mom's recipe cookbook");
            let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_ok();
            assert_not_in_html(
                &res,
                &[
                    r##"<li _="on click document.activeElement.blur()"><button hx-get="/recipes/1/rescrape" hx-target="#content" hx-swap="none" hx-indicator="#fullscreen-loader"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 21a9.004 9.004 0 0 0 8.716-6.747M12 21a9.004 9.004 0 0 1-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 0 1 7.843 4.582M12 3a8.997 8.997 0 0 0-7.843 4.582m15.686 0A11.953 11.953 0 0 1 12 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0 1 21 12c0 .778-.099 1.533-.284 2.253m0 0A17.919 17.919 0 0 1 12 16.5c-3.162 0-6.133-.815-8.716-2.247m0 0A9.015 9.015 0 0 1 3 12c0-1.605.42-3.113 1.157-4.418"></path></svg>Rescrape</button></li>"##,
                ],
            )?;
            Ok(())
        }

        #[tokio::test]
        async fn test_get_recipe_bold_enabled_ok() -> Result<()> {
            let config = Some(Config::default());
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let users = User::all(&state.mm).await?;
            users[0].update_bold_ingredients(&state.mm, true).await?;
            let settings = UserSettingDetails::get(&state.mm, users[0].id).await?;
            let (mut recipe, _) = a_complete_recipe_for_create();
            recipe.ingredients = SectionComponents::Flat(vec![
                Item::new("1 cup butter, softened"),
                Item::new("1 cup white sugar"),
                Item::new("1 cup packed brown sugar"),
            ]);
            recipe.instructions = SectionComponents::Flat(vec![Item::new(
                "Preheat the oven to 350 degrees F (175 degrees C). Beat butter, white sugar, and brown sugar together in a large bowl with an electric mixer until smooth and creamy.",
            )]);
            let _ = Recipe::create(&state.mm, users[0].id, &recipe, &settings).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    r##"Preheat the oven to 350 degrees F (175 degrees C). Beat <b>butter</b>, <b>white <b>sugar</b></b>, and <b>brown</b> <b>sugar</b> together in a large bowl with an electric mixer until smooth and creamy.</div>"##,
                ],
            );
            Ok(())
        }
    }

    mod tests_delete {

        use super::*;

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
            let user_id = User::all(&state.mm).await?[0].id;
            let settings = UserSettingDetails::get(&state.mm, user_id).await?;
            let (recipe, _) = a_complete_recipe_for_create();
            let _recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

            let res = server.delete(&base_uri(1)).await;

            res.assert_status(StatusCode::NO_CONTENT);
            res.assert_header(axum_htmx::HX_REDIRECT, "/");
            Ok(())
        }
    }

    fn assert_complete_recipe(res: &TestResponse, recipe: &RecipeForCreate, images: &RecipeImages) {
        assert_html(
            res,
            &[
                &format!(
                    "<title hx-swap-oob=\"true\">{} | Recipya</title>",
                    recipe.name
                ),
                r#"<button title="Toggle screen lock"#,
                r##"<button id="edit-recipe" class="ml-2 hidden sm:block" title="Edit recipe" hx-get="/recipes/1/edit" hx-push-url="true" hx-target="#content" hx-swap="innerHTML transition:true">"##,
                &format!(
                    "<span class=\"text-center pb-2 print:w-full\" itemprop=\"name\">{}</span>",
                    recipe.name
                ),
                r##"<li _="on click document.activeElement.blur()"><button hx-get="/recipes/1/rescrape" hx-target="#content" hx-swap="none" hx-indicator="#fullscreen-loader"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 21a9.004 9.004 0 0 0 8.716-6.747M12 21a9.004 9.004 0 0 1-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 0 1 7.843 4.582M12 3a8.997 8.997 0 0 0-7.843 4.582m15.686 0A11.953 11.953 0 0 1 12 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0 1 21 12c0 .778-.099 1.533-.284 2.253m0 0A17.919 17.919 0 0 1 12 16.5c-3.162 0-6.133-.815-8.716-2.247m0 0A9.015 9.015 0 0 1 3 12c0-1.605.42-3.113 1.157-4.418"></path></svg>Rescrape</button></li>"##,
                r##"<a title="Share recipe" hx-post="/recipes/1/share" hx-target="#share-dialog-result" hx-push-url="false""##,
                r#"<li title="Print recipe" _="on click print()">"#,
                r##"<a title="Delete recipe" hx-delete="/recipes/1" hx-swap="none" hx-confirm="Are you sure you wish to delete this recipe?" hx-indicator="#fullscreen-loader">"##,
                r#"<iframe src="https://example.com/embed/j43yfe3.mp4" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen="" style="height: 100%;width: 100%;"></iframe>"#,
                &format!(
                    r#"<div id="media-0" class="carousel-item relative w-full"><img style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src="/data/images/{}.webp">"#,
                    images.main,
                ),
                &format!(
                    r#"<img style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src="/data/images/{}.webp">"#,
                    images.main,
                ),
                r#"<div class="badge badge-primary badge-outline">dinner</div>"#,
                r#"<div class="badge badge-sm badge-neutral m-1 flex-auto">tofu</div><div class="badge badge-sm badge-neutral m-1 flex-auto">vegetarian</div>"#,
                r##"<fieldset class="fieldset"><legend>Servings</legend><input id="yield" type="number" min="1" name="yield" value="4" class="input max-w-18 md:max-w-24" hx-get="/recipes/1/scale" hx-trigger="input" hx-target="#ingredients-instructions-container"></fieldset>"##,
                r#"<a class="btn btn-sm btn-outline no-underline print:hidden" href="https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/" target="_blank"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 21a9.004 9.004 0 0 0 8.716-6.747M12 21a9.004 9.004 0 0 1-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 0 1 7.843 4.582M12 3a8.997 8.997 0 0 0-7.843 4.582m15.686 0A11.953 11.953 0 0 1 12 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0 1 21 12c0 .778-.099 1.533-.284 2.253m0 0A17.919 17.919 0 0 1 12 16.5c-3.162 0-6.133-.815-8.716-2.247m0 0A9.015 9.015 0 0 1 3 12c0-1.605.42-3.113 1.157-4.418"></path></svg>Source</a>"#,
                r#"<textarea readonly class="textarea textarea-ghost w-full h-full resize-none rounded-none focus:outline-none">This is the most delicious recipe!</textarea>"#,
                "<p class=\"text-xs\">Nutrition Facts\nPer 100g: calories 300 kcal; total carbohydrates 55g; sugar 43g; protein 7g; total fat 6g; saturated fat 1g; unsaturated fat 2g; trans fat 3g; cholesterol 5mg; sodium 12mg; fiber 10g</p>",
                r#"<div class="grid grid-flow-col border-y border-gray-700 col-span-6 py-1 md:border-b md:border-t-0 md:row-span-1 print:border-none print:hidden md:grid-cols-4 print:border-none md:grid-cols-4"><div class="contents grid grid-flow-col md:col-span-6"><div class="flex justify-self-center items-center gap-1 cursor-default" title="Prep time">"#,
                r#"<table class="table table-zebra table-xs print:hidden"><thead><tr><select class="select select-sm" onchange="filterNutritionRows(this, this.value)"><option value="per-100g">Nutrition (per 100g)</option><option value="per-serving">Nutrition (per serving)</option></select></tr></thead><tbody><tr data-nutrition-type="per-100g"><td>Calories:</td><td>300 kcal</td></tr><tr data-nutrition-type="per-100g"><td>Total carbs:</td><td>55g</td></tr><tr data-nutrition-type="per-100g"><td>Sugars:</td><td>43g</td></tr><tr data-nutrition-type="per-100g"><td>Protein:</td><td>7g</td></tr><tr data-nutrition-type="per-100g"><td>Total fat:</td><td>6g</td></tr><tr data-nutrition-type="per-100g"><td>Saturated fat:</td><td>1g</td></tr><tr data-nutrition-type="per-100g"><td>Unsaturated fat:</td><td>2g</td></tr><tr data-nutrition-type="per-100g"><td>Trans fat:</td><td>3g</td></tr><tr data-nutrition-type="per-100g"><td>Cholesterol:</td><td>5mg</td></tr><tr data-nutrition-type="per-100g"><td>Sodium:</td><td>12mg</td></tr><tr data-nutrition-type="per-100g"><td>Fiber:</td><td>10g</td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Serving size:</td><td></td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Calories:</td><td>240 kcal</td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Total carbs:</td><td>30g</td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Sugars:</td><td>24g</td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Protein:</td><td>4g</td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Total fat:</td><td>6g</td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Saturated fat:</td><td>2g</td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Unsaturated fat:</td><td>7g</td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Trans fat:</td><td>2g</td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Cholesterol:</td><td>12mg</td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Sodium:</td><td>100mg</td></tr><tr class="hidden" data-nutrition-type="per-serving"><td>Fiber:</td><td>18g</td></tr></tbody></table>"#,
                r#"<h1 class="text-sm print:mb-1"><b>Tools</b></h1><ol class="col-span-6 w-full mb-4" style="column-count: 1"><li class="text-sm"><label class="flex items-center w-full"><input type="checkbox"></label><span class="pl-2">1wok</span></li><li class="text-sm"><label class="flex items-center w-full"><input type="checkbox"></label><span class="pl-2">1frying pan</span></li></ol>"#,
                r#"<div class="col-span-6 px-8 py-2 border-gray-700 md:rounded-bl-none md:col-span-4 print:hidden"><h2 class="font-semibold text-center underline pb-1">Instructions</h2><div><h3 class="font-bold py-2">Sauce</h3><ol class="grid list-decimal"><li class="min-w-full py-2 select-none hover:bg-base-300" _="on mousedown toggle .line-through toggle .opacity-40 then if I match .line-through then add .invisible to .timer in me else remove .invisible from .timer in me"><div class="flex"><div class="whitespace-pre-line w-full transition-all">Mix all these ingredients</div></div></li></ol></div><div><h3 class="font-bold py-2">Chicken</h3><ol class="grid list-decimal"><li class="min-w-full py-2 select-none hover:bg-base-300" _="on mousedown toggle .line-through toggle .opacity-40 then if I match .line-through then add .invisible to .timer in me else remove .invisible from .timer in me"><div class="flex"><div class="whitespace-pre-line w-full transition-all">Turn the oven at 300 F</div></div></li><li class="min-w-full py-2 select-none hover:bg-base-300" _="on mousedown toggle .line-through toggle .opacity-40 then if I match .line-through then add .invisible to .timer in me else remove .invisible from .timer in me"><div class="flex"><div class="whitespace-pre-line w-full transition-all">Soak the chicken in the lemon juice</div></div></li><li class="min-w-full py-2 select-none hover:bg-base-300" _="on mousedown toggle .line-through toggle .opacity-40 then if I match .line-through then add .invisible to .timer in me else remove .invisible from .timer in me"><div class="flex"><div class="whitespace-pre-line w-full transition-all">Bake for 35 minutes</div><div id="timer-container-2" class="timer-container" _="on mousedown halt the event"><button class="timer btn btn-sm btn-wide btn-ghost" title="Start 35m timer" _="on click add .hidden to me"#,
                r#"<h1 class="text-sm print:mb-1"><b>Ingredients</b></h1><div><h3 class="font-bold py-2">Sauce</h3><ul class="col-span-6 w-full print:mb-2" style="column-count: 1"><li class="text-sm"><label><input type="checkbox"></label><span class="pl-2">1 cup blue spinach</span></li><li class="text-sm"><label><input type="checkbox"></label><span class="pl-2">1/2 tbsp cinnamon</span></li></ul><h3 class="font-bold py-2">Main</h3><ul class="col-span-6 w-full print:mb-2" style="column-count: 1"><li class="text-sm"><label><input type="checkbox"></label><span class="pl-2">4 pounds top quality chicken filet</span></li><li class="text-sm"><label><input type="checkbox"></label><span class="pl-2">1/8 cup lemon juice</span></li></ul></div></div><div class="hidden col-span-5 overflow-visible print:inline">"#,
            ],
        );
    }
}
