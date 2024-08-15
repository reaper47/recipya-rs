use rpc_router::{Router, RouterBuilder};

pub mod user_rpc;

pub fn all_rpc_router_builder() -> RouterBuilder {
	Router::builder()
		.extend(user_rpc::rpc_router_builder())
}