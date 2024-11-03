use thiserror::Error;

#[derive(Debug, Error)]
pub enum RaveGraphqlError{
    #[error("http error: {0}")]
    Http(String),
    #[error("database error: {0}")]
    DatabaseDriver(#[from] rave_entity::sqlx::Error),
}

pub type RaveApiResult<T> = Result<T, RaveGraphqlError>;