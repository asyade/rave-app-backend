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
use reqwest::Client;

#[derive(Debug)]
pub enum ApiUser {
    Guest,
    Identified { claims: IdTokenClaims },
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for ApiUser {
    type Rejection = async_graphql_axum::GraphQLResponse;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await {
            Ok(auth) => {
                let iam = parts
                    .extensions
                    .get::<Iam>()
                    .ok_or_else(|| build_error_response("iam extension not installed"))?;
                Ok(Self::Identified {
                    claims: iam
                        .id_token_claims(auth.0.token())
                        .await
                        .map_err(|e| build_error_response(&e.to_string()))?,
                })
            }
            _ => Ok(ApiUser::Guest),
        }
    }
}

fn build_error_response(details: &str) -> GraphQLResponse {
    let response =
        async_graphql::Response::from_errors(vec![async_graphql::ServerError::new(details, None)]);
    async_graphql_axum::GraphQLResponse::from(response)
}

impl Display for ApiUser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiUser::Guest => write!(f, "Guest"),
            ApiUser::Identified { claims } => write!(f, "{:?}", claims),
        }
    }
}
