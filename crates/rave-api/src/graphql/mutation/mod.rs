use crate::prelude::*;

pub mod user;
pub mod content;

pub use user::UserMutation;
pub use content::ContentMutation;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(UserMutation, ContentMutation);