mod error;

pub use self::error::{Error, Result};

#[derive(Clone)]
pub struct Ctx {
    user_id: i64,
}

impl Ctx {
    pub fn root_ctx() -> Self {
        Ctx { user_id: 0 }
    }

    pub fn new(user_id: i64) -> Result<Self> {
        if user_id == 0 {
            Err(Error::CtxCannotNewRootCtx)
        } else {
            Ok(Self { user_id })
        }
    }
}

// Property accessors
impl Ctx {
    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}
