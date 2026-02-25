#[cfg(test)]
mod tests {
    use std::time::Duration;

    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;
    use reqwest::{Method, StatusCode};

    use config::Config;
    use models::report::ReportLog;
    use recipya_scraper::tests::support::scraper::scrape_test_websites;
    use repository::schema;
    use testing::utils::{
        HIDDEN_WS_NOTIFICATION, TestDb, assert_must_be_logged_in, assert_ws_message,
        build_server_ws, collect_ws_messages, create_app_state,
    };

    use crate::recipes_router::params::RecipeScrapeForm;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

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
        assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetching recipes</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">0 of 1</span><span class="font-semibold">0.0%</span></div><div id="export-progress"><progress max="100" value="0.00"></progress></div></div></div>"#).await;
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
                urls: "https://zweigles.com/recipes/polish-kielbasa-sheet-pan-and-potatoes/".into(),
            })
            .await;

        res.assert_status(StatusCode::ACCEPTED);
        assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetching recipes</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">0 of 1</span><span class="font-semibold">0.0%</span></div><div id="export-progress"><progress max="100" value="0.00"></progress></div></div></div>"#).await;
        assert_ws_message(&mut ws_server, HIDDEN_WS_NOTIFICATION).await;
        assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","action":"View /recipes/1","message":"Recipe has been added to your collection.","status":"alert-info","title":"Operation Successful"}}"#).await;
        tokio::time::sleep(Duration::from_millis(50)).await;
        let got_logs = fetch_logs(config.clone()).await?;
        pretty_assertions::assert_eq!(got_logs, vec![zweigles_report_log(1)]);
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
                urls: "https://zweigles.com/recipes/polish-kielbasa-sheet-pan-and-potatoes/\nhttps://zweigles.com/recipes/polish-kielbasa-sheet-pan-and-potatoes/".into(),
            })
            .await;

        res.assert_status(StatusCode::ACCEPTED);
        assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetching recipes</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">0 of 1</span><span class="font-semibold">0.0%</span></div><div id="export-progress"><progress max="100" value="0.00"></progress></div></div></div>"#).await;
        assert_ws_message(&mut ws_server, HIDDEN_WS_NOTIFICATION).await;
        assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","action":"View /recipes/1","message":"Recipe has been added to your collection.","status":"alert-info","title":"Operation Successful"}}"#).await;
        tokio::time::sleep(Duration::from_millis(50)).await;
        let got_logs = fetch_logs(config.clone()).await?;
        pretty_assertions::assert_eq!(got_logs, vec![zweigles_report_log(1)]);
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_add_a_website_that_has_already_been_added_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let (server, mut ws_server) = build_server_ws(config.clone()).await?;
        let form = RecipeScrapeForm {
            urls: "https://zweigles.com/recipes/polish-kielbasa-sheet-pan-and-potatoes/\nhttps://zweigles.com/recipes/polish-kielbasa-sheet-pan-and-potatoes".into(),
        };
        scrape_test_websites(1).await?;
        let _ = server.post(BASE_URI).form(&form).await;

        let res = server.post(BASE_URI).form(&form).await;

        res.assert_status(StatusCode::ACCEPTED);
        let messages = collect_ws_messages(&mut ws_server, 5).await;
        assert_eq!(messages.len(), 5);
        pretty_assertions::assert_eq!(
            messages[0],
            r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetching recipes</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">0 of 1</span><span class="font-semibold">0.0%</span></div><div id="export-progress"><progress max="100" value="0.00"></progress></div></div></div>"#
        );
        pretty_assertions::assert_eq!(
            messages[1],
            r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetching recipes</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">0 of 1</span><span class="font-semibold">0.0%</span></div><div id="export-progress"><progress max="100" value="0.00"></progress></div></div></div>"#
        );
        pretty_assertions::assert_eq!(messages[2], HIDDEN_WS_NOTIFICATION);
        pretty_assertions::assert_eq!(messages[3], HIDDEN_WS_NOTIFICATION);
        pretty_assertions::assert_eq!(
            messages[4],
            r#"{"showMessageHtmx":{"type":"toast","action":"View /recipes/1","message":"Recipe has been added to your collection.","status":"alert-info","title":"Operation Successful"}}"#
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
        let got_logs = fetch_logs(config.clone()).await?;
        let want_logs = [
            zweigles_report_log(1),
            ReportLog {
                id: 2,
                report_id: 2,
                title: "https://zweigles.com/recipes/polish-kielbasa-sheet-pan-and-potatoes"
                    .to_string(),
                is_success: false,
                is_warning: true,
                is_error: false,
                error_reason: "Recipe exists".into(),
            },
        ];
        let normalize = |log: &ReportLog| {
            (
                log.title.clone(),
                log.is_success,
                log.is_warning,
                log.is_error,
                log.error_reason.clone(),
            )
        };
        let mut got_normalized: Vec<_> = got_logs.iter().map(normalize).collect();
        let mut want_normalized: Vec<_> = want_logs.iter().map(normalize).collect();
        got_normalized.sort();
        want_normalized.sort();
        pretty_assertions::assert_eq!(got_normalized, want_normalized);
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
            urls: "https://zweigles.com/recipes/polish-kielbasa-sheet-pan-and-potatoes/\nhttps://zumavalley.com/blogs/smoothies-bowls/coconut-mango-smoothie-bowl\nhttps://zsuzsaisinthekitchen.blogspot.com/2014/06/cherry-chutney.html".into(),
        }).await;

        res.assert_status(StatusCode::ACCEPTED);
        assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetching recipes</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">0 of 3</span><span class="font-semibold">0.0%</span></div><div id="export-progress"><progress max="100" value="0.00"></progress></div></div></div>"#).await;
        assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetched 1/3</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">1 of 3</span><span class="font-semibold">33.3%</span></div><div id="export-progress"><progress max="100" value="33.33"></progress></div></div></div>"#).await;
        assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetched 2/3</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">2 of 3</span><span class="font-semibold">66.7%</span></div><div id="export-progress"><progress max="100" value="66.67"></progress></div></div></div>"#).await;
        assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetched 3/3</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">3 of 3</span><span class="font-semibold">100.0%</span></div><div id="export-progress"><progress max="100" value="100.00"></progress></div></div></div>"#).await;
        assert_ws_message(&mut ws_server, HIDDEN_WS_NOTIFICATION).await;
        assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","action":"View /reports?view=latest","message":"Fetched: 3. Skipped: 0","status":"alert-info","title":"Operation Successful"}}"#).await;
        tokio::time::sleep(Duration::from_millis(50)).await;
        let got_logs = fetch_logs(config.clone()).await?;
        let want_logs = [
            zweigles_report_log(1),
            ReportLog {
                id: 2,
                report_id: 1,
                title: "https://zsuzsaisinthekitchen.blogspot.com/2014/06/cherry-chutney.html"
                    .to_owned(),
                is_success: true,
                is_warning: false,
                is_error: false,
                error_reason: String::new(),
            },
            ReportLog {
                id: 3,
                report_id: 1,
                title: "https://zumavalley.com/blogs/smoothies-bowls/coconut-mango-smoothie-bowl"
                    .to_owned(),
                is_success: true,
                is_warning: false,
                is_error: false,
                error_reason: String::new(),
            },
        ];
        let normalize = |log: &ReportLog| {
            (
                log.title.clone(),
                log.is_success,
                log.is_warning,
                log.is_error,
                log.error_reason.clone(),
            )
        };
        let mut got_normalized: Vec<_> = got_logs.iter().map(normalize).collect();
        let mut want_normalized: Vec<_> = want_logs.iter().map(normalize).collect();
        got_normalized.sort();
        want_normalized.sort();
        pretty_assertions::assert_eq!(got_normalized, want_normalized);
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

    fn zweigles_report_log(id: i64) -> ReportLog {
        ReportLog {
            id,
            report_id: id,
            title: "https://zweigles.com/recipes/polish-kielbasa-sheet-pan-and-potatoes"
                .to_string(),
            is_success: true,
            is_warning: false,
            is_error: false,
            error_reason: String::new(),
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
            error_reason: "Scraper(DomainNotImplemented)".to_string(),
        }
    }
}
