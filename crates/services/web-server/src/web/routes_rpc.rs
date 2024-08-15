use axum::Router;
use lib_core::model::ModelManager;

///  Build the Axum router for '/api/rpc'
/// Note: This will build the `rpc-router::Router` that will be used by the
///       rpc_axum_handler
pub fn routes(_mm: ModelManager) -> Router {
    todo!()
    /*let rpc_router = all_rpc_router_builder()
        // Add the common resources for all rpc calls
        .append_resource(mm)
        .build();

    // Build the Axum Router for '/rpc'
    Router::new()
        .route("/rpc", post(handlers_rpc::rpc_axum_handler))
        .with_state(rpc_router)*/
}
