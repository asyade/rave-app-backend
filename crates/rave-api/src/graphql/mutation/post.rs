use crate::prelude::*;
use async_graphql::{self, Context, InputObject, Object, Result, SimpleObject};
use rave_entity::post::{self, Post};
use crate::services::database::Database;

#[derive(InputObject)]
pub struct CreatePostInput {
    pub name: String,
    pub email: String,
}

impl CreatePostInput {
    fn into_model_with_arbitrary_id(self) -> Post {
        todo!()
    }
}

#[derive(SimpleObject)]
pub struct DeletePostResult {
    pub success: bool,
}

#[derive(Default)]
pub struct PostMutation;

#[Object]
impl PostMutation {
    pub async fn create_post(
        &self,
        _ctx: &Context<'_>,
        _input: CreatePostInput,
    ) -> Result<Post> {
        // let db = ctx.data::<Database>().unwrap();
        // let conn = db.get_connection();

        // Ok(Mutation::create_note(conn, input.into_model_with_arbitrary_id()).await?)
        todo!()
    }

    pub async fn delete_post(&self, ctx: &Context<'_>, id: i32) -> Result<DeletePostResult> {
        // let db = ctx.data::<Database>().unwrap();
        // let conn = db.get_connection();

        // let res = Mutation::delete_note(conn, id)
        // .await
        // .expect("Cannot delete note");

        // if  res.rows_affected <= 1 {
        // Ok(DeleteResult {
        // success: true,
        // rows_affected: res.rows_affected,
        // })
        // } else {
        todo!()
        // }
    }
}
