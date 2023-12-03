use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct Feed {
    sid: Option<i32>,
}
