use aws_config::{meta::credentials::CredentialsProviderChain, SdkConfig};
use rave_core_database::{
    database::AssetCursor,
    tables::{
        asset::{AssetKind, AssetRow, DataColumnJson},
        entity::EntitySid,
    },
};
use tokio::io::AsyncReadExt;

use crate::{analyser::AsAssetData, options::AssetOptions, prelude::*};

pub struct AssetManager {
    database: Database,
}

impl AssetManager {
    pub fn init(database: Database, options: AssetOptions) -> Self {
        let client = aws_sdk_s3::Client::new(
            &aws_config::from_env()
                .credentials_provider(
                    CredentialsProviderChain::first_try(
                        "Environment",
                        aws_config::environment::credentials::EnvironmentVariableCredentialsProvider::new(),
                    )
                    .or_else(
                        "Profile",
                        aws_config::profile::ProfileFileCredentialsProvider::builder().build(),
                    ),
                )
                .load()
                .await
                .expect("Failed to load AWS config"),
        );
        Self { database }
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
    pub async fn create_asset_from_raw_file(
        &self,
        owner_sid: EntitySid,
        guest_kind: Option<AssetKind>,
        file: File,
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
        Ok(DataColumnJson::Blob { buffer })
    }
}
