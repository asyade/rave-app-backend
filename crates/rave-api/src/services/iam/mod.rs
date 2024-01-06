use std::borrow::Borrow;

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
use rave_entity::graph::user::ExternalUserViewRow;
use reqwest::Client;
use sqlx::PgPool;

use self::{
    api_user::{AnyApiUser, IdentifiedApiUser},
    models::IdTokenClaims,
};

pub mod api_user;
pub mod error;
pub mod models;

#[derive(Clone)]
pub struct Iam {
    pub client: Arc<Client>,
    pub auth0_options: Auth0Options,
    jwks: Arc<RwLock<Jwks>>,
    database: Database,
}

impl Iam {
    #[instrument(skip(auth0_options, database), err, fields(domain = %auth0_options.domain, audience = %auth0_options.audience))]
    pub async fn init(database: Database, auth0_options: Auth0Options) -> IamResult<Self> {
        let jwks =
            Jwks::from_oidc_url(&auth0_options.oidc_url(), auth0_options.audience.clone()).await?;

        Ok(Self {
            database,
            client: Arc::new(Client::new()),
            jwks: Arc::new(RwLock::new(jwks)),
            auth0_options: auth0_options.clone(),
        })
    }

    #[instrument(skip(self, token), err)]
    pub async fn api_user_from_token(&self, token: &str) -> IamResult<IdentifiedApiUser> {
        let claims = self
            .jwks
            .read()
            .await
            .validate_claims::<IdTokenClaims>(token)
            .map_err(IamError::InvalidIdToken)
            .map(|data| data.claims)?;
        tracing::Span::current().record("sub", &claims.sub.as_str());
        tracing::Span::current().record("email", &claims.email.as_str());
        Self::api_user_from_claims(&self.database, claims, true).await
    }

    #[async_recursion]
    async fn api_user_from_claims(
        database: &Database,
        claims: IdTokenClaims,
        allow_recursion: bool,
    ) -> IamResult<IdentifiedApiUser> {
        let mut conn = database
            .acquire()
            .await
            .map_err(|_| IamError::DatabaseUnavailable)?;
        let stored_user = sqlx::query_as::<_, ExternalUserViewRow>(
            r#"
                SELECT external_user_id, entity_sid, email, name
                FROM public_users
                WHERE external_user_id = $1
            "#,
        )
        .bind(&claims.sub)
        .fetch_one(conn.as_mut())
        .await;

        match stored_user {
            Ok(user) => {
                Ok(IdentifiedApiUser {
                    stored: user,
                    claims,
                })
            }
            Err(sqlx::Error::RowNotFound) if allow_recursion => {
                info!("no stored user found, creating record");
                let mut conn = database
                    .acquire()
                    .await
                    .map_err(|_| IamError::DatabaseUnavailable)?;

                sqlx::query(
                    r#"
                        with created_entity as (
                            insert into public.entities (sid, uid)
                                values (default, default) returning sid
                        )
                        insert into public.external_identities
                            (sid, external_user_id, email, name, entity_sid)
                            values (default, $1, $2, $3, (select sid from created_entity))
                    "#,
                    )
                    .bind(&claims.sub)
                    .bind(&claims.email)
                    .bind(&claims.name)
                    .execute(conn.as_mut())
                    .await?;
                Self::api_user_from_claims(database, claims, false).await
            }
            Err(e) => Err(IamError::DatabaseDriver(e)),
        }
    }
}
