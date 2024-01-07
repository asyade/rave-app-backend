use crate::prelude::*;
use super::models::IdTokenClaims;
use rave_core_database::{tables::entity::EntitySid, views::external_user::ExternalUserRow};
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

impl AnyApiUser {
    pub fn entity_sid(&self) -> Option<EntitySid> {
        match self {
            AnyApiUser::Guest => None,
            AnyApiUser::Identified(user) => Some(user.stored.entity_sid.clone()),
        }
    }

    pub fn require_entity_sid(&self) -> IamResult<EntitySid> {
        self.entity_sid()
            .ok_or_else(|| IamError::GuestUserHasNoEntitySid)
    }
}

impl Display for AnyApiUser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyApiUser::Guest => write!(f, "Guest"),
            AnyApiUser::Identified(user) => write!(f, "{}={}", user.claims.email, user.claims.sub),
        }
    }
}
