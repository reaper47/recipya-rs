use crate::{
    ctx::Ctx,
    model::{ModelManager, Result},
};
use deadpool_postgres::tokio_postgres::Row;

pub trait DbBmc {
    const TABLE: &'static str;
}

pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where
    MC: DbBmc,
    E: for<'r> TryFrom<Row> + Unpin + Send,
{
    todo!()
}
