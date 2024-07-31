use axum::{
    http::{Method, Uri},
    Json,
    response::{IntoResponse, Response},
};
use serde_json::{json, to_value};
use uuid::Uuid;

use crate::{
    {ctx::Ctx, log::log_request},
    web::Error,
};
use crate::web::rpc::RpcInfo;

pub async fn map(ctx: Option<Ctx>, uri: Uri, req_method: Method, res: Response) -> Response {
    let uuid = Uuid::new_v4();

    let rpc_info = res.extensions().get::<RpcInfo>();

    let web_error = res.extensions().get::<Error>();
    let client_status_error = web_error.map(|se| se.client_status_and_error());

    let error_res = client_status_error
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
                        "detail": detail,
                    }
                }
            });

            (*status_code, Json(client_error_body)).into_response()
        });

    // TODO: Build and log the server log line
    let client_error = client_status_error.unzip().1;
    let _res = log_request(
        uuid,
        req_method,
        uri,
        rpc_info,
        ctx,
        web_error,
        client_error,
    )
    .await;

    error_res.unwrap_or(res)
}
