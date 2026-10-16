use std::{fmt, io::Write};

use diesel::{
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    serialize::{self, Output, ToSql},
    sql_types::SmallInt,
};

const NO_RATING: u8 = 0;

#[derive(Copy, Clone, Debug, Eq, PartialEq, AsExpression, FromSqlRow)]
#[diesel(sql_type = SmallInt)]
pub struct Rating(u8);

impl From<i16> for Rating {
    fn from(value: i16) -> Self {
        if value > 10 || value == 0 {
            Self(NO_RATING)
        } else {
            Self(u8::try_from(value).unwrap_or_default())
        }
    }
}

impl From<i32> for Rating {
    fn from(value: i32) -> Self {
        if value > 10 || value == 0 {
            Self(NO_RATING)
        } else {
            Self(u8::try_from(value).unwrap_or_default())
        }
    }
}

impl From<u8> for Rating {
    fn from(value: u8) -> Self {
        if value > 10 || value == 0 {
            Self(NO_RATING)
        } else {
            Self(value)
        }
    }
}

impl From<Option<i16>> for Rating {
    fn from(value: Option<i16>) -> Self {
        Self::from(value.unwrap_or_default())
    }
}

impl Default for Rating {
    fn default() -> Self {
        Self(NO_RATING)
    }
}

impl fmt::Display for Rating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_none() {
            write!(f, "")
        } else {
            write!(f, "{}/5", f32::from(self.0) / 2.0)
        }
    }
}

impl Rating {
    /// Returns whether there is no rating.
    pub const fn is_none(&self) -> bool {
        self.0 == NO_RATING
    }

    /// Consumes the `Rating` and returns the underlying number.
    pub const fn into_inner(self) -> u8 {
        self.0
    }

    /// Converts the rating to an option.
    pub fn as_option(&self) -> Option<i16> {
        (!self.is_none()).then_some(i16::from(self.0))
    }
}

impl FromSql<SmallInt, Pg> for Rating {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        Ok(Self::from(i16::from_sql(bytes)?))
    }
}

impl ToSql<SmallInt, Pg> for Rating {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        if self.is_none() {
            return Ok(serialize::IsNull::Yes);
        }

        let val = i16::from(self.0);
        out.write_all(&val.to_be_bytes())?;
        Ok(serialize::IsNull::No)
    }
}
