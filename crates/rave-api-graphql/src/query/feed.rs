use crate::prelude::*;
use rave_api_service_feed_provider::{FeedCategory, FeedChunk, FeedProvider};


#[derive(Default)]
pub struct FeedQuery;

#[Object]
impl FeedQuery {
    async fn get_current_user_feed(
        &self,
        ctx: &Context<'_>,
        category: FeedCategory,
        limit: usize,
    ) -> Result<FeedChunk> {
        // let api_user = ctx.data::<crate::AnyApiUser>()?;
        let feed_provider = ctx.data::<FeedProvider>()?;
        let chunk = feed_provider
            .get(None, category, limit, None)
            .await;
        tracing::info!(
            offset = chunk.offset,
            version = chunk.version,
            post_count = chunk.posts.len(),
            "got feed chunk",
        );
        Ok(chunk)
    }
}
