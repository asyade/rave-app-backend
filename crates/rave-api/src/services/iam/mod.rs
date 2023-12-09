use crate::{options::Auth0Options, prelude::*};
use axum::{
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    response::Response,
    TypedHeader,
};
use axum_jwks::Jwks;
use error::*;
use reqwest::Client;

use self::models::IdTokenClaims;

pub mod api_user;
pub mod error;
pub mod models;

#[derive(Clone)]
pub struct Iam {
    pub client: Arc<Client>,
    pub auth0_options: Auth0Options,
    jwks: Arc<RwLock<Jwks>>,
}

impl Iam {
    #[instrument(skip(auth0_options), err, fields(domain = %auth0_options.domain, audience = %auth0_options.audience))]
    pub async fn init(auth0_options: Auth0Options) -> IamResult<Self> {
        tracing::info!("feetching jwks");
        let jwks =
            Jwks::from_oidc_url(&auth0_options.oidc_url(), auth0_options.audience.clone()).await?;

        Ok(Self {
            client: Arc::new(Client::new()),
            jwks: Arc::new(RwLock::new(jwks)),
            auth0_options: auth0_options.clone(),
        })
    }

    pub async fn id_token_claims(&self, token: &str) -> IamResult<IdTokenClaims> {
        self
            .jwks
            .read()
            .await
            .validate_claims::<IdTokenClaims>(dbg!(token))
            .map_err(IamError::InvalidIdToken)
            .map(|data| data.claims)
    }
}
