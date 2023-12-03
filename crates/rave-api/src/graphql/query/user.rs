use crate::prelude::*;

use async_graphql::{Context, Object, Result};
use rave_entity::{
    async_graphql,
    iam::user::{self},
};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_current_user(&self, ctx: &Context<'_>) -> Result<Option<user::Model>> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.acquire().await?;

        // let user = sqlx::query_as!(user::Model, r"SELECT sid, name, email FROM public_users WHERE sid = '1'")
        //     .fetch_one(&db.pool)
        //     .await?;

        Ok(unimplemented!())
    }

    async fn get_user_by_uid(&self, ctx: &Context<'_>, uid: String) -> Result<Option<user::Model>> {
        let db = ctx.data::<Database>().unwrap();
        // let conn = db.get_connection();

        Ok(unimplemented!())
    }
}
