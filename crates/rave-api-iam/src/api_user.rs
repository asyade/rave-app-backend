use super::models::IdTokenClaims;
use rave_core_database::views::external_user::ExternalUserRow;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum AnyApiUser {
    Guest,
    Identified(IdentifiedApiUser),
}

#[derive(Debug, Clone)]
pub struct IdentifiedApiUser {
    pub claims: IdTokenClaims,
    pub stored: ExternalUserRow,
}

impl Display for AnyApiUser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyApiUser::Guest => write!(f, "Guest"),
            AnyApiUser::Identified(user) => write!(f, "{}={}", user.claims.email, user.claims.sub),
        }
    }
}
