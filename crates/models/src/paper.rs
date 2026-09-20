use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use indexmap::IndexMap;
use repository::{ModelManager, schema};

use crate::Result;

/// A map of paper sizes organized by category.
pub type PaperSizes = IndexMap<String, Vec<PaperSize>>;

/// The entity for the paper sizes table.
#[derive(Debug, PartialEq, Associations, Queryable, Selectable)]
#[diesel(belongs_to(PaperCategory))]
#[diesel(table_name = schema::paper_sizes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PaperSize {
    pub id: i16,
    pub paper_category_id: i16,
    pub name: String,
    pub height_mm: f64,
    pub width_mm: f64,
    pub height_in: f64,
    pub width_in: f64,
}

impl Eq for PaperSize {}

impl PaperSize {
    /// Fetches a paper size by id.
    pub async fn get(mm: &ModelManager, id: i16) -> Result<Self> {
        let paper_size = schema::paper_sizes::table
            .filter(schema::paper_sizes::id.eq(id))
            .select(Self::as_select())
            .first::<Self>(&mut mm.pool.get().await?)
            .await?;

        Ok(paper_size)
    }

    /// Fetches all the paper sizes organized by category.
    pub async fn get_all(mm: &ModelManager) -> Result<PaperSizes> {
        let values = schema::paper_sizes::table
            .inner_join(
                schema::paper_categories::table
                    .on(schema::paper_categories::id.eq(schema::paper_sizes::paper_category_id)),
            )
            .select((schema::paper_categories::name, Self::as_select()))
            .get_results::<(String, Self)>(&mut mm.pool.get().await?)
            .await?;

        Ok(values
            .into_iter()
            .fold(IndexMap::new(), |mut map, (category, paper_size)| {
                map.entry(category).or_default().push(paper_size);
                map
            }))
    }
}

/// The entity for the paper categories table.
#[derive(Debug, Eq, PartialEq, Selectable)]
#[diesel(table_name = schema::paper_categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct PaperCategory {
    id: i16,
    name: String,
}
