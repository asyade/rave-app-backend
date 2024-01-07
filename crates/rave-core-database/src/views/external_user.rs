use crate::{prelude::*, tables::entity::EntitySid};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct ExternalUserRow {
    pub external_user_id: ExternalUserId,
    pub entity_sid: EntitySid,
    pub name: String,
    pub email: String,
}

crate::typed_sql_wrapper!(ExternalUserId, String);
