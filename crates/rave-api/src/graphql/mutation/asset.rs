use crate::prelude::*;
use async_graphql::{self, Context, InputObject, Object, Result, SimpleObject};
use async_graphql::{Json, Upload, Enum};

#[derive(Debug, InputObject)]
pub struct CreateAssetInput {
    kind: String,
}

#[derive(Default)]
pub struct AssetMutation;

#[Object]
impl AssetMutation {
    pub async fn upload_asset(
        &self,
        ctx: &Context<'_>,
        _input: CreateAssetInput,
        file: Upload
    ) -> Result<String> {
        let ctx = file.value(ctx)?.content;
        // let db = ctx.data::<Database>().unwrap();
        // let conn = db.get_connection();
        dbg!(_input);
        unimplemented!()
    }
}
