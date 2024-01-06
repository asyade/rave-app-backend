use crate::prelude::*;
use async_graphql::{self, Context, InputObject, Object, Result, SimpleObject};
use crate::services::database::Database;



#[derive(InputObject)]
pub struct CreateContentInput {
    
}

#[derive(Default)]
pub struct PostMutation;

#[Object]
impl PostMutation {
    pub async fn create_content(
        &self,
        _ctx: &Context<'_>,
        _input: CreateContentInput,
    ) -> Result<String> {
        // let db = ctx.data::<Database>().unwrap();
        // let conn = db.get_connection();
        
        unimplemented!()
    }

}
