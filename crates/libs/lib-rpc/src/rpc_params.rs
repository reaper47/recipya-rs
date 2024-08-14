//! Base constructs for the typed RPC Params that will be used in their respective
//! rpc handler functions (e.g., `task_rpc::create_task` and `task_rpc::list_tasks`).
//!
//! Most of these base constructs use generics for their respective data elements, allowing
//! each rpc handler function to receive the exact desired type.
//!

use serde::{de::DeserializeOwned, Deserialize};

use crate::router::IntoParams;

/// Params structure for any RPC Create call.
#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
    pub data: D,
}

impl<D> IntoParams for ParamsForCreate<D> where D: DeserializeOwned + Send {}

#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
    pub id: i64,
    pub data: D,
}

impl<D> IntoParams for ParamsForUpdate<D> where D: DeserializeOwned + Send {}

#[derive(Deserialize)]
pub struct ParamsIded {
    pub id: i64,
}

impl IntoParams for ParamsIded {}
