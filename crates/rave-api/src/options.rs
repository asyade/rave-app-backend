use std::net::SocketAddr;

use rave_api_iam::Auth0Options;
use serde::{Deserialize, Serialize};

use crate::prelude::{RaveApiError, RaveApiResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RaveApiOptions {
    #[cfg(feature = "cors")]
    pub allowed_origins: Vec<String>,
    pub listen_address: SocketAddr,
    pub database_url: String,
    pub auth0: Auth0Options,
}

impl RaveApiOptions {
    pub fn try_from_env() -> RaveApiResult<Self> {
        macro_rules! opt_from_env {
            ($name:expr) => {{
                let ____name = $name;
                std::env::var(____name)
                    .map_err(|_| RaveApiError::Config(format!("{} not set", ____name)))
            }};
        }

        Ok(Self {
            #[cfg(feature = "cors")]
            allowed_origins: opt_from_env!("ALLOWED_ORIGINS")?
                .split(',')
                .map(|s| s.to_string())
                .collect(),
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
