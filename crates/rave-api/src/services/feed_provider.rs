use crate::prelude::*;
use async_graphql::{Enum, MergedObject, SimpleObject};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
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
    pub title: String,
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
        let _ = feed_uid;
        let _ = owner_uid;
        let _ = category;
        let _ = limit;
        let _ = offset;
        Ok(FeedChunk {
            version: 1,
            offset: 0,
            posts: vec![
                FeedPost {
                    uid: "1".to_string(),
                    entity_sid: 1,
                    title: "Hello, world!".to_string(),
                    content: "Hello, world!".to_string(),
                },
                FeedPost {
                    uid: "2".to_string(),
                    entity_sid: 2,
                    title: "Hello, world!".to_string(),
                    content: "Hello, world!".to_string(),
                },
                FeedPost {
                    uid: "3".to_string(),
                    entity_sid: 3,
                    title: "Hello, world!".to_string(),
                    content: "Hello, world!".to_string(),
                },
                FeedPost {
                    uid: "4".to_string(),
                    entity_sid: 4,
                    title: "Hello, world!".to_string(),
                    content: "Hello, world!".to_string(),
                },
                FeedPost {
                    uid: "5".to_string(),
                    entity_sid: 5,
                    title: "Hello, world!".to_string(),
                    content: "Hello, world!".to_string(),
                },
            ],
        })
    }
}
