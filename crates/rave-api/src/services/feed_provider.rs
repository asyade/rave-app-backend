use std::vec;

use crate::prelude::*;
use rave_entity::{prelude::{InputType, SimpleObject}, async_graphql::MergedObject};


#[derive(Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum)]
pub enum FeedCategory {
    Home,
    Gems,
    Events,
    Music,
    Stream,
}

pub struct FeedProvider {}

pub struct FeedUID {}

pub struct Feed {
    uid: Uuid,
    onwer_uid: Uuid,
    category: FeedCategory,
    version: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct FeedChunk {
    pub version: i32,
    pub offset: usize,
    pub posts: Vec<FeedPost>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct FeedPost {
    pub uid: String,
    pub entity_sid: i32,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct FeedOffset {
    pub version: i32,
    pub offset: usize,
}

impl FeedProvider {
    pub async fn new() -> Self {
        Self {}
    }
    
    pub async fn get(
        &self,
        feed_uid: Option<Uuid>,
        owner_uid: Uuid,
        category: FeedCategory,
        limit: usize,
        offset: Option<FeedOffset>,
    ) -> RaveApiResult<FeedChunk> {
        Ok(FeedChunk {
            version: 1,
            offset: 0,
            posts: vec![
            ],
        })
    }
}
