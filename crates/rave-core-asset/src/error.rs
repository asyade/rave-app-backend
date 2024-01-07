use crate::prelude::*;

#[derive(Debug, Error)]
pub enum AssetError {
    #[error(transparent)]
    DatabaseError(#[from] CoreDatabaseError),
    #[error("failed to accquire database connection")]
    DatabaseUnavailable,
}

pub type AssetResult<T> = Result<T, AssetError>;