use crate::prelude::*;

pub mod user;
pub mod post;

pub use user::UserMutation;
pub use post::PostMutation;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(UserMutation, PostMutation);