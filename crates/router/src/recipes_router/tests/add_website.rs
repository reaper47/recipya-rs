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
        build_server_ws, create_app_state,
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
                urls: "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/".into(),
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
                    title: "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies"
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
