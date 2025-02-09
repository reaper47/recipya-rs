mod error;

pub use self::error::{Error, Result};

/// Represents a context associated with a user, identified by a unique user ID.
///
/// A `Ctx` is typically used to store information about the current user's session or state.
#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: i64,
}

impl Ctx {
    /// Creates a root context, which always has a user ID of zero.
    pub fn root_ctx() -> Self {
        Self { user_id: 0 }
    }

    /// Creates a context for the user based on its user ID.
    pub fn new(user_id: i64) -> Result<Self> {
        if user_id == 0 {
            Err(Error::CtxCannotBeNewRootCtx)
        } else {
            Ok(Self { user_id })
        }
    }
}

impl Ctx {
    /// Returns the context's user ID.
    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}
