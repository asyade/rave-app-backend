use thiserror::Error;

#[derive(Debug, Error)]
pub enum IamError {
    #[error(transparent)]
    InvalidIdToken(axum_jwks::TokenError),    
    #[error("failed to fetch JWKS: {0}")]
    JwksError(#[from] axum_jwks::JwksError),    
}

pub type IamResult<T> = Result<T, IamError>;
