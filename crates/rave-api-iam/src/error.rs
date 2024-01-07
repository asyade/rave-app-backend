use crate::prelude::*;

#[derive(Debug, Error)]
pub enum IamError {
    #[error(transparent)]
    InvalidIdToken(axum_jwks::TokenError),
    #[error(transparent)]
    JwksError(#[from] axum_jwks::JwksError),
    #[error(transparent)]
    DatabaseError(#[from] CoreDatabaseError),
}

pub type IamResult<T> = Result<T, IamError>;
