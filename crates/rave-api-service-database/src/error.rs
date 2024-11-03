use thiserror::Error;

pub type DatabaseServiceResult<T> = Result<T, DatabaseServiceError>;

#[derive(Debug, Error)]
pub enum DatabaseServiceError {
    #[error("failed to create connections pool: {0}")]
    PoolCreation(#[from] sqlx::Error),
    #[error("database config error: {0}")]
    DatabaseConfig(String),
}
