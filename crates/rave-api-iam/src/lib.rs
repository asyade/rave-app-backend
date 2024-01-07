use crate::prelude::*;
use axum_jwks::Jwks;
use error::*;
use rave_core_database::{database::ExternalUserCursor, views::external_user::ExternalUserId};

use self::{
    api_user::IdentifiedApiUser,
    models::IdTokenClaims,
};

pub mod api_user;
pub mod error;
pub mod models;
pub mod prelude;

#[derive(Clone)]
pub struct Iam {
    // pub client: Arc<Client>,
    pub auth0_options: Auth0Options,
    jwks: Arc<RwLock<Jwks>>,
    database: Database,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Auth0Options {
    pub client_id: String,
    pub client_secret: String,
    pub domain: String,
    pub audience: String,
}

impl Iam {
    #[instrument(skip(auth0_options, database), err, fields(domain = %auth0_options.domain, audience = %auth0_options.audience))]
    pub async fn init(database: Database, auth0_options: Auth0Options) -> IamResult<Self> {
        let jwks =
            Jwks::from_oidc_url(&auth0_options.oidc_url(), auth0_options.audience.clone()).await?;

        Ok(Self {
            database,
            // client: Arc::new(Client::new()),
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
        Self::api_user_from_claims(&self.database, claims).await
    }

    async fn api_user_from_claims(
        database: &Database,
        claims: IdTokenClaims,
    ) -> IamResult<IdentifiedApiUser> {
        let external_user_id = ExternalUserId::from(claims.sub.clone());
        let stored_user = database
            .acquire()
            .await?
            .find_external_user_by_external_user_id(&external_user_id)
            .await?;
        Ok(IdentifiedApiUser {
            stored: if let Some(user) = stored_user {
                user
            } else {
                database
                    .acquire()
                    .await?
                    .create_external_user(&external_user_id, &claims.email, &claims.name)
                    .await?
            },
            claims,
        })
    }
}

impl Auth0Options {
    pub fn oidc_url(&self) -> String {
        format!("https://{}/.well-known/openid-configuration", self.domain)
    }
}
