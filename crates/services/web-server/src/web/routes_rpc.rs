use std::sync::Arc;

use axum::{
    response::IntoResponse,
    routing::post,
    {extract::State, response::Response, Json, Router},
};
use serde_json::{json, Value};

use lib_core::model::ModelManager;
use lib_rpc::{router::RpcRouter, user_rpc, RpcRequest, RpcResources};

use crate::web::mw_auth::CtxW;

/// The RpcState is the Axum State that will
/// be used for the Axum RPC router handler.
///
/// Note: Not to be confused with the RpcResources that are for the lib-rpc
///      layer for the RpcRouter System. The RpcResources typically contains some elements
///      from the RpcState
#[derive(Clone)]
pub struct RpcState {
    pub mm: ModelManager,
}

/// RPC basic information containing the rpc request id and method for additional logging purposes.
#[derive(Clone, Debug)]
pub struct RpcInfo {
    pub id: Option<Value>,
    pub method: String,
}

/// Axum router for '/api/rpc'
pub fn routes(rpc_state: RpcState) -> Router {
    let rpc_router = RpcRouter::new().extend(user_rpc::rpc_router());

    Router::new()
        .route("/rpc", post(rpc_axum_handler))
        .with_state((rpc_state, Arc::new(rpc_router)))
}

async fn rpc_axum_handler(
    State((rpc_state, rpc_router)): State<(RpcState, Arc<RpcRouter>)>,
    ctx: CtxW,
    Json(rpc_req): Json<RpcRequest>,
) -> Response {
    let ctx = ctx.0;

    let rpc_info = RpcInfo {
        id: rpc_req.id.clone(),
        method: rpc_req.method.clone(),
    };
    let rpc_method = &rpc_info.method;
    let rpc_params = rpc_req.params;
    let rpc_resources = RpcResources {
        ctx: Some(ctx),
        mm: rpc_state.mm,
    };

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
