use super::CursorExecutor;
use crate::{
    prelude::*,
    views::external_user::{ExternalUserId, ExternalUserRow},
};

pub trait ExternalUserCursor {
    fn find_external_user_by_external_user_id(
        &mut self,
        external_user_id: &ExternalUserId,
    ) -> impl Future<Output = CoreDatabaseResult<Option<ExternalUserRow>>>;

    fn create_external_user(
        &mut self,
        external_user_id: &ExternalUserId,
        email: &str,
        name: &str,
    ) -> impl Future<Output = CoreDatabaseResult<ExternalUserRow>>;
}

impl<T: AsMut<CursorExecutor>> ExternalUserCursor for T {
    async fn find_external_user_by_external_user_id(
        &mut self,
        external_user_id: &ExternalUserId,
    ) -> CoreDatabaseResult<Option<ExternalUserRow>> {
        sqlx::query_as(
            r#"
                SELECT external_user_id, entity_sid, email, name
                FROM public_user
                WHERE external_user_id = $1
            "#,
        )
        .bind(external_user_id.as_ref())
        .fetch_one(self.as_mut())
        .await
        .not_found_to_none()
    }

    /// Creates a new external user by both creating a new entity and a new external identity.
    /// This is a convenience method for the common case where a user authenticated from external IDP for the first time.
    ///
    /// # Arguments
    /// * `external_user_id` - The external user ID of the user (auth0 user id).
    /// * `email` - The email of the user.
    /// * `name` - The public name of the user.
    ///
    /// # Returns
    /// * `Ok(ExternalUserRow)` - If the user was created successfully.
    /// * `Err(CoreDatabaseError::UniqueViolation)` - If the user already exists.
    async fn create_external_user(
        &mut self,
        external_user_id: &ExternalUserId,
        email: &str,
        name: &str,
    ) -> CoreDatabaseResult<ExternalUserRow> {
        sqlx::query(
            r#"
            with created_entity as (
                insert into public.entity (sid, uid)
                    values (default, default) returning sid
            )
            insert into public.external_identity
                (sid, external_user_id, email, name, entity_sid)
                values (default, $1, $2, $3, (select sid from created_entity))
                
        "#,
        )
        .bind(external_user_id)
        .bind(email)
        .bind(name)
        .execute(self.as_mut())
        .await?;
        self.find_external_user_by_external_user_id(external_user_id)
            .await?
            .ok_or(CoreDatabaseError::CreatedEntityNotFound)
    }
}
