use axum::{
    http::{Method, Uri},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, to_value};
use tracing::debug;
use uuid::Uuid;

use crate::{error::Error, handlers::handlers_rpc::RpcInfo, log::log_request, middleware::mw_auth::CtxW};


pub async fn mw_reponse_map(
    ctx: Option<CtxW>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    let ctx = ctx.map(|ctx| ctx.0);
    let uuid = Uuid::new_v4();
    let rpc_info = res.extensions().get::<RpcInfo>();

    // Get the eventual response error.
    let web_error = res.extensions().get::<Error>();
    let client_status_error = web_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error = to_value(client_error).ok();
            let message = client_error.as_ref().and_then(|v| v.get("message"));
            let detail = client_error.as_ref().and_then(|v| v.get("detail"));

            let client_error_body = json!({
                "id": rpc_info.as_ref().map(|rpc| rpc.id.clone()),
                "error": {
                    "message": message, // Variant name
                    "data": {
                        "req_uuid": uuid.to_string(),
                        "detail": detail
                    },
                }
            });

            debug!("CLIENT ERROR BODY:\n{client_error_body}");

            (*status_code, Json(client_error_body)).into_response()
        });

    let client_error = client_status_error.unzip().1;
    // TODO: Need to handle if log_request fail (but should not fail request)
    let _ = log_request(
        uuid,
        req_method,
        uri,
        rpc_info,
        ctx,
        web_error,
        client_error,
    )
    .await;

    error_response.unwrap_or(res)
}
