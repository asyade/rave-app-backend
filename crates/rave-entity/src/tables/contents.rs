use crate::prelude::*;
use sqlx::types::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ContentRowJson {
    Text(String),
}

pub struct ContentRow {
    sid: i32,
    uid: Uuid,
    entity_sid: i32,
    // created_at,
    // updated_at
}
