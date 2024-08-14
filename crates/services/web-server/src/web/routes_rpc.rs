use std::sync::Arc;

use axum::{
    response::IntoResponse,
    routing::post,
    {extract::State, response::Response, Json, Router},
};
use serde_json::{json, Value};

use lib_core::model::ModelManager;
use lib_rpc::{all_rpc_router, router::RpcRouter, RpcRequest, RpcResources};

use crate::web::mw_auth::CtxW;

#[derive(Clone)]
pub struct RpcAxumHandlerState {
    rpc_router: Arc<RpcRouter>,
    mm: ModelManager,
}

/// RPC basic information containing the rpc request id and method for additional logging purposes.
#[derive(Clone, Debug)]
pub struct RpcInfo {
    pub id: Option<Value>,
    pub method: String,
}

/// Axum router for '/api/rpc'
pub fn routes(mm: ModelManager) -> Router {
    let rpc_router = Arc::new(all_rpc_router());
    let axum_state = RpcAxumHandlerState { rpc_router, mm };

    Router::new()
        .route("/rpc", post(rpc_axum_handler))
        .with_state(axum_state)
}

async fn rpc_axum_handler(
    State(axum_state): State<RpcAxumHandlerState>,
    ctx: CtxW,
    Json(rpc_req): Json<RpcRequest>,
) -> Response {
    let RpcAxumHandlerState { rpc_router, mm } = axum_state;
    let ctx = ctx.0;

    let rpc_info = RpcInfo {
        id: rpc_req.id.clone(),
        method: rpc_req.method.clone(),
    };
    let rpc_method = &rpc_info.method;
    let rpc_params = rpc_req.params;
    let rpc_resources = RpcResources { ctx: Some(ctx), mm };

    let res = rpc_router.call(rpc_method, rpc_resources, rpc_params).await;
    let res = res.map(|v| {
        let body_response = json!({
            "id": rpc_info.id,
            "result": v
        });
        Json(body_response)
    });

    let res: crate::web::Result<_> = res.map_err(crate::web::Error::from);
    let mut res = res.into_response();
    res.extensions_mut().insert(Arc::new(rpc_info));

    res
}
