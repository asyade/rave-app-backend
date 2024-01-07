use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentRow {
    pub sid: i32,
    pub uid: PgUuid,
    pub entity_sid: i32,
    pub content: ContentColumnJson,
    // created_at,
    // updated_at
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ContentColumnJson {
    Text {
        text: String
    },
}
