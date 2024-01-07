use crate::prelude::*;

#[derive(Debug, Error)]
pub enum CoreDatabaseError {
    #[error("the created entity cannot be found")]
    CreatedEntityNotFound,
    #[error("sqlx error: {0}")]
    DatabaseDriver(#[from] sqlx::Error),
    #[error("failed to accquire database connection")]
    DatabaseUnavailable,
}

pub type CoreDatabaseResult<T> = Result<T, CoreDatabaseError>;

impl CoreDatabaseError {
    pub fn is_not_found(&self) -> bool {
        match self {
            Self::DatabaseDriver(sqlx::Error::RowNotFound) => true,
            _ => false,
        }
    }
}

pub trait CoreDatabaseResultExt<T> {
    fn not_found_to_none(self) -> CoreDatabaseResult<Option<T>>;
}

impl<T, E: Into<CoreDatabaseError>> CoreDatabaseResultExt<T> for Result<T, E> {
    fn not_found_to_none(self) -> CoreDatabaseResult<Option<T>> {
        match self.map_err(|e| e.into()) {
            Ok(v) => Ok(Some(v)),
            Err(e) if e.is_not_found() => Ok(None),
            Err(e) => Err(e),
        }
    }
}

