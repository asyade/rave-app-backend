use crate::prelude::*;

#[derive(Debug, Error)]
pub enum RaveApiError{
    #[error(transparent)]
    Database(#[from] CoreDatabaseError),
    #[error("wrong configuration: {0}")]
    Config(String),
}

pub type RaveApiResult<T> = Result<T, RaveApiError>;