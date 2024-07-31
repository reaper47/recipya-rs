mod user_rpc;

use axum::{{extract::State, Json, response::Response, Router}, middleware, response::IntoResponse, routing::post};
use serde_json::{from_value, json, to_value, Value};
use serde_with::serde_derive::Deserialize;

use crate::{ctx::Ctx, model::ModelManager, web::{Error, Result, rpc::user_rpc::create_user}, web};
use crate::web::rpc::user_rpc::delete_user;

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
		$rpc_fn($ctx, $mm, params).await.map(to_value)??
	}};

	// Without Params
	($rpc_fn:expr, $ctx:expr, $mm:expr) => {
		$rpc_fn($ctx, $mm).await.map(to_value)??
	};
}

#[derive(Deserialize)]
struct RpcRequest {
    id: Option<Value>,
    method: String,
    params:Option<Value>,
}

#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
    data: D,
}

#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
    id: i64,
    data: D,
}

#[derive(Deserialize)]
pub struct ParamsIded {
    id: i64,
}

#[derive(Clone)]
pub struct RpcInfo {
    pub id: Option<Value>,
    pub method: String,
}

pub fn routes(mm: ModelManager)->Router {
    Router::new()
        .route("/rpc", post(rpc_handler))
        .route_layer(middleware::from_fn(web::middleware::ctx::require))
        .with_state(mm)
}

async fn rpc_handler(State(mm): State<ModelManager>, ctx: Ctx, Json(rpc_req): Json<RpcRequest>) -> Response {
    let rpc_info = RpcInfo {
      id:   rpc_req.id.clone(),
        method:  rpc_req.method.clone(),
    };

    // Exec and store RpcInfo in the response.
    let mut res = _rpc_handler(ctx, mm, rpc_req).await.into_response();
    res.extensions_mut().insert(rpc_info);

    res
}

async fn _rpc_handler(ctx: Ctx, mm: ModelManager, rpc_req: RpcRequest) -> Result<Json<Value>> {
    let RpcRequest {
        id: rpc_id,
        method: rpc_method,
        params: rpc_params
    } = rpc_req;

    let result_json: Value = match rpc_method.as_str() {
        "create_user" => exec_rpc_fn!(create_user, ctx, mm, rpc_params),
        //"list_tasks" => exec_rpc_fn!(list_tasks, ctx, mm),
        "delete_user" => exec_rpc_fn!(delete_user, ctx, mm, rpc_params),

        _=> return Err(Error::RpcMethodUnknown(rpc_method)),
    };

    let body_response = json!({
        "id": rpc_id,
        "result": result_json,
    });

    Ok(Json(body_response))
}