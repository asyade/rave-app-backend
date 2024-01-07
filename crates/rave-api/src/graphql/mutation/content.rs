use crate::prelude::*;
use async_graphql::{self, Context, InputObject, Object, Result, SimpleObject, Json};
use rave_core_database::tables::content::ContentColumnJson;

#[derive(Debug, InputObject)]
pub struct CreateContentInput {
    content: Json<ContentColumnJson>,
}

#[derive(Default)]
pub struct ContentMutation;

#[Object]
impl ContentMutation {
    pub async fn create_content(
        &self,
        _ctx: &Context<'_>,
        _input: CreateContentInput,
    ) -> Result<String> {
        // let db = ctx.data::<Database>().unwrap();
        // let conn = db.get_connection();
        dbg!(_input);
        unimplemented!()
    }
}
