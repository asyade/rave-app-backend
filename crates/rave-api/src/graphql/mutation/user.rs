use crate::prelude::*;
use async_graphql::{self, Context, InputObject, Object, Result, SimpleObject};

use crate::services::database::Database;

// I normally separate the input types into separate files/modules, but this is just
// a quick example.

#[derive(Default)]
pub struct UserMutation;

#[derive(InputObject)]
pub struct UpdateUserInput {
    pub uid: String,
}

#[Object]
impl UserMutation {
    pub async fn update_user(
        &self,
        _ctx: &Context<'_>,
        _input: UpdateUserInput,
    ) -> Result<String> {
        // let db = ctx.data::<Database>().unwrap();
        // let conn = db.get_connection();

        // Ok(Mutation::create_note(conn, input.into_model_with_arbitrary_id()).await?)
        unimplemented!()
    }

    // pub async fn delete_note(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
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
        // unimplemented!()
        // }
    // }
}