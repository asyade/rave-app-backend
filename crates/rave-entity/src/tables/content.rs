use crate::prelude::*;
use sqlx::types::Uuid;

pub struct ContentRow {
    sid: i32,
    uid: Uuid,
    entity_sid: i32,
    content: ContentFieldJson,
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
