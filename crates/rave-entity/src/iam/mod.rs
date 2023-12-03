use crate::prelude::*;

pub mod user {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SimpleObject)]
    #[graphql(concrete(name = "User", params()))]
    pub struct Model {
        pub sid: i32,
        pub name: String,
        pub email: String,
        // pub roles: Vec<UserRole>,
    }

    // #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SimpleObject)]
    // #[graphql(concrete(name = "UserRole", params()))]
    // pub struct UserRole {
        // pub role_sid: i32,
        // pub role_name: String,
    // }
}
