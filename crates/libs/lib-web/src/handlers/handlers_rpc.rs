use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use rpc_router::resources_builder;
use serde_json::{json, Value};

use crate::middleware::mw_auth::CtxW;

/// RPC basic information containing the rpc request id and method for additional logging purposes.
#[derive(Clone, Debug)]
pub struct RpcInfo {
    pub id: Option<Value>,
    pub method: String,
}

pub async fn rpc_axum_handler(
    State(rpc_router): State<rpc_router::Router>,
    ctx: CtxW,
    Json(rpc_req): Json<Value>,
) -> Response {
    let ctx = ctx.0;

    let rpc_req = match rpc_router::Request::try_from(rpc_req) {
        Ok(rpc_req) => rpc_req,
        Err(rpc_req_error) => {
            let res = crate::error::Error::RpcRequestParsing(rpc_req_error).into_response();
            return res;
        }
    };

    let rpc_info = RpcInfo {
        id: Some(rpc_req.id.clone()),
        method: rpc_req.method.clone(),
    };

    let additional_resources = resources_builder![ctx].build();

    let rpc_call_result = rpc_router
        .call_with_resources(rpc_req, additional_resources)
        .await;

    let res = rpc_call_result.map(|rpc_call_response| {
        let body_response = json!({
            "jsonrpc": "2.0",
            "id": rpc_call_response.id,
            "result": rpc_call_response.value,
        });
        Json(body_response)
    });

    let res: crate::error::Result<_> = res.map_err(crate::error::Error::from);
    let mut res = res.into_response();
    res.extensions_mut().insert(Arc::new(rpc_info));

    res
}
