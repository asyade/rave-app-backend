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
    pub provider_uid: DataProviderUid,
}

crate::typed_sql_wrapper!(AssetSid, PgSid);
crate::typed_sql_wrapper!(AssetUid, PgUuid);

/// Type used to distinguish between different data providers.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Type, Serialize, Deserialize)]
#[sqlx(transparent)]
pub struct DataProviderUid(pub i32);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Type, Hash)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "AssetKind", rename_all = "lowercase")]
pub enum AssetKind {
    Image,
    Video,
    Audio,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataColumnJson {
    key: String,
    bucket: String,
    provider: DataProviderUid,
}
