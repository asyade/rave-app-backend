use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct PostContent {
    title: Option<String>,
    content: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct Post {
    pub sid: Option<i32>,
    pub owner_sid: Option<i32>,
    pub content: Option<Json<PostContent>>,
}
