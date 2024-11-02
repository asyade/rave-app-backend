use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

use crate::prelude::{RaveApiError, RaveApiResult};

macro_rules! opt_from_env {
    ($name:expr, $default:expr) => {{
        if let Some(____optional_default) = $default {
            Ok(____optional_default)
        } else {
            opt_from_env!($name)
        }
    }};
    ($name:expr) => {{
        let ____name = $name;
        std::env::var(____name).map_err(|_| RaveApiError::Config(format!("{} not set", ____name)))
    }};
}

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

impl RaveApiOptions {
    pub fn try_from_env(
        listen_address: Option<String>,
        database_url: Option<String>,
    ) -> RaveApiResult<Self> {
        Ok(Self {
            listen_address: opt_from_env!("LISTEN_ADDRESS", listen_address)?
                .parse()
                .map_err(|e| RaveApiError::Config(format!("invalid listen address: {}", e)))?,
            database_url: opt_from_env!("DATABASE_URL", database_url)?,
            auth0: Auth0Options::try_from_env()?,
        })
    }
}

impl Auth0Options {
    pub fn oidc_url(&self) -> String {
        format!("https://{}/.well-known/openid-configuration", self.domain)
    }

    pub fn try_from_env() -> RaveApiResult<Self> {
        Ok(Auth0Options {
            client_id: opt_from_env!("AUTH0_CLIENT_ID")?,
            client_secret: opt_from_env!("AUTH0_CLIENT_SECRET")?,
            domain: opt_from_env!("AUTH0_DOMAIN")?,
            audience: opt_from_env!("AUTH0_AUDIENCE")?,
        })
    }
}
