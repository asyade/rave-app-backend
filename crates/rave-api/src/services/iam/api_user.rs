use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use super::{error::IamError, models::IdTokenClaims, Iam};
use crate::{graphql, prelude::*};
use async_graphql_axum::GraphQLResponse;
use axum::{
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    response::Response,
    TypedHeader,
};
use rave_entity::iam::user::PublicUser;
use reqwest::Client;

#[derive(Debug)]
pub enum AnyApiUser {
    Guest,
    Identified(IdentifiedApiUser),
}

#[derive(Debug)]
pub struct IdentifiedApiUser {
    pub claims: IdTokenClaims,
    pub stored: PublicUser,
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for AnyApiUser {
    type Rejection = async_graphql_axum::GraphQLResponse;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await {
            Ok(auth) => {
                let iam = parts
                    .extensions
                    .get::<Iam>()
                    .ok_or_else(|| build_error_response("iam extension not installed"))?;

                let user = iam
                    .api_user_from_token(auth.0.token())
                    .await
                    .map_err(|e| build_error_response(&e.to_string()))?;

                Ok(Self::Identified(user))
            }
            _ => Ok(AnyApiUser::Guest),
        }
    }
}

fn build_error_response(details: &str) -> GraphQLResponse {
    let response =
        async_graphql::Response::from_errors(vec![async_graphql::ServerError::new(details, None)]);
    async_graphql_axum::GraphQLResponse::from(response)
}

impl Display for AnyApiUser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyApiUser::Guest => write!(f, "Guest"),
            AnyApiUser::Identified(user) => write!(f, "{}={}", user.claims.email, user.claims.sub),
        }
    }
}
