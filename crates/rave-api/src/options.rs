use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

use crate::prelude::{RaveApiError, RaveApiResult};
use rave_api_service_iam::AuthOptions;

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
    pub auth0: AuthOptions,
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
            auth0: AuthOptions::try_from_env()?,
        })
    }
}
