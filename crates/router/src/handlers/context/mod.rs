mod error;

use uuid::Uuid;

pub use self::error::{Error, Result};

/// Represents a context associated with a user, identified by a unique user ID.
///
/// A `Ctx` is typically used to store information about the current user's session or state.
#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: Uuid,
    is_admin: bool,
}

impl Ctx {
    /// Creates a root context, which always has a user ID of zero.
    pub fn root_ctx() -> Self {
        Self {
            user_id: Uuid::nil(),
            is_admin: false,
        }
    }

    /// Creates a context for the user based on its user ID.
    pub fn new(user_id: Uuid, is_admin: bool) -> Result<Self> {
        if user_id.is_nil() {
            Err(Error::CtxCannotBeNewRootCtx)
        } else {
            Ok(Self { user_id, is_admin })
        }
    }
}

impl Ctx {
    /// Returns the context's user ID.
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn is_admin(&self) -> bool {
        self.is_admin
    }

    pub fn is_root(&self) -> bool {
        self.user_id.is_nil()
    }
}
