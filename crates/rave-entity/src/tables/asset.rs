use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetRow {
    pub sid: i32,
    pub uid: PgUuid,
    pub kind: AssetKind,
    pub data: DataColumnJson,
}

#[derive(Debug, Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "AssetKind", rename_all = "lowercase")]
pub enum AssetKind {
    Image,
    Video,
    Audio,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DataColumnJson {
    Blob { buffer: Vec<u8> },
    // S3 {
    // bucket: String,
    // key: String,
    // region: String,
    // }
}
