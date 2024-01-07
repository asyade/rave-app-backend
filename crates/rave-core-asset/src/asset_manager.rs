use rave_core_database::{database::AssetCursor, tables::asset::DataColumnJson};

use crate::{analyser::AsAssetData, prelude::*};

pub struct AssetManager {
    database: Database,
}

impl AssetManager {
    pub fn init(database: Database) -> Self {
        Self { database }
    }

    #[instrument(skip(self), err)]
    pub async fn create_asset_from_raw_file(
        &self,
        owner_sid: PgSid,
        file: File,
    ) -> AssetResult<()> {
        let kind = file.get_asset_kind().await?;
        let data = self.store_asset_data(file).await?;
        let asset_row = self
            .database
            .acquire()
            .await?
            .create_asset(owner_sid, &kind, &data)
            .await?;
        Ok(())
    }

    /// Ingests asset data from a file to the asset storage.
    ///
    /// # TODO
    /// - [ ] Implement different backend based like object storage, local file system, etc.
    async fn store_asset_data(&self, file: File) -> AssetResult<DataColumnJson> {
        todo!()
    }
}
