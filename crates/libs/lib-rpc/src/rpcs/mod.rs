use crate::router::RpcRouter;

pub mod user_rpc;

pub fn all_rpc_router() -> RpcRouter {
    RpcRouter::new().extend(user_rpc::rpc_router())
}
