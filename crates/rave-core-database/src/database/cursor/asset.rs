use sqlx::types::Json;

use super::CursorExecutor;
use crate::{
    prelude::*,
    tables::asset::{AssetKind, AssetRow, DataColumnJson},
};

pub trait AssetCursor {
    fn create_asset(
        &mut self,
        owner_sid: PgSid,
        kind: &AssetKind,
        data: &DataColumnJson,
    ) -> impl Future<Output = CoreDatabaseResult<AssetRow>>;
}

impl<T: AsMut<CursorExecutor>> AssetCursor for T {
    async fn create_asset(
        &mut self,
        owner_sid: PgSid,
        kind: &AssetKind,
        data: &DataColumnJson,
    ) -> CoreDatabaseResult<AssetRow> {
        let row = sqlx::query_as(
            r#"
                INSERT INTO public.asset (owner_sid, kind, data)
                VALUES ($1, $2, $3)
                RETURNING sid, uid, owner_sid, kind, data
            "#,
        )
        .bind(owner_sid)
        .bind(kind)
        .bind(Json(data))
        .fetch_one(self.as_mut())
        .await?;
        Ok(row)
    }
}
