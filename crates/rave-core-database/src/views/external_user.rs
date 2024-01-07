use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct ExternalUserRow {
    pub external_user_id: String,
    pub entity_sid: i32,
    pub name: String,
    pub email: String,
}
