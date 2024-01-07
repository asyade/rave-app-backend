use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, FromRow)]
pub struct EntityRow {
    pub sid: EntitySid,
    pub uid: EntityUid,
}

crate::typed_sql_wrapper!(EntitySid, PgSid);
crate::typed_sql_wrapper!(EntityUid, PgUuid);
