//! Model Layer
//!
//! Design:
//!
//! - The Model layer normalizes the application's data type
//!   structures and access.
//! - All application code data access must go through the Model layer.
//! - The `ModelManager` holds the internal states/resources
//!   needed by ModelControllers to access data.
//!   (e.g., db_pool, S3 client, redis client).
//! - Model Controllers (e.g., `TaskBmc`, `ProjectBmc`) implement
//!   CRUD and other data access methods on a given "entity"
//!   (e.g., `Task`, `Project`).
//!   (`Bmc` is short for Backend Model Controller).
//! - In frameworks like Axum, Tauri, `ModelManager` are typically used as App State.
//! - ModelManager are designed to be passed as an argument
//!   to all Model Controllers functions.
//!

mod error;
mod recipe;
pub(in crate::model) mod schema;
pub mod store;
pub mod user;

use diesel_async::{pooled_connection::bb8, AsyncPgConnection};

use crate::config::config;
use crate::model::store::{new_db_pool, Pool};
use lib_email::Sendgrid;

pub use self::error::{Error, Result};

#[derive(Clone)]
pub struct ModelManager {
    pub db: Pool,
    pub email: Option<Sendgrid>,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            db: new_db_pool(&config().DB_URL).await?,
            email: Some(Sendgrid::new()),
        })
    }

    pub(in crate::model) async fn connection(
        &self,
    ) -> Result<bb8::PooledConnection<AsyncPgConnection>> {
        Ok(self.db.get().await?)
    }
}

#[macro_export]
macro_rules! name_entity_with_relations {
    ($struct_name:ident, $table_name:ident, $relation_table:ident) => {
        #[derive(Queryable, Identifiable, Selectable)]
        #[diesel(table_name = super::schema::$table_name)]
        #[diesel(check_for_backend(diesel::pg::Pg))]
        pub struct $struct_name {
            pub id: i64,
            pub name: String,
        }

        paste::paste! {
            #[derive(Insertable)]
            #[diesel(table_name = schema::$table_name)]
            struct [<$struct_name ForInsert>] {
                name: String,
            }
        }

        paste::paste! {
            #[derive(Identifiable, Selectable, Queryable, Insertable, Associations)]
            #[diesel(belongs_to($struct_name), belongs_to(Recipe))]
            #[diesel(table_name = super::schema::$relation_table)]
            #[diesel(primary_key([<$struct_name:lower _id>], recipe_id))]
            pub struct [<$struct_name Recipe>] {
                pub [<$struct_name:lower _id>]: i64,
                pub recipe_id: i64,
            }
        }
    };
}
