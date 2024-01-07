use crate::prelude::*;
use async_graphql::{self, Context, InputObject, Object, Result, SimpleObject};
use rave_entity::{tables::content::ContentColumnJson, async_graphql::Json};

#[derive(InputObject)]
pub struct CreatePublicationInput {
    content_uid: String,
}

#[derive(InputObject)]
pub struct CreatePublicationCommentInput {
    publication_uid: String,
    content_uid: String,
}

#[derive(Default)]
pub struct PublicationMutation;

#[Object]
impl PublicationMutation {
    pub async fn create_publication(
        &self,
        _ctx: &Context<'_>,
        _input: CreatePublicationInput,
    ) -> Result<String> {
        unimplemented!()
    }
    
    pub async fn create_publication_comment(
        &self,
        _ctx: &Context<'_>,
        _input: CreatePublicationCommentInput,
    ) -> Result<String> {
        // let db = ctx.data::<Database>().unwrap();
        // let conn = db.get_connection();
        
        unimplemented!()
    }
}
