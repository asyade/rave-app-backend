use crate::prelude::*;
use async_graphql::{self, Context, InputObject, Object, Result, SimpleObject};
use async_graphql::{Enum, Json, Upload};
use rave_api_iam::access::ScopedResource;
use rave_api_iam::api_user::AnyApiUser;
use rave_core_asset::AssetManager;
use rave_core_database::tables::asset::AssetKind;

#[derive(Default)]
pub struct AssetMutation;

#[Object]
impl AssetMutation {
    #[instrument(skip(self, ctx), ret)]
    pub async fn upload_image_asset(&self, ctx: &Context<'_>, file: Upload) -> Result<String> {
        let entity_sid = ctx.data::<AnyApiUser>()?.require_entity_sid()?;
        let file = file.value(ctx)?.content;
        let asset_manager = ctx.data::<AssetManager>()?;
        let asset = asset_manager
            .create_asset_from_raw_file(
                entity_sid,
                Some(AssetKind::Image),
                tokio::fs::File::from_std(file),
            )
            .await?;

        Ok(asset.uid.0.to_string())
    }
}

impl ScopedResource for AssetMutation {
    fn scope(&self) -> &str {
        "asset"
    }
}
