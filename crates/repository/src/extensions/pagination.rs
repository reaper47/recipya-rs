use diesel::pg::Pg;
use diesel::query_builder::{AstPass, Query, QueryFragment};
use diesel::sql_types::BigInt;
use diesel::{QueryId, QueryResult};

pub trait Paginate: Sized {
    fn paginate(self, page: i64, results_per_page: i64) -> Paginated<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, page: i64, results_per_page: i64) -> Paginated<Self> {
        Paginated {
            query: self,
            page,
            per_page: results_per_page,
            offset: (page - 1) * results_per_page,
        }
    }
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    page: i64,
    per_page: i64,
    offset: i64,
}

impl<T> Paginated<T> {
    #[must_use]
    pub fn per_page(self, per_page: i64) -> Self {
        Self {
            per_page,
            offset: (self.page - 1) * self.per_page,
            ..self
        }
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = T::SqlType;
}

impl<T> QueryFragment<Pg> for Paginated<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(" LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&self.offset)?;
        Ok(())
    }
}
