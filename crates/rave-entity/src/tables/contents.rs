use crate::prelude::*;
use sqlx::types::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ContentRowJson {
    Text(String),
}
