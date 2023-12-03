use crate::prelude::*;

use async_graphql::{Context, Object, Result};
use rave_entity::{
    async_graphql,
    iam::user::{User},
};


#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_current_user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        dbg!(ctx.field());
        let db = ctx.data::<Database>()?;
        let user = sqlx::query_as!(
            User,
            r#"SELECT sid, name, email FROM public_users WHERE sid = '1'"#
        )
        .fetch_one(&db.pool)
        .await?;
        Ok(Some(user))
    }

}
