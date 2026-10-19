use core::fmt;
use std::{
    io::Write,
    ops::{Add, Sub},
};

use diesel::{
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    serialize::{self, Output, ToSql},
    sql_types::Integer,
};

/// Fifteen minutes expressed in seconds.
pub const FIFTEEN_MINUTES_SEC: u32 = 15 * 60;

/// Thirty minutes expressed in seconds.
pub const THIRTY_MINUTES_SEC: u32 = 30 * 60;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Integer)]
pub struct DurationSeconds(u32);

impl DurationSeconds {
    /// Creates a new `DurationSeconds` from a number of seconds.
    pub const fn new(seconds: u32) -> Self {
        Self(seconds)
    }
}

impl From<i32> for DurationSeconds {
    fn from(seconds: i32) -> Self {
        Self(u32::try_from(seconds).unwrap_or_default())
    }
}

impl From<u32> for DurationSeconds {
    fn from(seconds: u32) -> Self {
        Self(seconds)
    }
}

impl From<u64> for DurationSeconds {
    fn from(seconds: u64) -> Self {
        Self(u32::try_from(seconds).unwrap_or(u32::MAX))
    }
}

impl From<DurationSeconds> for i32 {
    fn from(val: DurationSeconds) -> Self {
        Self::try_from(val.0).unwrap_or_default()
    }
}

impl fmt::Display for DurationSeconds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}s", self.0)
    }
}

impl Sub for DurationSeconds {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new((self.0 - rhs.0).clamp(u32::MIN, u32::MAX))
    }
}

impl Add for DurationSeconds {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new((self.0 + rhs.0).clamp(u32::MIN, u32::MAX))
    }
}

impl PartialEq<u32> for DurationSeconds {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u32> for DurationSeconds {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl FromSql<Integer, Pg> for DurationSeconds {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let value = i32::from_sql(bytes)?;
        Ok(Self(u32::try_from(value).unwrap_or(0)))
    }
}

impl ToSql<Integer, Pg> for DurationSeconds {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let val = i32::try_from(self.0).unwrap_or_default();
        out.write_all(&val.to_be_bytes())?;
        Ok(serialize::IsNull::No)
    }
}
