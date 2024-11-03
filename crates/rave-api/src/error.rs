use thiserror::Error;

use rave_api_service_iam::error::IamError;
use rave_api_service_database::error::DatabaseServiceError;


#[derive(Debug, Error)]
pub enum RaveApiError{
    #[error(transparent)]
    GraphQL(#[from] rave_api_graphql::error::RaveGraphqlError),
    #[error(transparent)]
    Iam(#[from] IamError),
    #[error(transparent)]
    Database(#[from] DatabaseServiceError),
    #[error("http error: {0}")]
    Http(String),
    #[error("wrong configuration: {0}")]
    Config(String),
}

pub type RaveApiResult<T> = Result<T, RaveApiError>;