use crate::prelude::*;

pub mod user;
pub mod asset;
pub mod content;
pub mod publication;

pub use user::UserMutation;
pub use asset::AssetMutation;
pub use content::ContentMutation;
pub use publication::PublicationMutation;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(UserMutation, ContentMutation, PublicationMutation);