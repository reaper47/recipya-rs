use diesel::{Associations, Identifiable, Queryable, Selectable};
use serde::Serialize;
use uuid::Uuid;

#[derive(Associations, Queryable, Identifiable, Selectable)]
#[diesel(belongs_to(super::user::User))]
#[diesel(table_name = super::schema::recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recipe {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<Uuid>,
    pub yield_: i16,
    pub language: String,
    pub source: Option<String>,
    pub user_id: i64,
}

pub struct RecipeWithData {
    pub recipe: Recipe,
    pub category: String,
    pub cuisine: Option<String>,
}

/// Recipe backend model controller.
pub struct RecipeBmc;

impl RecipeBmc {}

#[cfg(test)]
mod tests {
    use super::*;

    pub type Result<T> = core::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>;

    #[tokio::test]
    async fn test_create_ok() {
        todo!()
    }
}
