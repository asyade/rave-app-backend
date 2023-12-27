use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub mod user {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
    pub struct PublicUser {
        pub external_user_id: String,
        pub entity_sid: i32,
        pub name: String,
        pub email: String,
    }
}
