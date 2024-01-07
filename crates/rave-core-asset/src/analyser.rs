use crate::prelude::*;
use rave_core_database::tables::asset::AssetKind;
use tokio::fs::File;

pub trait AsAssetData {
    fn get_asset_kind(&self) -> impl Future<Output = AssetResult<AssetKind>>;
}

impl AsAssetData for File {
    async fn get_asset_kind(&self) -> AssetResult<AssetKind> {
        todo!()
    }
}
