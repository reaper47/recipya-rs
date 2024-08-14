use crate::{router::Result, RpcResources};

pub trait FromResources {
    fn from_resources(rpc_resources: &RpcResources) -> Result<Self>
    where
        Self: Sized;
}
