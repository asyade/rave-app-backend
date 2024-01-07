use sqlx::Postgres;

pub use asset::*;
pub use external_user::*;

mod asset;
mod external_user;

type CursorExecutor = <Postgres as sqlx::Database>::Connection;
