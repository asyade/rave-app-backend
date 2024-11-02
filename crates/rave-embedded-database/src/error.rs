use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmbeddedDatabaseError {
    #[error(transparent)]
    Database(#[from] postgresql_embedded::Error),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    SqlxMigrate(#[from] sqlx::migrate::MigrateError),
}

pub type EmbeddedDatabaseResult<T> = Result<T, EmbeddedDatabaseError>;