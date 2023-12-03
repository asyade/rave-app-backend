use crate::prelude::*;

use async_graphql::{Context, Object, Result};
use rave_entity::{
    async_graphql,
    feed::Feed,
};

#[derive(Default)]
pub struct FeedQuery;

#[Object]
impl FeedQuery {
    async fn get_feed(&self, ctx: &Context<'_>, name: String) -> Result<Option<Feed>> {
        dbg!(ctx.field());
        let db = ctx.data::<Database>()?;
        // let user = sqlx::query_as!(
        //     Feed,
        //     r#"SELECT sid, name, email FROM public_users WHERE sid = '1'"#
        // )
        // .fetch_one(&db.pool)
        // .await?;
        // Ok(Some(user))
        unimplemented!()
    }

}
