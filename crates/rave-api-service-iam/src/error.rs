use thiserror::Error;

#[derive(Debug, Error)]
pub enum IamError {
    #[error(transparent)]
    InvalidIdToken(axum_jwks::TokenError),  
    #[error("failed to fetch JWKS: {0}")]
    JwksError(#[from] axum_jwks::JwksError),  
    #[error("sqlx error: {0}")]
    DatabaseDriver(#[from] rave_entity::sqlx::Error),
    #[error("failed to accquire database connection")]
    DatabaseUnavailable,
    #[error("missing option: {0}")]
    MissingOption(String),
}

pub type IamResult<T> = Result<T, IamError>;
