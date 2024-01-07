use crate::prelude::*;

use sqlx::types::Json;
use super::entity::EntitySid;


#[derive(Clone, Debug, PartialEq, FromRow)]
pub struct AssetRow {
    pub sid: AssetSid,
    pub uid: AssetUid,
    pub owner_sid: EntitySid,
    pub kind: AssetKind,
    pub data: Json<DataColumnJson>,
}

crate::typed_sql_wrapper!(AssetSid, PgSid);
crate::typed_sql_wrapper!(AssetUid, PgUuid);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
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
