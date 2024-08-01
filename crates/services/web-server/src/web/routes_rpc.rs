use axum::{
    response::IntoResponse,
    routing::post,
    {extract::State, response::Response, Json, Router},
};
use serde_json::{json, Value};
use tracing::debug;

use lib_core::{ctx::Ctx, model::ModelManager};
use lib_rpc::{exec_rpc, RpcRequest};

use crate::web::{mw_auth::CtxW, Result};

/// RPC basic information containing the rpc request id and method for additional logging purposes.
#[derive(Clone, Debug)]
pub struct RpcInfo {
    pub id: Option<Value>,
    pub method: String,
}

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/rpc", post(rpc_handler))
        .with_state(mm)
}

async fn rpc_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Json(rpc_req): Json<RpcRequest>,
) -> Response {
    let ctx = ctx.0;
    let rpc_info = RpcInfo {
        id: rpc_req.id.clone(),
        method: rpc_req.method.clone(),
    };

    let mut res = _rpc_handler(ctx, mm, rpc_req).await.into_response();
    res.extensions_mut().insert(rpc_info);
    res
}

async fn _rpc_handler(ctx: Ctx, mm: ModelManager, rpc_req: RpcRequest) -> Result<Json<Value>> {
    let rpc_method = rpc_req.method.clone();
    let rpc_id = rpc_req.id.clone();

    debug!("{:<12} - _rpc_handler - method: {rpc_method}", "HANDLER");

    let result = exec_rpc(ctx, mm, rpc_req).await?;

    let body_response = json!({
        "id": rpc_id,
        "result": result
    });

    Ok(Json(body_response))
}
