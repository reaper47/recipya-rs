use axum::{handler::HandlerWithoutStateExt, http::StatusCode, routing::{any_service, MethodRouter}};
use maud::Markup;
use tower_http::services::ServeDir;

use crate::templates::general;

// Note: Here we can just return a MethodRouter rather than a full Router
//       since ServeDir is a service.
pub fn serve_dir(web_folder: &'static String) -> MethodRouter {
	async fn handle_404() -> (StatusCode, Markup) {
		(
			StatusCode::NOT_FOUND, 
			general::simple(
				"Page Not Found",
				"The page you requested to view is not found. Please go back to the main page.",
			)		
		)
	}

	any_service(
		ServeDir::new(web_folder)
			.not_found_service(handle_404.into_service()),
	)
}