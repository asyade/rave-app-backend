use crate::prelude::*;

#[derive(InputObject)]
pub struct CreatePostInput {
    pub title: String,
    pub content: String,
}

#[derive(Default)]
pub struct PostMutation;

#[Object]
impl PostMutation {
    pub async fn create_post(
        &self,
        _ctx: &Context<'_>,
        _input: CreatePostInput,
    ) -> Result<String> {
        // let db = ctx.data::<Database>().unwrap();
        // let conn = db.get_connection();
        
        unimplemented!()
    }

}
