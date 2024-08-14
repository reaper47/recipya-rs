use lib_core::{
    ctx::Ctx,
    model::{
        user::{User, UserBmc, UserForCreate},
        ModelManager,
    },
};

use crate::{
    params::{ParamsForCreate, ParamsIded},
    router::RpcRouter,
    rpc_router, Result,
};

pub fn rpc_router() -> RpcRouter {
    rpc_router!(create_user, delete_user,)
}

pub async fn create_user(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<UserForCreate>,
) -> Result<User> {
    let ParamsForCreate { data } = params;

    let id = UserBmc::create(&ctx, &mm, data).await?;
    let user = UserBmc::get(&ctx, &mm, id).await?;

    Ok(user)
}

pub async fn delete_user(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<User> {
    let ParamsIded { id } = params;

    let user = UserBmc::get(&ctx, &mm, id).await?;
    UserBmc::delete(&ctx, &mm, id).await?;

    Ok(user)
}
