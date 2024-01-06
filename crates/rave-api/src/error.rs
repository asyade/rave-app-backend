use crate::prelude::*;

#[derive(Debug, Error)]
pub enum RaveApiError{
    #[error("sqlx error: {0}")]
    DatabaseDriver(#[from] sqlx::Error),
    #[error("wrong database configuration: {0}")]
    DatabaseConfig(String),
    #[error("wrong configuration: {0}")]
    Config(String),
}

pub type RaveApiResult<T> = Result<T, RaveApiError>;