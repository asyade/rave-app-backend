use crate::prelude::*;

use async_graphql::{Context, Object, Result};
use rave_entity::{
    async_graphql,
    iam::user::{PublicUser},
};


#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_current_user(&self, ctx: &Context<'_>) -> Result<Option<PublicUser>> {
        dbg!(ctx.field());
        // let db = ctx.data::<Database>()?;
        // let user = sqlx::query_as!(
        //     PublicUser,
        //     r#"SELECT external_user_id, entity_sid, name, email FROM public_users WHERE entity_sid = '1'"#
        // )
        // .fetch_one(&db.pool)
        // .await?;
        unimplemented!()
        // Ok(Some(user))
    }

}
