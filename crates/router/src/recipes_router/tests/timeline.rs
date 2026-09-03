#[cfg(test)]
mod tests {
    use axum_test::http::StatusCode;
    use axum_test::multipart::MultipartForm;
    use reqwest::Method;
    use time::macros::format_description;
    use time::{OffsetDateTime, PrimitiveDateTime};
    use uuid::Uuid;

    use models::Recipe;
    use models::recipe::timeline::{RecipeTimeline, RecipeTimelineForCreate};
    use models::settings::UserSettingDetails;
    use models::user::User;
    use test_db::default_config;
    use test_fixtures::{assert_html, assert_ws_message};
    use test_models::a_complete_recipe_for_create;
    use test_utils::{assert_must_be_logged_in, build_server_logged_in, build_server_ws};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    fn base_uri(recipe_id: i64) -> String {
        format!("/recipes/{recipe_id}/timeline")
    }

    fn base_uri_timeline(recipe_id: i64, timeline_id: i64, index: usize, max_index: i64) -> String {
        format!("/recipes/{recipe_id}/timelines/{timeline_id}?index={index}&max-index={max_index}")
    }

    fn create_timeline_event_form() -> MultipartForm {
        let mut form = MultipartForm::new();

        form = form.add_text("title", "Recipe made");
        form = form.add_text("comment", "comment 1");
        form = form.add_text("rating", "1");

        form
    }

    #[tokio::test]
    async fn test_must_be_logged_in_ok() -> Result<()> {
        assert_must_be_logged_in(Method::GET, &base_uri(1)).await?;
        assert_must_be_logged_in(Method::POST, &base_uri(1)).await
    }

    #[tokio::test]
    async fn test_get_recipe_not_exist_ok() -> Result<()> {
        let (server, mut ws_server, _) = build_server_ws(default_config()).await?;

        let res = server.get(&base_uri(5)).await;

        res.assert_status_not_found();
        assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Recipe not found.","status":"alert-error","title":"Operation Failed"}}"# ).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_recipe_exists_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (recipe, _) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;
        let an_image = Uuid::new_v4();
        let now = {
            let dt = OffsetDateTime::from_unix_timestamp(1_643_609_600).expect("Invalid timestamp");
            PrimitiveDateTime::new(dt.date(), dt.time())
        };
        let today = OffsetDateTime::now_utc();
        let timeline_id1 = RecipeTimeline::create(
            &state.mm,
            recipe_id,
            user_id,
            &RecipeTimelineForCreate {
                title: "Recipe made".into(),
                comment: Some("comment 1".into()),
                rating: Some(1),
                image: Some(an_image),
                created_at: Some(now),
            },
        )
        .await?;
        let timeline_id2 = RecipeTimeline::create(
            &state.mm,
            recipe_id,
            user_id,
            &RecipeTimelineForCreate::default(),
        )
        .await?;

        let res = server.get(&base_uri(recipe_id)).await;

        res.assert_status_ok();
        assert_html(
            &res,
            &[
                &format!(
                    r#"<li id="timeline-event-0"><div class="timeline-start timeline-box p-0"><div class="card lg:card-side card-sm bg-base-100 shadow-sm max-w-md"><div class="card-body max-w-60"><h2 class="card-title">Recipe created</h2></div></div></div><div class="timeline-middle"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="h-5 w-5"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd"></svg></div><div class="timeline-end"><p class="text-center pb-2">{}</p></div><hr></li>"#,
                    today
                        .format(format_description!("[year]-[month]-[day]"))
                        .unwrap()
                ),
                &format!(
                    r##"<li id="timeline-event-{timeline_id1}"><hr><div class="timeline-start timeline-box p-0"><div class="card lg:card-side card-sm bg-base-100 shadow-sm max-w-md"><div class="card-body max-w-60"><h2 class="card-title">Recipe made</h2><p>comment 1</p><div class="flex justify-between items-baseline"><div class="rating rating-sm"><div class="mask mask-star-2" aria-label="1 star" aria-current="true"></div><div class="mask mask-star-2" aria-label="2 star"></div><div class="mask mask-star-2" aria-label="3 star"></div><div class="mask mask-star-2" aria-label="4 star"></div><div class="mask mask-star-2" aria-label="5 star"></div></div><button class="btn btn-ghost btn-square btn-sm" hx-get="/recipes/{recipe_id}/timelines/{timeline_id1}/edit?index=1&amp;max-index=3" hx-target="#timeline-event-{timeline_id1}" hx-swap="outerHTML"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button></div></div></div></div><div class="timeline-middle"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="h-5 w-5"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd"></svg></div><div class="timeline-end"><p class="text-center pb-2">{}</p><figure><img src="/data/images/Timelines/{an_image}.webp" alt="Timeline event" class="w-full lg:w-60 h-60 object-cover rounded-lg"></figure></div><hr></li>"##,
                    now.format(format_description!("[year]-[month]-[day]"))
                        .unwrap()
                ),
                &format!(
                    r##"<li id="timeline-event-{timeline_id2}"><hr><div class="timeline-start timeline-box p-0"><div class="card lg:card-side card-sm bg-base-100 shadow-sm max-w-md"><div class="card-body max-w-60"><h2 class="card-title"></h2><div class="card-actions justify-end"><button class="btn btn-ghost btn-square btn-sm" hx-get="/recipes/{recipe_id}/timelines/{timeline_id2}/edit?index=2&amp;max-index=3" hx-target="#timeline-event-{timeline_id2}" hx-swap="outerHTML"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button></div></div></div></div><div class="timeline-middle"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="h-5 w-5"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd"></svg></div><div class="timeline-end"><p class="text-center pb-2">{}</p></div></li>"##,
                    today
                        .format(format_description!("[year]-[month]-[day]"))
                        .unwrap()
                ),
            ],
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_post_add_new_timeline_recipe_not_exist_ok() -> Result<()> {
        let (server, mut ws_server, _) = build_server_ws(default_config()).await?;

        let res = server
            .post(&base_uri(9999))
            .multipart(create_timeline_event_form())
            .await;

        res.assert_status_internal_server_error();
        assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Could not create timeline event.","status":"alert-error","title":"Operation Failed"}}"# ).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_post_add_new_timeline_ok() -> Result<()> {
        let (server, mut ws_server, state) = build_server_ws(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (recipe, _) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

        let res = server
            .post(&base_uri(recipe_id))
            .multipart(create_timeline_event_form())
            .await;

        res.assert_status(StatusCode::CREATED);
        assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Timeline event created.","status":"alert-info","title":"Success"}}"# ).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_timeline_event_not_exist_ok() -> Result<()> {
        let (server, mut ws_server, _) = build_server_ws(default_config()).await?;

        let res = server.get(&base_uri_timeline(1, 5, 1, 5)).await;

        res.assert_status_not_found();
        assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Timeline event does not exist.","status":"alert-warning","title":"Attention"}}"# ).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_timeline_event_exists_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let now = {
            let dt = OffsetDateTime::from_unix_timestamp(1_543_609_500).expect("Invalid timestamp");
            PrimitiveDateTime::new(dt.date(), dt.time())
        };
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (recipe, _) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;
        let event_id = RecipeTimeline::create(
            &state.mm,
            recipe_id,
            user_id,
            &RecipeTimelineForCreate {
                title: "a title".to_string(),
                comment: Some("a comment".to_string()),
                rating: Some(2),
                image: None,
                created_at: Some(now),
            },
        )
        .await?;

        let res = server
            .get(&base_uri_timeline(recipe_id, event_id, 1, 5))
            .await;

        res.assert_status_ok();
        assert_html(
            &res,
            &[
                &format!(r#"<li id="timeline-event-{event_id}"><hr>"#),
                &format!(
                    r##"<div class="timeline-start timeline-box p-0"><div class="card lg:card-side card-sm bg-base-100 shadow-sm max-w-md"><div class="card-body max-w-60"><h2 class="card-title">a title</h2><p>a comment</p><div class="flex justify-between items-baseline"><div class="rating rating-sm"><div class="mask mask-star-2" aria-label="1 star"></div><div class="mask mask-star-2" aria-label="2 star" aria-current="true"></div><div class="mask mask-star-2" aria-label="3 star"></div><div class="mask mask-star-2" aria-label="4 star"></div><div class="mask mask-star-2" aria-label="5 star"></div></div><button class="btn btn-ghost btn-square btn-sm" hx-get="/recipes/{recipe_id}/timelines/{event_id}/edit?index=1&amp;max-index=5" hx-target="#timeline-event-{event_id}" hx-swap="outerHTML"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button></div></div></div></div>"##
                ),
                r#"<div class="timeline-middle"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="h-5 w-5"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd"></svg></div>"#,
                &format!(
                    r#"<div class="timeline-end"><p class="text-center pb-2">{}</p></div><hr></li>"#,
                    now.format(format_description!("[year]-[month]-[day]"))
                        .unwrap()
                ),
            ],
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_get_edit_timeline_event_not_exist_ok() -> Result<()> {
        let (server, mut ws_server, _) = build_server_ws(default_config()).await?;

        let res = server.get(&base_uri_timeline(1, 5, 1, 5)).await;

        res.assert_status_not_found();
        assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Timeline event does not exist.","status":"alert-warning","title":"Attention"}}"# ).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_edit_timeline_event_recipe_exists_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (recipe, _) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;
        let _ = server
            .post(&base_uri(recipe_id))
            .multipart(create_timeline_event_form())
            .await;
        let now = OffsetDateTime::now_utc();
        let timeline_id = RecipeTimeline::all(&state.mm, recipe_id, user_id)
            .await?
            .last()
            .unwrap()
            .id;

        let res = server
            .get(&base_uri_timeline(recipe_id, timeline_id, 1, 5))
            .await;

        res.assert_status_ok();
        assert_html(
            &res,
            &[
                &format!(r#"<li id="timeline-event-{timeline_id}"><hr>"#),
                &format!(
                    r##"<div class="timeline-start timeline-box p-0"><div class="card lg:card-side card-sm bg-base-100 shadow-sm max-w-md"><div class="card-body max-w-60"><h2 class="card-title">Recipe made</h2><p>comment 1</p><div class="card-actions justify-end"><button class="btn btn-ghost btn-square btn-sm" hx-get="/recipes/{recipe_id}/timelines/{timeline_id}/edit?index=1&amp;max-index=5" hx-target="#timeline-event-{timeline_id}" hx-swap="outerHTML"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button></div></div></div></div>"##
                ),
                r#"<div class="timeline-middle"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="h-5 w-5"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd"></svg></div>"#,
                &format!(
                    r#"<div class="timeline-end"><p class="text-center pb-2">{}</p></div><hr></li>"#,
                    now.format(format_description!("[year]-[month]-[day]"))
                        .unwrap()
                ),
            ],
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_put_edit_timeline_event_not_exist_ok() -> Result<()> {
        let (server, mut ws_server, _) = build_server_ws(default_config()).await?;

        let res = server
            .put(&base_uri_timeline(1, 5, 1, 5))
            .multipart(create_timeline_event_form())
            .await;

        res.assert_status_not_found();
        assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Timeline event does not exist.","status":"alert-warning","title":"Attention"}}"# ).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_put_edit_timeline_event_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (recipe, _) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;
        let _ = server
            .post(&base_uri(recipe_id))
            .multipart(create_timeline_event_form())
            .await;
        let mut form = MultipartForm::new();
        form = form.add_text("title", "Hey");
        form = form.add_text("comment", "Hello");
        form = form.add_text("rating-event", "5");
        let _ = server.post(&base_uri(recipe_id)).multipart(form).await;
        let now = OffsetDateTime::now_utc();
        let mut form = MultipartForm::new();
        form = form.add_text("title", "Edited");
        form = form.add_text("index", "2");
        form = form.add_text("max-index", "2");
        let timeline_id = RecipeTimeline::all(&state.mm, recipe_id, user_id)
            .await?
            .last()
            .unwrap()
            .id;

        let res = server
            .put(&base_uri_timeline(recipe_id, timeline_id, 2, 2))
            .multipart(form)
            .await;

        res.assert_status_ok();
        assert_html(
            &res,
            &[
                &format!(r#"<li id="timeline-event-{timeline_id}"><hr>"#),
                &format!(
                    r##"<div class="timeline-start timeline-box p-0"><div class="card lg:card-side card-sm bg-base-100 shadow-sm max-w-md"><div class="card-body max-w-60"><h2 class="card-title">Edited</h2><p>Hello</p><div class="flex justify-between items-baseline"><div class="rating rating-sm"><div class="mask mask-star-2" aria-label="1 star"></div><div class="mask mask-star-2" aria-label="2 star"></div><div class="mask mask-star-2" aria-label="3 star"></div><div class="mask mask-star-2" aria-label="4 star"></div><div class="mask mask-star-2" aria-label="5 star" aria-current="true"></div></div><button class="btn btn-ghost btn-square btn-sm" hx-get="/recipes/{recipe_id}/timelines/{timeline_id}/edit?index=2&amp;max-index=2" hx-target="#timeline-event-{timeline_id}" hx-swap="outerHTML"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button></div></div></div></div>"##
                ),
                r#"<div class="timeline-middle"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="h-5 w-5"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd"></svg></div>"#,
                &format!(
                    r#"<div class="timeline-end"><p class="text-center pb-2">{}</p></div><hr></li>"#,
                    now.format(format_description!("[year]-[month]-[day]"))
                        .unwrap()
                ),
            ],
        );
        Ok(())
    }
}
