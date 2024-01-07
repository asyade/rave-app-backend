use crate::prelude::*;

pub struct AssetManager {
    database: Database,
}

impl AssetManager {
    pub fn init(database: Database) -> Self {
        Self { database }
    }

    #[instrument(skip(self), err)]
    pub async fn create_asset_from_raw_file(&self, owner_sid: PgSid, file: File) -> AssetResult<()> {

        Ok(())
    }
}