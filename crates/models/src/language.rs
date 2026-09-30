use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use repository::{ModelManager, schema};

use crate::Result;

/// Represents the database's language entity.
#[derive(Debug, Eq, PartialEq, Queryable, Selectable)]
#[diesel(table_name = schema::languages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Language {
    pub id: i64,
    pub locale: String,
    pub name: String,
    pub english_name: String,
}

impl Language {
    /// Gets all the languages for internationalisation.
    pub async fn get_all(mm: &ModelManager) -> Result<Vec<Self>> {
        Ok(schema::languages::table
            .load::<Self>(&mut mm.pool.get().await?)
            .await?)
    }
}
