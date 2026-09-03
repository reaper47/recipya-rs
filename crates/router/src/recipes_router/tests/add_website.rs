#[cfg(test)]
mod tests {
    use std::{iter::once, time::Duration};

    use app::state::AppState;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;
    use models::reports::report_log::ReportLog;
    use reqwest::{Method, StatusCode};

    use recipya_scraper::tests::support::scraper::scrape_test_websites;
    use repository::schema;
    use test_db::default_config;
    use test_fixtures::{HIDDEN_WS_NOTIFICATION, assert_ws_message, collect_ws_messages};
    use test_utils::{assert_must_be_logged_in, build_server_ws};

    use crate::recipes_router::params::RecipeScrapeForm;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    const BASE_URI: &str = "/recipes/add/website";

    fn normalize_log(log: &ReportLog) -> (i64, String, Option<String>, i16, Option<String>) {
        (
            log.report_id,
            log.entity_name.clone(),
            log.error_reason.clone(),
            log.level_id,
            log.error_code.clone(),
        )
    }

    #[tokio::test]
    async fn test_must_be_logged_in_ok() -> Result<()> {
        assert_must_be_logged_in(Method::POST, BASE_URI).await
    }

    #[tokio::test]
    async fn test_no_input_ok() -> Result<()> {
        let (server, mut ws_server, _) = build_server_ws(default_config()).await?;

        let res = server
            .post(BASE_URI)
            .form(&RecipeScrapeForm {
                urls: String::new(),
            })
            .await;

        res.assert_status_bad_request();
        assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"No valid URLs found.","status":"alert-error","title":"Operation Failed"}}"#).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_no_valid_urls_ok() -> Result<()> {
        let (server, mut ws_server, _) = build_server_ws(default_config()).await?;

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
    async fn test_add_one_valid_url_from_unsupported_websites_ok() -> Result<()> {
        let (server, mut ws_server, state) = build_server_ws(default_config()).await?;

        let res = server
            .post(BASE_URI)
            .form(&RecipeScrapeForm {
                urls: "https://www.example.com".into(),
            })
            .await;

        res.assert_status(StatusCode::ACCEPTED);
        assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetching recipes</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">0 of 1</span><span class="font-semibold">0.0%</span></div><div id="export-progress"><progress max="100" value="0.00"></progress></div></div></div>"#).await;
        assert_ws_message(&mut ws_server, HIDDEN_WS_NOTIFICATION).await;
        assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","action":"View /reports?view=latest","message":"Fetching the recipe failed.","status":"alert-error","title":"Error"}}"#).await;
        tokio::time::sleep(Duration::from_millis(500)).await;
        let got_logs = fetch_logs(&state).await?;
        let got_normalized = got_logs.iter().map(normalize_log).collect::<Vec<_>>();
        let mut want_normalized = once(&example_report_log())
            .map(normalize_log)
            .collect::<Vec<_>>();
        want_normalized
            .iter_mut()
            .zip(got_normalized.clone())
            .for_each(|(want, got)| {
                want.0 = got.0;
            });
        pretty_assertions::assert_eq!(got_normalized, want_normalized);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "flaky in CI, needs manual testing"]
    async fn test_add_one_valid_url_from_supported_websites_ok() -> Result<()> {
        let (server, mut ws_server, state) = build_server_ws(default_config()).await?;
        scrape_test_websites(1).await?;

        let res = server
            .post(BASE_URI)
            .form(&RecipeScrapeForm {
                urls: "https://zweigles.com/recipes/polish-kielbasa-sheet-pan-and-potatoes/".into(),
            })
            .await;

        res.assert_status(StatusCode::ACCEPTED);
        let messages = collect_ws_messages(&mut ws_server, 3).await;
        pretty_assertions::assert_eq!(
            messages[0],
            r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetching recipes</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">0 of 1</span><span class="font-semibold">0.0%</span></div><div id="export-progress"><progress max="100" value="0.00"></progress></div></div></div>"#
        );
        pretty_assertions::assert_eq!(messages[1], HIDDEN_WS_NOTIFICATION);
        let success_msg = r#"{"showMessageHtmx":{"type":"toast","action":"View /recipes/1","message":"Recipe has been added to your collection.","status":"alert-info","title":"Success"}}"#;
        let failure_msg = r#"{"showMessageHtmx":{"type":"toast","action":"View /reports?view=latest","message":"Fetching the recipe failed.","status":"alert-info","title":"Operation Failed"}}"#;
        assert!(
            messages[2] == success_msg || messages[2] == failure_msg,
            "Unexpected message: {}",
            messages[2]
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
        let got_logs = fetch_logs(&state).await?;
        let got_normalized = got_logs.iter().map(normalize_log).collect::<Vec<_>>();
        let want_normalized = once(&zweigles_report_log(1))
            .map(normalize_log)
            .collect::<Vec<_>>();
        pretty_assertions::assert_eq!(got_normalized, want_normalized);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "flaky in CI, needs manual testing"]
    async fn test_add_duplicates_ok() -> Result<()> {
        let (server, mut ws_server, state) = build_server_ws(default_config()).await?;
        scrape_test_websites(1).await?;

        let res = server
            .post(BASE_URI)
            .form(&RecipeScrapeForm {
                urls: "https://zweigles.com/recipes/polish-kielbasa-sheet-pan-and-potatoes/\nhttps://zweigles.com/recipes/polish-kielbasa-sheet-pan-and-potatoes/".into(),
            })
            .await;

        res.assert_status(StatusCode::ACCEPTED);
        let messages = collect_ws_messages(&mut ws_server, 3).await;
        assert!(
            messages.iter().any(|m| m.contains("Fetching recipes")),
            "Missing progress notification"
        );
        assert!(
            messages.iter().any(|m| m == HIDDEN_WS_NOTIFICATION),
            "Missing hidden notification"
        );
        assert!(
            messages.iter().any(|m| m.contains("Recipe has been added")),
            "Missing success toast"
        );
        let got_logs = tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                let logs = fetch_logs(&state).await?;
                if !logs.is_empty() {
                    return Ok::<_, Box<dyn std::error::Error>>(logs);
                }
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
        })
        .await
        .map_err(|_| "Timed out waiting for logs to appear in DB")??;
        let got_normalized = got_logs.iter().map(normalize_log).collect::<Vec<_>>();
        let want_normalized = once(&zweigles_report_log(1))
            .map(normalize_log)
            .collect::<Vec<_>>();
        pretty_assertions::assert_eq!(got_normalized, want_normalized);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "flaky in CI, needs manual testing"]
    async fn test_add_a_website_that_has_already_been_added_ok() -> Result<()> {
        let (server, mut ws_server, state) = build_server_ws(default_config()).await?;
        let form = RecipeScrapeForm {
            urls: "https://zweigles.com/recipes/polish-kielbasa-sheet-pan-and-potatoes/\nhttps://zweigles.com/recipes/polish-kielbasa-sheet-pan-and-potatoes".into(),
        };
        scrape_test_websites(1).await?;
        let _ = server.post(BASE_URI).form(&form).await;
        let _ = collect_ws_messages(&mut ws_server, 4).await;

        let res = server.post(BASE_URI).form(&form).await;

        res.assert_status(StatusCode::ACCEPTED);
        let messages = collect_ws_messages(&mut ws_server, 4).await;
        assert_eq!(messages.len(), 4);
        pretty_assertions::assert_eq!(
            messages[0],
            r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetching recipes</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">0 of 1</span><span class="font-semibold">0.0%</span></div><div id="export-progress"><progress max="100" value="0.00"></progress></div></div></div>"#
        );
        pretty_assertions::assert_eq!(
            messages[1],
            r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default hidden"><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1"></p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">-1 of -1</span><span class="font-semibold">0.0%</span></div><div id="export-progress"><progress max="100" value="0.00"></progress></div></div></div>"#
        );
        pretty_assertions::assert_eq!(
            messages[2],
            r#"{"showMessageHtmx":{"type":"toast","action":"View /recipes/1","message":"The recipe exists.","status":"alert-warning","title":"Warning"}}"#
        );
        pretty_assertions::assert_eq!(
            messages[3],
            r#"{"headers": {"HX-Trigger": "refreshReports"}}"#
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
        let got_logs = fetch_logs(&state).await?;
        let want_logs = [
            zweigles_report_log(1),
            ReportLog {
                id: 2,
                report_id: 2,
                entity_name: "https://zweigles.com/recipes/polish-kielbasa-sheet-pan-and-potatoes"
                    .to_string(),
                error_reason: Some("Recipe exists".into()),
                seq_num: 1,
                recipe_id: Some(1),
                level_id: 3,
                error_code: None,
                exec_time_ms: 100,
            },
        ];
        let mut got_normalized = got_logs.iter().map(normalize_log).collect::<Vec<_>>();
        let mut want_normalized = want_logs.iter().map(normalize_log).collect::<Vec<_>>();
        got_normalized.sort();
        want_normalized.sort();
        pretty_assertions::assert_eq!(got_normalized, want_normalized);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "flaky in CI, needs manual testing"]
    async fn test_add_many_valid_urls_from_supported_websites_ok() -> Result<()> {
        let (server, mut ws_server, state) = build_server_ws(default_config()).await?;
        scrape_test_websites(1).await?;
        scrape_test_websites(2).await?;
        scrape_test_websites(3).await?;

        let res = server.post(BASE_URI).form(&RecipeScrapeForm {
            urls: "https://zweigles.com/recipes/polish-kielbasa-sheet-pan-and-potatoes/\nhttps://zumavalley.com/blogs/smoothies-bowls/coconut-mango-smoothie-bowl\nhttps://zsuzsaisinthekitchen.blogspot.com/2014/06/cherry-chutney.html".into(),
        }).await;

        res.assert_status(StatusCode::ACCEPTED);
        assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetching recipes</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">0 of 3</span><span class="font-semibold">0.0%</span></div><div id="export-progress"><progress max="100" value="0.00"></progress></div></div></div>"#).await;
        assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetching recipes</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">1 of 3</span><span class="font-semibold">33.3%</span></div><div id="export-progress"><progress max="100" value="33.33"></progress></div></div></div>"#).await;
        assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetching recipes</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">2 of 3</span><span class="font-semibold">66.7%</span></div><div id="export-progress"><progress max="100" value="66.67"></progress></div></div></div>"#).await;
        assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetching recipes</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">3 of 3</span><span class="font-semibold">100.0%</span></div><div id="export-progress"><progress max="100" value="100.00"></progress></div></div></div>"#).await;
        assert_ws_message(&mut ws_server, HIDDEN_WS_NOTIFICATION).await;
        assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","action":"View /reports?view=latest","message":"Fetched: 3. Skipped: 0","status":"alert-info","title":"Success"}}"#).await;
        tokio::time::sleep(Duration::from_millis(50)).await;
        let got_logs = fetch_logs(&state).await?;
        let want_logs = [
            zweigles_report_log(1),
            ReportLog {
                id: 2,
                report_id: 1,
                entity_name:
                    "https://zsuzsaisinthekitchen.blogspot.com/2014/06/cherry-chutney.html".into(),
                error_reason: None,
                seq_num: 1,
                recipe_id: Some(2),
                level_id: 2,
                error_code: None,
                exec_time_ms: 100,
            },
            ReportLog {
                id: 3,
                report_id: 1,
                entity_name:
                    "https://zumavalley.com/blogs/smoothies-bowls/coconut-mango-smoothie-bowl"
                        .into(),
                error_reason: None,
                seq_num: 2,
                recipe_id: Some(3),
                level_id: 2,
                error_code: None,
                exec_time_ms: 200,
            },
        ];
        let mut got_normalized = got_logs.iter().map(normalize_log).collect::<Vec<_>>();
        let mut want_normalized = want_logs.iter().map(normalize_log).collect::<Vec<_>>();
        got_normalized.sort();
        want_normalized.sort();
        pretty_assertions::assert_eq!(got_normalized, want_normalized);
        Ok(())
    }

    async fn fetch_logs(state: &AppState) -> Result<Vec<ReportLog>> {
        let mut conn = state.mm.pool.get().await?;
        let logs = schema::reports_logs::table
            .select(ReportLog::as_select())
            .load(&mut conn)
            .await?;
        Ok(logs)
    }

    fn zweigles_report_log(id: i64) -> ReportLog {
        ReportLog {
            id,
            report_id: id,
            entity_name: "https://zweigles.com/recipes/polish-kielbasa-sheet-pan-and-potatoes"
                .into(),
            error_reason: None,
            seq_num: i32::try_from(id).unwrap(),
            recipe_id: Some(id),
            level_id: 2,
            error_code: None,
            exec_time_ms: 100,
        }
    }

    fn example_report_log() -> ReportLog {
        ReportLog {
            id: 1,
            report_id: 1,
            entity_name: "https://www.example.com/".to_string(),
            error_reason: Some("Scraper(DomainNotImplemented)".to_string()),
            seq_num: 2,
            recipe_id: None,
            level_id: 4,
            error_code: Some("WebsiteImportFail".into()),
            exec_time_ms: 250,
        }
    }
}
