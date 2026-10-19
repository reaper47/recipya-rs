use core::fmt;
use std::io::Write;

use diesel::{
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    serialize::{self, Output, ToSql},
    sql_types::SmallInt,
};

const DEFAULT_YIELD: u16 = 4;

/// The number of servings a recipe makes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, AsExpression, FromSqlRow)]
#[diesel(sql_type = SmallInt)]
pub struct RecipeYield(u16);

impl Default for RecipeYield {
    fn default() -> Self {
        Self(DEFAULT_YIELD)
    }
}

impl From<i16> for RecipeYield {
    fn from(value: i16) -> Self {
        if value <= 1 {
            Self::default()
        } else {
            Self(value.cast_unsigned())
        }
    }
}

impl From<Option<i16>> for RecipeYield {
    fn from(value: Option<i16>) -> Self {
        Self::from(value.unwrap_or_default())
    }
}

impl fmt::Display for RecipeYield {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// AI usage:
// Claude Sonnet 5.5 assisted in writing this because I couldn't find any documentation about this.
impl FromSql<SmallInt, Pg> for RecipeYield {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let value = i16::from_sql(bytes)
            .map_or(DEFAULT_YIELD, |v| u16::try_from(v).unwrap_or(DEFAULT_YIELD));

        Ok(Self(value))
    }
}

// AI usage:
// Claude Sonnet 5.5 assisted in writing this because I couldn't find any documentation about this.
impl ToSql<SmallInt, Pg> for RecipeYield {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let val = self.0.cast_signed();
        out.write_all(&val.to_be_bytes())?;
        Ok(serialize::IsNull::No)
    }
}

impl RecipeYield {
    /// Consumes the `RecipeYield` and returns the underlying number.
    pub const fn into_inner(self) -> u16 {
        self.0
    }
}
