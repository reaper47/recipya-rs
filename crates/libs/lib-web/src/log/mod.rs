use std::time::{SystemTime, UNIX_EPOCH};

use axum::http::{Method, Uri};
use lib_core::ctx::Ctx;
use serde::Serialize;
use serde_json::Value;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::{
    error::{ClientError, Error, Result},
    handlers::handlers_rpc::RpcInfo,
};

pub async fn log_request(
    uuid: Uuid,
    req_method: Method,
    uri: Uri,
    rpc_info: Option<&RpcInfo>,
    ctx: Option<Ctx>,
    web_error: Option<&Error>,
    client_error: Option<ClientError>,
) -> Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let error_type = web_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(web_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    // TODO: Send to cloud-watch or log it somewhere.
    let _log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: timestamp.to_string(),

        http_path: uri.to_string(),
        http_method: req_method.to_string(),

        rpc_id: rpc_info.and_then(|rpc| rpc.id.as_ref().map(|id| id.to_string())),
        rpc_method: rpc_info.map(|rpc| rpc.method.to_string()),

        user_id: ctx.map(|c| c.user_id()),

        client_error_type: client_error.map(|e| e.as_ref().to_string()),

        error_type,
        error_data,
    };

    Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,      // uuid string formatted
    timestamp: String, // (should be iso8601)

    // User and context attributes.
    user_id: Option<i64>,

    // http request attributes.
    http_path: String,
    http_method: String,

    // rpc info.
    rpc_id: Option<String>,
    rpc_method: Option<String>,

    // Errors attributes.
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}
