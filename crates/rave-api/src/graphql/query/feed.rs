use crate::{prelude::*, services::feed_provider::{FeedProvider, FeedCategory, FeedOffset, FeedChunk}};

use async_graphql::{Context, Object, Result};
use rave_entity::{
    async_graphql,
    feed::Feed,
};

#[derive(Default)]
pub struct FeedQuery;

#[Object]
impl FeedQuery {
    async fn get_current_user_feed(&self, ctx: &Context<'_>, category: FeedCategory, limit: usize) -> Result<FeedChunk> {
        dbg!(ctx.field());
        let feed_provider = ctx.data::<FeedProvider>()?;
        let chunk = feed_provider.get(None, unimplemented!(), category, limit, None).await?;
        Ok(chunk)
    }
}
