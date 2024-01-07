use crate::prelude::*;
use axum_jwks::Jwks;
use error::*;
use rave_core_database::database::ExternalUserCursor;

use self::{
    api_user::{AnyApiUser, IdentifiedApiUser},
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
        Self::api_user_from_claims(&self.database, claims, true).await
    }

    #[async_recursion]
    async fn api_user_from_claims(
        database: &Database,
        claims: IdTokenClaims,
        allow_recursion: bool,
    ) -> IamResult<IdentifiedApiUser> {
        let stored_user = database
            .acquire()
            .await?
            .find_external_user_by_external_user_id(&claims.sub)
            .await?;

        match stored_user {
            Some(user) => Ok(IdentifiedApiUser {
                stored: user,
                claims,
            }),
            None => {
                info!("no stored user found, creating record");
                let mut conn = database
                    .acquire()
                    .await?
                    .create_external_user(&claims.sub, &claims.email, &claims.name)
                    .await?;
                Self::api_user_from_claims(database, claims, false).await
            }
        }
    }
}

impl Auth0Options {
    pub fn oidc_url(&self) -> String {
        format!("https://{}/.well-known/openid-configuration", self.domain)
    }
}
