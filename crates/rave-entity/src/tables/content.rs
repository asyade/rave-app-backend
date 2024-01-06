use crate::prelude::*;
use sqlx::types::Uuid;

pub struct ContentRow {
    pub sid: i32,
    pub uid: Uuid,
    pub entity_sid: i32,
    pub content: ContentFieldJson,
    // created_at,
    // updated_at
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ContentFieldJson {
    Text {
        text: String
    },
}
