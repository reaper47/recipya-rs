use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicI64, Ordering},
    },
    time::Duration,
};

use axum::{
    Form,
    extract::{State, ws::Message},
    response::IntoResponse,
};
use futures_util::lock::Mutex;
use rand::RngExt;
use reqwest::StatusCode;
use tokio::{sync::mpsc, time::Instant};
use tracing::error;
use url::Url;
use uuid::Uuid;

use app::state::AppState;
use models::{
    Recipe,
    reports::{
        report::{Items, ReportForCreate},
        report_log::ReportLogForCreate,
        report_types::ReportTypeFull,
    },
    settings::UserSettingDetails,
};

use crate::{
    Error,
    handlers::{
        message::{IMessage, MessageHtmx, MessageStatus, MessageType, broadcast_error},
        recipes::common::schema_to_recipe_for_create,
    },
    middleware::mw_auth::RequireAuth,
    recipes_router::params::RecipeScrapeForm,
};

#[derive(Clone)]
struct FetchWebsiteContext {
    count_success: Arc<AtomicI64>,
    count_warning: Arc<AtomicI64>,
    count_error: Arc<AtomicI64>,
    recipe_ids: Arc<Mutex<Vec<i64>>>,
    report_logs: Arc<Mutex<Vec<ReportLogForCreate>>>,
    total: i64,
}

impl FetchWebsiteContext {
    fn new(num_websites: usize, _user_id: Uuid) -> Self {
        Self {
            count_success: Arc::new(AtomicI64::default()),
            count_warning: Arc::new(AtomicI64::default()),
            count_error: Arc::new(AtomicI64::default()),
            recipe_ids: Arc::new(Mutex::new(Vec::with_capacity(num_websites))),
            report_logs: Arc::new(Mutex::new(Vec::new())),
            total: num_websites
                .try_into()
                .inspect_err(|err| error!("Failed to cast total '{num_websites}' to i64: {err}"))
                .unwrap_or(i64::MAX),
        }
    }

    async fn send_toast_after_processing(&self, state: &AppState, user_id: Uuid) {
        let count_success = self.count_success.load(Ordering::SeqCst);
        let count_error = self.count_error.load(Ordering::SeqCst);
        let count_warning = self.count_warning.load(Ordering::SeqCst);

        let toast = if self.total == 1 {
            if count_warning == 1 {
                let recipe_id = self.recipe_ids.lock().await.pop().unwrap();
                let view_recipe_link = format!("View /recipes/{recipe_id}");

                MessageHtmx::builder(MessageType::Toast, "Warning", "The recipe exists.")
                    .status(MessageStatus::Warning)
                    .action(Some(&view_recipe_link))
                    .build()
            } else if count_error == 1 {
                MessageHtmx::builder(MessageType::Toast, "Error", "Fetching the recipe failed.")
                    .status(MessageStatus::Error)
                    .action(Some("View /reports?view=latest"))
                    .build()
            } else if count_success == 1 {
                let recipe_id = self.recipe_ids.lock().await.pop().unwrap();
                let view_recipe_link = format!("View /recipes/{recipe_id}");

                MessageHtmx::builder(
                    MessageType::Toast,
                    "Success",
                    "Recipe has been added to your collection.",
                )
                .action(Some(&view_recipe_link))
                .build()
            } else {
                MessageHtmx::error("No recipe has been scraped.")
            }
        } else {
            let num_skipped = self.total - (count_success + count_warning);

            let message = format!("Fetched: {count_success}. Skipped: {num_skipped}");
            MessageHtmx::builder(MessageType::Toast, "Success", &message)
                .action(Some("View /reports?view=latest"))
                .build()
        };

        if let Ok(json) = serde_json::to_string(&toast) {
            state.broadcast(Message::Text(json.into()), user_id).await;
        }
    }
}

/// Handles scraping recipes from websites.
pub async fn add_website_post_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Form(form): Form<RecipeScrapeForm>,
) -> impl IntoResponse {
    let mut urls = form
        .urls
        .lines()
        .filter_map(|line| Url::parse(line.trim_end_matches('/')).ok())
        .collect::<Vec<_>>();

    if urls.is_empty() {
        broadcast_error(&state, user.id, "No valid URLs found.").await;
        return Error::InvalidPayload.into_response();
    }
    urls.sort();
    urls.dedup();

    scrape_recipes(state, urls, user.id);

    (StatusCode::ACCEPTED, "").into_response()
}

#[allow(clippy::too_many_lines)]
fn scrape_recipes(state: AppState, urls: Vec<Url>, user_id: Uuid) {
    tokio::spawn(async move {
        let num_urls = urls.len();
        let (tx, mut rx) = mpsc::channel::<()>(num_urls);
        let fetch_ctx = FetchWebsiteContext::new(num_urls, user_id);
        let total_exec_time = Instant::now();

        let mut by_host: HashMap<String, Vec<(usize, Url)>> = HashMap::new();
        for (idx, url) in urls.into_iter().enumerate() {
            let host = url.host_str().unwrap_or("unknown").to_string();
            by_host.entry(host).or_default().push((idx, url));
        }

        for (_host, host_urls) in by_host {
            let tx = tx.clone();
            let fetch_ctx = fetch_ctx.clone();
            let state = state.clone();

            tokio::spawn(async move {
                let mut host_urls = host_urls.into_iter().peekable();

                while let Some((idx, url)) = host_urls.next() {
                    process_recipe_url(&state, &fetch_ctx, idx, url, user_id).await;

                    let _ = tx.send(()).await;

                    if host_urls.peek().is_some() {
                        let jitter = rand::rng().random_range(0..500);
                        tokio::time::sleep(Duration::from_millis(jitter)).await;
                    }
                }
            });
        }
        drop(tx);

        let num_urls = num_urls
            .try_into()
            .inspect_err(|err| {
                error!("Failed to convert num_urls '{num_urls}' to i64: {err}");
            })
            .unwrap_or(i64::MAX);

        state
            .broadcast_progress("Fetching recipes", 0, num_urls, true, user_id)
            .await;

        let mut processed = 0;
        while rx.recv().await.is_some() {
            processed += 1;
            if fetch_ctx.total > 1 {
                state
                    .broadcast_progress(
                        "Fetching recipes",
                        processed,
                        fetch_ctx.total,
                        true,
                        user_id,
                    )
                    .await;
            }
        }

        state.hide_broadcast(user_id).await;

        let items = Items {
            total: i32::try_from(fetch_ctx.total).unwrap_or(0),
            success: i32::try_from(fetch_ctx.count_success.load(Ordering::SeqCst)).unwrap_or(0),
            skipped: i32::try_from(fetch_ctx.count_warning.load(Ordering::SeqCst)).unwrap_or(0),
            failed: i32::try_from(fetch_ctx.count_error.load(Ordering::SeqCst)).unwrap_or(0),
        };

        fetch_ctx.send_toast_after_processing(&state, user_id).await;

        let report = ReportForCreate::new(
            ReportTypeFull::website(),
            Arc::try_unwrap(fetch_ctx.report_logs)
                .expect("Report logs arc still has multiple owners")
                .into_inner(),
            items,
            i64::try_from(total_exec_time.elapsed().as_millis()).unwrap_or_default(),
            user_id,
        );

        match report.insert(&state.mm).await {
            Ok(()) => state.broadcast_trigger("refreshReports", user_id).await,
            Err(err) => {
                error!("Error inserting website report into the database: {err}");
            }
        }
    });
}

async fn process_recipe_url(
    state: &AppState,
    fetch_ctx: &FetchWebsiteContext,
    idx: usize,
    url: Url,
    user_id: Uuid,
) {
    let seq_num = i32::try_from(idx + 1).unwrap_or(1);
    let start_time = Instant::now();

    match state.scrape(url.clone()).await {
        Ok(schema) => {
            let user_settings = UserSettingDetails::get(&state.mm, user_id)
                .await
                .unwrap_or_default();
            let recipe_c = schema_to_recipe_for_create(state, schema).await;
            let created_result =
                Recipe::create(&state.mm, user_id, &recipe_c, &user_settings).await;
            let exec_time_ms = i64::try_from(start_time.elapsed().as_millis()).unwrap_or(0);

            match created_result {
                Ok(recipe_id) => {
                    fetch_ctx.count_success.fetch_add(1, Ordering::SeqCst);
                    fetch_ctx.recipe_ids.lock().await.push(recipe_id);
                    fetch_ctx
                        .report_logs
                        .lock()
                        .await
                        .push(ReportLogForCreate::success(
                            seq_num,
                            url.as_str(),
                            Some(recipe_id),
                            exec_time_ms,
                        ));
                }
                Err(err) => {
                    fetch_ctx.count_warning.fetch_add(1, Ordering::SeqCst);
                    error!("Error inserting recipe into database '{url}': {err}");
                    match err {
                        models::Error::DuplicateEntityWithID(id) => {
                            fetch_ctx.recipe_ids.lock().await.push(id);
                            fetch_ctx
                                .report_logs
                                .lock()
                                .await
                                .push(ReportLogForCreate::warning(
                                    seq_num,
                                    url.as_str(),
                                    Some(id),
                                    "Recipe exists",
                                    exec_time_ms,
                                ));
                        }
                        _ => {
                            fetch_ctx
                                .report_logs
                                .lock()
                                .await
                                .push(ReportLogForCreate::error(
                                    seq_num,
                                    url.as_str(),
                                    None,
                                    "DBInsertFail",
                                    &err.to_string(),
                                    exec_time_ms,
                                ));
                        }
                    }
                }
            }
        }
        Err(err) => {
            fetch_ctx.count_error.fetch_add(1, Ordering::SeqCst);
            error!("Error fetching recipe '{url}': {err}");
            fetch_ctx
                .report_logs
                .lock()
                .await
                .push(ReportLogForCreate::error(
                    seq_num,
                    url.as_str(),
                    None,
                    "WebsiteImportFail",
                    &err.to_string(),
                    i64::try_from(start_time.elapsed().as_millis()).unwrap_or(0),
                ));
        }
    }
}
