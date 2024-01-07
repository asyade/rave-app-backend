use crate::prelude::*;

use super::entity::EntitySid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentRow {
    pub sid: ContentSid,
    pub uid: ContentUid,
    pub entity_sid: EntitySid,
    pub content: ContentColumnJson,
    // created_at,
    // updated_at
}

crate::typed_sql_wrapper!(ContentSid, PgSid);
crate::typed_sql_wrapper!(ContentUid, PgSid);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ContentColumnJson {
    Text {
        text: String
    },
}
