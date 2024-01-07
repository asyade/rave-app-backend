use async_graphql::{EmptySubscription, Schema};
use rave_core_asset::asset_manager::AssetManager;

use crate::prelude::*;
use crate::graphql::{mutation::Mutation, query::Query};
use crate::prelude::RaveApiResult;
use crate::services::feed_provider::FeedProvider;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// Builds the GraphQL Schema, attaching the Database to the context
pub async fn build_schema(db: Database) -> RaveApiResult<AppSchema> {
    let feed = FeedProvider::new().await;
    let asset = AssetManager::init(db.clone());
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .data(feed)
        .data(asset)
        .finish();
    Ok(schema)
}
