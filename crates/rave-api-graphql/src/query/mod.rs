use rave_entity::async_graphql;
pub mod feed;

use self::feed::FeedQuery;
// You can add other queries here to create a unified Query object
// e.x. Query(FeedQuery, OtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(FeedQuery);