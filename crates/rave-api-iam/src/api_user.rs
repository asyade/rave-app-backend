use super::{models::IdTokenClaims, Iam};
use axum::{extract::FromRequestParts, http::request::Parts};
use axum_jwks::Token;
use async_graphql::{self, BatchResponse, ServerError};
use rave_core_database::views::external_user::ExternalUserRow;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AnyApiUser {
    Guest,
    Identified(IdentifiedApiUser),
}

#[derive(Debug)]
pub struct IdentifiedApiUser {
    pub claims: IdTokenClaims,
    pub stored: ExternalUserRow,
}

#[async_trait::async_trait]
impl<S: Send + Sync> FromRequestParts<S> for AnyApiUser {
    type Rejection = async_graphql_axum::GraphQLResponse;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match Token::from_request_parts(parts, state).await {
            Ok(token) => {
                let iam = parts
                    .extensions
                    .get::<Iam>()
                    .ok_or_else(|| build_error_response("iam extension not installed"))?;
                let user = iam
                    .api_user_from_token(token.value())
                    .await
                    .map_err(|e| build_error_response(&e.to_string()))?;
                Ok(Self::Identified(user))
            }
            _ => Ok(AnyApiUser::Guest),
        }
    }
}

fn build_error_response(message: &str) -> async_graphql_axum::GraphQLResponse {
    async_graphql_axum::GraphQLResponse(BatchResponse::Single(
        async_graphql::Response::from_errors(vec![ServerError::new(message, None)]),
    ))
}

impl Display for AnyApiUser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyApiUser::Guest => write!(f, "Guest"),
            AnyApiUser::Identified(user) => write!(f, "{}={}", user.claims.email, user.claims.sub),
        }
    }
}
