use rave_entity::async_graphql;

pub mod user;
pub mod feed;

pub use user::UserQuery;

use self::feed::FeedQuery;

// Add your other ones here to create a unified Query object
// e.x. Query(NoteQuery, OtherQuery, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(UserQuery, FeedQuery);