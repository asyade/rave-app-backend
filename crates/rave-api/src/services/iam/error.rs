use crate::prelude::*;

#[derive(Debug, Error)]
pub enum IamError {
    #[error(transparent)]
    InvalidIdToken(axum_jwks::TokenError),
    #[error("failed to fetch JWKS: {0}")]
    JwksError(#[from] axum_jwks::JwksError),
    #[error("sqlx error: {0}")]
    DatabaseDriver(#[from] sqlx::Error),
    #[error("failed to accquire database connection")]
    DatabaseUnavailable,
}

pub type IamResult<T> = Result<T, IamError>;
