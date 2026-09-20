use test_db::default_config;
use test_fixtures::insert_user;
use test_harness::create_app_state;

use models::reports::{
    ViewReport, ViewReportLog, ViewReportType,
    report::{Items, ReportForCreate},
    report_log::Level,
    report_log::ReportLogForCreate,
    report_types::{
        API, Import, PrimaryReportType, ReportTypeFull, SecondaryReportType, TertiaryReportType,
    },
    report_types::{ReportTypePrimary, ReportTypeSecondary, ReportTypeTertiary},
};

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

mod tests_view {
    use super::*;

    #[tokio::test]
    #[allow(clippy::too_many_lines)]
    async fn test_fetch_reports_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let reports = [
            ReportForCreate::new(
                ReportTypeFull {
                    primary: PrimaryReportType::<Import>::new(),
                    secondary: Some(SecondaryReportType::<Import, API>::new()),
                    tertiary: Some(TertiaryReportType::<Import, API>::mealie()),
                },
                vec![
                    ReportLogForCreate::success(1, "Raspberry Pi", None, 167),
                    ReportLogForCreate::success(2, "Orange Pi", None, 544),
                    ReportLogForCreate::success(3, "Pink Pi", None, 78),
                ],
                Items {
                    total: 3,
                    success: 3,
                    skipped: 0,
                    failed: 0,
                },
                789,
                user.id,
            ),
            ReportForCreate::new(
                ReportTypeFull::website(),
                vec![
                    ReportLogForCreate::success(
                        1,
                        "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/",
                        None,
                        243,
                    ),
                    ReportLogForCreate::warning(
                        2,
                        "https://www.allrecipes.com/southern-breakfast-potatoes-recipe-11907741",
                        None,
                        "The recipe exists in your collection.",
                        124,
                    ),
                    ReportLogForCreate::error(
                        3,
                        "https://www.allrecipes.com/southern-breakfast-potatoes-recipe-11907741",
                        None,
                        "NetworkFailure",
                        "Failed to connect to the website.",
                        124,
                    ),
                ],
                Items {
                    total: 3,
                    success: 1,
                    skipped: 1,
                    failed: 1,
                },
                491,
                user.id,
            ),
        ];
        reports[0].insert(&state.mm).await?;
        reports[1].insert(&state.mm).await?;

        let mut got = ViewReport::fetch_all(&state.mm, 1, user.id).await?;

        got.sort_by_key(|a| a.id);
        pretty_assertions::assert_eq!(
                got,
                vec![
                    ViewReport {
                        id: 1,
                        report_type: ViewReportType {
                            primary: ReportTypePrimary {
                                id: 1,
                                name: "import".into(),
                            },
                            secondary: Some(
                                ReportTypeSecondary {
                                    id: 1,
                                    name: "api".into(),
                                },
                            ),
                            tertiary: Some(
                                ReportTypeTertiary {
                                    id: 1,
                                    name: "mealie".into(),
                                },
                            ),
                        },
                        report_logs: vec![
                            ViewReportLog {
                                id: 1,
                                seq_num: 1,
                                entity_name: "Raspberry Pi".into(),
                                recipe_id: None,
                                level: Level {
                                    id: 2,
                                    name: "success".into(),
                                },
                                error_code: None,
                                error_reason: None,
                                exec_time_ms: 167,
                            },
                            ViewReportLog {
                                id: 2,
                                seq_num: 2,
                                entity_name: "Orange Pi".into(),
                                recipe_id: None,
                                level: Level {
                                    id: 2,
                                    name: "success".into(),
                                },
                                error_code: None,
                                error_reason: None,
                                exec_time_ms: 544,
                            },
                            ViewReportLog {
                                id: 3,
                                seq_num: 3,
                                entity_name: "Pink Pi".into(),
                                recipe_id: None,
                                level: Level {
                                    id: 2,
                                    name: "success".into(),
                                },
                                error_code: None,
                                error_reason: None,
                                exec_time_ms: 78,
                            },
                        ],
                        items: Items {
                            total: 3,
                            success: 3,
                            skipped: 0,
                            failed: 0,
                        },
                        user_id: user.id,
                        total_exec_time_ms: 789,
                        created_at: got[0].created_at,
                    },
                    ViewReport {
                        id: 2,
                        report_type: ViewReportType {
                            primary: ReportTypePrimary {
                                id: 2,
                                name: "website".into(),
                            },
                            secondary: None,
                            tertiary: None,
                        },
                        report_logs: vec![
                            ViewReportLog {
                                id: 4,
                                seq_num: 1,
                                entity_name: "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/".into(),
                                recipe_id: None,
                                level: Level {
                                    id: 2,
                                    name: "success".into(),
                                },
                                error_code: None,
                                error_reason: None,
                                exec_time_ms: 243,
                            },
                            ViewReportLog {
                                id: 5,
                                seq_num: 2,
                                entity_name: "https://www.allrecipes.com/southern-breakfast-potatoes-recipe-11907741".into(),
                                recipe_id: None,
                                level: Level {
                                    id: 3,
                                    name: "warning".into(),
                                },
                                error_code: None,
                                error_reason: Some(
                                    "The recipe exists in your collection.".into(),
                                ),
                                exec_time_ms: 124,
                            },
                            ViewReportLog {
                                id: 6,
                                seq_num: 3,
                                entity_name: "https://www.allrecipes.com/southern-breakfast-potatoes-recipe-11907741".into(),
                                recipe_id: None,
                                level: Level {
                                    id: 4,
                                    name: "error".into(),
                                },
                                error_code: Some(
                                    "NetworkFailure".into(),
                                ),
                                error_reason: Some(
                                    "Failed to connect to the website.".into(),
                                ),
                                exec_time_ms: 124,
                            },
                        ],
                        items: Items {
                            total: 3,
                            success: 1,
                            skipped: 1,
                            failed: 1,
                        },
                        user_id: user.id,
                        total_exec_time_ms: 491,
                        created_at: got[1].created_at,
                    },
                ]
            );
        Ok(())
    }
}
