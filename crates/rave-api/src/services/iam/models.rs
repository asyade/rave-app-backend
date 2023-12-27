use crate::prelude::*;

/// User claims from Auth0
#[derive(Debug, Serialize, Deserialize)]
pub struct IdTokenClaims {
    pub name: String,
    pub nickname: String,
    pub email: String,
    pub email_verified: bool,
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}
