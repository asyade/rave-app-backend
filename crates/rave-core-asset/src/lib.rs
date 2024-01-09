use crate::{analyser::AsAssetData, options::AssetOptions, prelude::*};
use data_provider::DataProviderManager;
use rave_core_database::{
    database::AssetCursor,
    tables::{
        asset::{AssetKind, AssetRow, DataColumnJson},
        entity::EntitySid,
    },
};
use tokio::io::AsyncReadExt;

pub mod error;
pub mod options;
pub mod prelude;

mod analyser;
mod data_provider;

#[allow(dead_code)]
pub struct AssetManager {
    options: AssetOptions,
    database: Database,
    providers: DataProviderManager,
}

impl AssetManager {
    #[allow(unused_mut)]
    pub async fn init(database: Database, options: AssetOptions) -> Self {
        let mut providers = DataProviderManager::new();

        #[cfg(feature = "aws-s3-data-provider")]
        {
            let provider = crate::data_provider::aws_s3::AwsS3DataProvider::init_from_environ(
                &options.aws_s3_bucket_name,
                &options.aws_s3_bucket_region,
            )
            .await;
            providers.register_provider(provider);
        }

        Self {
            options,
            database,
            providers,
        }
    }

    /// Create a new asset from a raw file.
    ///
    /// # Parameters
    /// * `owner_sid` - The entity that owns the asset.
    /// * `guest_kind` - The kind of asset that is expected. If `None` is provided, the asset kind will be determined from the file.
    /// * `file` - The file to ingest.
    ///
    /// # Returns
    /// The created asset.
    #[instrument(skip(self), err)]
    pub async fn create_asset(
        &self,
        owner_sid: EntitySid,
        guest_kind: AssetKind,
    ) -> AssetResult<AssetRow> {
        let kind = file.get_asset_kind().await?;

        if let Some(guest_kind) = guest_kind {
            if kind != guest_kind {
                return Err(AssetError::KindMismatch {
                    expected: guest_kind,
                    actual: kind,
                });
            }
        }

        let data = self.store_asset_data(file).await?;
        let asset = self
            .database
            .acquire()
            .await?
            .create_asset(owner_sid, &kind, &data)
            .await?;
        Ok(asset)
    }

    /// Ingests asset data from a file to the asset storage.
    ///
    /// # TODO
    /// * [ ] Implement different backend based like object storage, local file system, etc.
    async fn store_asset_data(&self, mut file: File) -> AssetResult<DataColumnJson> {
        let mut buffer = Vec::new();
        let _ = file.read_to_end(&mut buffer).await?;
        // Ok(DataColumnJson::Blob { buffer })
        todo!()
    }
}
