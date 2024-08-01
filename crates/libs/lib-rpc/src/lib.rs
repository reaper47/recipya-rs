use serde::Deserialize;
use serde_json::{from_value, to_value, Value};

use lib_core::{ctx::Ctx, model::ModelManager};

use crate::user_rpc::{create_user, delete_user};

pub use self::error::{Error, Result};

mod error;
mod params;
mod user_rpc;

/// The raw JSON-RPC request object, serving as the foundation for RPC routing.
#[derive(Deserialize)]
pub struct RpcRequest {
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

macro_rules! exec_rpc_fn {
    // With Params
    ($rpc_fn:expr, $ctx:expr, $mm:expr, $rpc_params:expr) => {{
        let rpc_fn_name = stringify!($rpc_fn);
        let params = $rpc_params.ok_or(Error::RpcMissingParams {
            rpc_method: rpc_fn_name.to_string(),
        })?;
        let params = from_value(params).map_err(|_| Error::RpcFailJsonParams {
            rpc_method: rpc_fn_name.to_string(),
        })?;
        $rpc_fn($ctx, $mm, params)
            .await
            .map(to_value)?
            .map_err(|ex| Error::SerdeJson(ex.to_string()))?
    }};

    // Without Params
    ($rpc_fn:expr, $ctx:expr, $mm:expr) => {
        $rpc_fn($ctx, $mm).await.map(to_value)??
    };
}

pub async fn exec_rpc(ctx: Ctx, mm: ModelManager, rpc_req: RpcRequest) -> Result<Value> {
    let rpc_method = rpc_req.method;
    let rpc_params = rpc_req.params;

    let result_json: Value = match rpc_method.as_str() {
        "create_task" => exec_rpc_fn!(create_user, ctx, mm, rpc_params),
        "delete_task" => exec_rpc_fn!(delete_user, ctx, mm, rpc_params),

        _ => return Err(Error::RpcMethodUnknown(rpc_method)),
    };

    Ok(result_json)
}
