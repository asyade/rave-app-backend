use thiserror::Error;

#[derive(Debug, Error)]
pub enum RaveApiError{
    #[error("sqlx error: {0}")]
    DatabaseDriver(#[from] sqlx::Error),
    #[error("wrong database configuration: {0}")]
    DatabaseConfig(String)
}

pub type RaveApiResult<T> = Result<T, RaveApiError>;