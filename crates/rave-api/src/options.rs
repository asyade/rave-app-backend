use std::net::SocketAddr;

use serde::{Serialize, Deserialize};

use crate::prelude::{RaveApiError, RaveApiResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RaveApiOptions {
    pub listen_address: SocketAddr,
    pub database_url: String,
    pub auth0: Auth0Options,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Auth0Options {
    pub client_id: String,
    pub client_secret: String,
    pub domain: String,
    pub audience: String,
}

macro_rules! opt_from_env {
    ($name:expr) => {{
        let ____name = $name;
        std::env::var(____name).map_err(|_| RaveApiError::Config(format!("{} not set", ____name)))
    }};
}

impl RaveApiOptions {
    pub fn try_from_env() -> RaveApiResult<Self> {
        Ok(Self {
            listen_address: opt_from_env!("LISTEN_ADDRESS")?
                .parse()
                .map_err(|e| RaveApiError::Config(format!("invalid listen address: {}", e)))?,
            database_url: opt_from_env!("DATABASE_URL")?,
            auth0: Auth0Options {
                client_id: opt_from_env!("AUTH0_CLIENT_ID")?,
                client_secret: opt_from_env!("AUTH0_CLIENT_SECRET")?,
                domain: opt_from_env!("AUTH0_DOMAIN")?,
                audience: opt_from_env!("AUTH0_AUDIENCE")?,
            },
        })
    }
}

impl Auth0Options {
    pub fn oidc_url(&self) -> String {
        format!("https://{}/.well-known/openid-configuration", self.domain)
    }
}