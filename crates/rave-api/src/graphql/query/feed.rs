use crate::{
    prelude::*,
    services::feed_provider::{FeedCategory, FeedChunk, FeedOffset, FeedProvider},
};

use async_graphql::{Context, Object, Result};
use rave_entity::async_graphql;

#[derive(Default)]
pub struct FeedQuery;

#[Object]
impl FeedQuery {
    #[instrument(skip(self, ctx))]
    async fn get_current_user_feed(
        &self,
        ctx: &Context<'_>,
        category: FeedCategory,
        limit: usize,
    ) -> Result<FeedChunk> {
        let api_user = ctx.data::<crate::AnyApiUser>()?;
        let feed_provider = ctx.data::<FeedProvider>()?;
        let chunk = feed_provider
            .get(None, api_user, category, limit, None)
            .await?;
        tracing::info!(
            offset = chunk.offset,
            version = chunk.version,
            post_count = chunk.posts.len(),
            "got feed chunk",
        );
        Ok(chunk)
    }
}
