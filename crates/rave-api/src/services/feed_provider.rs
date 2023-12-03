use std::vec;

use crate::prelude::*;
use rave_entity::post::{Post, PostContent};


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

pub struct FeedChunk {
    feed_uid: Uuid,
    version: i32,
    offset: usize,
    posts: Vec<Post>,
}

pub struct FeedOffset {
    pub version: i32,
    pub offset: usize,
}

impl FeedProvider {
    pub async fn new() -> Self {
        Self {}
    }
    
    pub async fn feed(
        feed_uid: Option<Uuid>,
        owner_uid: Uuid,
        category: FeedCategory,
        limit: usize,
        offset: Option<FeedOffset>,
    ) -> RaveApiResult<FeedChunk> {
        // Ok(FeedChunk {
            // feed_uid: Uuid,
            // version: 1,
            // offset: offset.unwrap_or(0),
        // })
        unimplemented!()
    }
}
