use diesel_async::RunQueryDsl;
use serde::de::DeserializeOwned;
use crate::{ctx::Ctx, model::{ModelManager}};
use crate::model::{Error, Result};

pub trait DbBmc {
    const TABLE: &'static str;
}


pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where
    MC: DbBmc,
    E: DeserializeOwned + Send + Sync + 'static,
{
    use diesel::{
        dsl::sql, sql_types::BigInt};

    let db = mm.db();

    let query = format!("SELECT * FROM {} WHERE id = {}", MC::TABLE, id);
    let entity = sql::<BigInt>(&query)
        .get_result::<E>(
            &mut db.get()
                .await
                .map_err(Error::from)?);

    Ok(entity.)
}
