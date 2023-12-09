use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub mod user {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
    pub struct User {
        pub entity_sid: Option<i32>,
        pub name: Option<String>,
        pub email: Option<String>,
    }
}
