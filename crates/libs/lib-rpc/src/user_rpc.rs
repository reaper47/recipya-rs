use lib_core::{
    ctx::Ctx,
    model::ModelManager,
    model::user::{User, UserBmc, UserForCreate},
};

use crate::{
    params::{ParamsForCreate, ParamsIded},
    Result,
};

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
