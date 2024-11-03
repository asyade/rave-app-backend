use crate::prelude::*;

macro_rules! opt_from_env {
    ($name:expr) => {{
        let ____name = $name;
        std::env::var(____name).map_err(|_| IamError::MissingOption(format!("{} not set", ____name)))
    }};
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthOptions {
    pub domain: String,
    pub audience: String,
}

impl AuthOptions {
    pub fn oidc_url(&self) -> String {
        format!("https://{}/.well-known/openid-configuration", self.domain)
    }

    pub fn try_from_env() -> IamResult<Self> {
        Ok(AuthOptions {
            domain: opt_from_env!("OIDC_DOMAIN")?,
            audience: opt_from_env!("OIDC_AUDIENCE")?,
        })
    }
}
